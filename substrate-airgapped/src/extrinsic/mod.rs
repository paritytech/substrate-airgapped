use crate::{
	frame::{balances::Balances, system::System},
	runtimes::Runtime,
};
use codec::{Decode, Encode, Input};
use core::fmt::Debug;
use extra::SignedExtra;
use sp_core::Pair;

pub(crate) mod extra;

/// Method with the method and module index
#[derive(Clone, Copy, Debug, PartialEq, Encode, Decode)]
pub struct CallIndex {
	module_index: u8,
	call_index: u8,
}

impl CallIndex {
	/// Get `module`, the index of the module the call is in
	// pub fn module_index(&self) -> u8 {
	// 	self.module_index
	// }

	/// Get `call_index`, the index of the call in the module
	// pub fn call_index(&self) -> u8 {
	// 	self.call_index
	// }

	/// Create `CallIndex`
	pub fn new(module_index: u8, call_index: u8) -> CallIndex {
		CallIndex { module_index, call_index }
	}

	/// Create a slice representing the call index
	pub fn to_vec(self) -> Vec<u8> {
		vec![self.module_index, self.call_index]
	}
}

/// Call represented by call index and argument struct.
/// This has the ability to correctly encode and decode itself without metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct GenericCall<C: Encode + Decode + Clone> {
	/// `CallIndex`
	pub call_index: CallIndex,
	/// Arguments
	pub args: C,
}

impl<C: Encode + Decode + Clone> GenericCall<C> {
	fn args_encoded(&self) -> Vec<u8> {
		self.args.encode()
	}

	fn call_index_encoded(&self) -> Vec<u8> {
		self.call_index.to_vec()
	}
}

impl<C: Encode + Decode + Clone> Encode for GenericCall<C> {
	fn encode(&self) -> Vec<u8> {
		let mut bytes = self.call_index.to_vec();
		bytes.extend(self.args.encode());

		bytes
	}
}

impl<C: Encode + Decode + Clone> Decode for GenericCall<C> {
	fn decode<I: Input>(value: &mut I) -> Result<Self, codec::Error> {
		let value_len = value.remaining_len()?.ok_or("Codec Error: No length")?;
		let mut buf: Vec<u8> = Vec::with_capacity(value_len);
		value.read(&mut buf[..])?;

		Ok(GenericCall {
			call_index: CallIndex::new(buf[0], buf[1]),
			args: C::decode(&mut &buf[2..]).unwrap(),
		})
	}
}

/// Local `UncheckedExtrinsic` convenience type
pub type UncheckedExtrinsic<C, R> = sp_runtime::generic::UncheckedExtrinsic<
	<R as System>::Address,
	GenericCall<C>,
	<R as Runtime>::Signature,
	extra::Extra<R>,
>;

/// Local `SignedPayload` convenience type
pub type SignedPayload<C, R> = sp_runtime::generic::SignedPayload<GenericCall<C>, extra::Extra<R>>;

/// Transaction builder all the components to create a signing payload.
pub struct TxBuilder<C: Encode + Decode + Clone, R: System + Balances + Runtime> {
	/// Call with all info for encoding and decoding
	call: GenericCall<C>,
	/// Signers Address
	address: R::Address,
	/// Signers nonce
	nonce: R::Index,
	/// Transaction version associated with the runtime
	tx_version: u32,
	/// API specification version of the runtime
	spec_version: u32,
	/// Hash of the networks genesis block
	genesis_hash: R::Hash,
	// TODO tip, era_period, checkpoint_block_hash, checkpoint_block_number
}

impl<C: Encode + Decode + Clone, R: System + Balances + Runtime> TxBuilder<C, R> {
	/// Create transaction builder, a struct with all the components to create a signing payload.
	pub fn new(
		call: GenericCall<C>,
		address: R::Address,
		nonce: R::Index,
		tx_version: u32,
		spec_version: u32,
		genesis_hash: R::Hash,
	) -> Self {
		TxBuilder { call, address, nonce, tx_version, spec_version, genesis_hash }
	}

	fn extra(&self) -> <R as Runtime>::Extra {
		R::Extra::new(self.spec_version, self.tx_version, self.nonce, self.genesis_hash)
	}

