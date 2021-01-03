use super::{
	extra::{Extra, SignedExtra},
	GenericCall,
};
use crate::{
	frame::{balances::Balances, system::System},
	runtimes::Runtime,
};
use codec::{Decode, Encode};
use sp_core::Pair;

/// Local `UncheckedExtrinsic` convenience type. This is a transaction.
pub type UncheckedExtrinsic<C, R> = sp_runtime::generic::UncheckedExtrinsic<
	<R as System>::Address,
	GenericCall<C>,
	<R as Runtime>::Signature,
	Extra<R>,
>;

/// Local `SignedPayload` convenience type. This is the payload that gets signed.
pub type SignedPayload<C, R> = sp_runtime::generic::SignedPayload<GenericCall<C>, Extra<R>>;

/// Transaction builder with all the components to create a signing payload.
pub struct Tx<C: Encode + Decode + Clone, R: System + Balances + Runtime> {
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

/// Create a tx from the senders address, a `SignedPayload` and the signature.
pub fn tx_from_parts<C, R>(
	sender: R::Address,
	signature: R::Signature,
	payload: SignedPayload<C, R>,
) -> UncheckedExtrinsic<C, R>
where
	C: Encode + Decode + Clone,
	R: System + Runtime,
{
	let (call, extra, _) = payload.deconstruct();
	let tx = UncheckedExtrinsic::<C, R>::new_signed(call, sender, signature, extra);

	tx
}

impl<C: Encode + Decode + Clone, R: System + Balances + Runtime> Tx<C, R> {
	/// Create transaction builder, a struct with all the components to create a signing payload.
	pub fn new(
		call: GenericCall<C>,
		address: R::Address,
		nonce: R::Index,
		tx_version: u32,
		spec_version: u32,
		genesis_hash: R::Hash,
	) -> Self {
		Tx { call, address, nonce, tx_version, spec_version, genesis_hash }
	}

	fn extra(&self) -> <R as Runtime>::Extra {
		R::Extra::new(self.spec_version, self.tx_version, self.nonce, self.genesis_hash)
	}

	/// Create a `SignedPayload`, the payload to sign.
	pub fn signed_payload(&self) -> SignedPayload<C, R> {
		let extra = self.extra();
		SignedPayload::<C, R>::new(self.call.clone(), extra.extra())
			.expect("TODO signed payload constructs")
	}

	/// Create a signed `UncheckedExtrinsic` (AKA transaction) using the given keyring pair to sign.
	pub fn tx_from_pair<P>(&self, pair: P) -> Result<UncheckedExtrinsic<C, R>, String>
	where
		P: Pair,
		<R as Runtime>::Signature: From<<P as sp_core::Pair>::Signature>,
	{
		let payload = self.signed_payload();
		let signature = payload.using_encoded(|payload| pair.sign(payload));
		let tx = tx_from_parts::<C, R>(self.address.clone(), signature.into(), payload);

		Ok(tx)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{balances::Transfer, CallIndex, KusamaRuntime};
	use sp_keyring::AccountKeyring;

	type TransferType = Transfer<KusamaRuntime>;

	/// Construct an instance of Tx for use in tests.
	fn test_tx_instance() -> Tx<TransferType, KusamaRuntime> {
		let bob_addr = AccountKeyring::Bob.to_account_id().into();
		let alice_addr = AccountKeyring::Alice.to_account_id().into();

		let transfer_args: TransferType = Transfer { to: bob_addr, amount: 12 };
		let transfer_call = GenericCall { call_index: CallIndex::new(5, 0), args: transfer_args };
		let genesis_hash = [
			221, 185, 147, 77, 30, 241, 157, 155, 28, 177, 225, 8, 87, 182, 228, 162, 79, 230, 196,
			149, 215, 168, 99, 34, 136, 35, 92, 20, 18, 83, 139, 132,
		];
		let genesis_hash = sp_core::H256::from_slice(&genesis_hash[..]);
		let tx: Tx<TransferType, KusamaRuntime> =
			Tx::new(transfer_call, alice_addr, 0, 4, 26, genesis_hash);

		tx
	}

	#[test]
	fn tx_correctly_constructs_encoded_signed_payload() {
		let tx = test_tx_instance();

		let signed_payload_encoded_expected = [
			5, 0, 142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97,
			54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72, 48, 0, 0, 0, 26, 0, 0,
			0, 4, 0, 0, 0, 221, 185, 147, 77, 30, 241, 157, 155, 28, 177, 225, 8, 87, 182, 228,
			162, 79, 230, 196, 149, 215, 168, 99, 34, 136, 35, 92, 20, 18, 83, 139, 132, 221, 185,
			147, 77, 30, 241, 157, 155, 28, 177, 225, 8, 87, 182, 228, 162, 79, 230, 196, 149, 215,
			168, 99, 34, 136, 35, 92, 20, 18, 83, 139, 132,
		];
		assert_eq!(signed_payload_encoded_expected.to_vec(), tx.signed_payload().encode())
	}

	#[test]
	fn tx_correctly_constructs_encoded_transaction_from_keyring_pair() {
		let tx = test_tx_instance();

		let signed_tx = tx.tx_from_pair(AccountKeyring::Alice.pair());
		let signed_tx_encoded = signed_tx.encode().to_vec();

		let version_and_address = [
			0u8, 33, 2, 132, 212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214,
			130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125, 1,
		];
		assert_eq!(version_and_address, signed_tx_encoded[0..37]);

		// Sig is variable so we do not assert equivalence
		let _sig = [
			236, 253, 48, 98, 178, 30, 37, 245, 91, 58, 158, 88, 180, 224, 236, 97, 249, 154, 143,
			229, 160, 134, 158, 219, 102, 51, 37, 186, 255, 101, 61, 83, 200, 8, 163, 93, 146, 54,
			28, 210, 53, 81, 147, 241, 170, 51, 213, 219, 27, 16, 45, 221, 53, 114, 174, 112, 175,
			48, 10, 243, 52, 80, 143, 1, 32,
		];

		let extra = [0, 0, 0];
		assert_eq!(extra, signed_tx_encoded[101..104]);

		let call = [
			5, 0, 142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97,
			54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72, 48,
		];
		assert_eq!(call, signed_tx_encoded[104..]);
	}
}