	/// Returns `SignedPayload`, the payload to sign.
	pub fn signed_payload(&self) -> SignedPayload<C, R> {
		let extra = self.extra();
		SignedPayload::<C, R>::new(self.call.clone(), extra.extra())
			.expect("TODO signed payload constructs")
	}

	/// Create an unchecked extrinsic signed with the given pair
	pub fn unchecked_from_pair<P>(&self, pair: P) -> Result<UncheckedExtrinsic<C, R>, String>
	where
		P: Pair,
		<R as Runtime>::Signature: From<<P as sp_core::Pair>::Signature>,
	{
		let payload = self.signed_payload();
		let signature = payload.using_encoded(|payload| pair.sign(payload));
		let (call, extra, _) = payload.deconstruct();
		let unchecked = UncheckedExtrinsic::<C, R>::new_signed(
			call,
			self.address.clone(),
			signature.into(),
			extra,
		);

		Ok(unchecked)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{balances::Transfer, PolkadotRuntime};
	use sp_keyring::AccountKeyring;

	#[test]
	fn generic_call_encode() {
		let bob_addr = AccountKeyring::Bob.to_account_id().into();
		let transfer_args: Transfer<PolkadotRuntime> = Transfer { to: bob_addr, amount: 12 };
		let transfer = GenericCall {
			call_index: CallIndex { module_index: 5, call_index: 0 },
			args: transfer_args,
		};

		assert_eq!(transfer.call_index_encoded(), [5, 0]);
		assert_eq!(
			transfer.args_encoded(),
			[
				255, 142, 175,   4,  21,  22, 135, 115,
				99,  38, 201, 254, 161, 126,  37, 252,
				82, 135,  97,  54, 147, 201,  18, 144,
				156, 178,  38, 170,  71, 148, 242, 106,
				72,  48
			]
		);

		let call_encoded_expected = [
			5, 0, 142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97,
			54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72, 48,
		];
		assert_eq!(transfer.encode(), call_encoded_expected);
		// let decoded_call = Transfer::<PolkadotRuntime>::decode(&mut &call_encoded_expected);
		// assert_eq!(decoded_call, transfer);
	}

	#[test]
	fn signed_payload_encodes() {
		let bob_addr = AccountKeyring::Bob.to_account_id().into();
		let alice_addr: <PolkadotRuntime as System>::Address = AccountKeyring::Alice.to_account_id().into();
		println!("alice {}", alice_addr);
		let transfer_args: Transfer<PolkadotRuntime> = Transfer { to: bob_addr, amount: 12 };
		let transfer = GenericCall {
			call_index: CallIndex { module_index: 5, call_index: 0 },
			args: transfer_args,
		};
		let genesis_hash = [
			221, 185, 147, 77, 30, 241, 157, 155, 28, 177, 225, 8, 87, 182, 228, 162, 79, 230, 196,
			149, 215, 168, 99, 34, 136, 35, 92, 20, 18, 83, 139, 132,
		];
		let genesis_hash = sp_core::H256::from_slice(&genesis_hash[..]);
		let tx_builder: TxBuilder<Transfer<PolkadotRuntime>, PolkadotRuntime> =
			TxBuilder::new(transfer, alice_addr, 0, 4, 26, genesis_hash);

		let signed_payload_encoded_expected = [
			/// CallIndex + Args
			5, 0, 255, 142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135,
			97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72, 48, 101, 1, 0, 0,
			26, 0, 0, 0, 5, 0, 0, 0, 221, 185, 147, 77, 30, 241, 157, 155, 28, 177, 225, 8, 87,
			182, 228, 162, 79, 230, 196, 149, 215, 168, 99, 34, 136, 35, 92, 20, 18, 83, 139, 132,
			225, 64, 130, 68, 166, 222, 158, 234, 239, 194, 156, 28, 250, 111, 112, 177, 208, 79,
			137, 252, 126, 151, 25, 124, 104, 147, 234, 164, 185, 169, 226, 92,
		];

		assert_eq!(signed_payload_encoded_expected.to_vec(), tx_builder.signed_payload().encode())
	}
}
