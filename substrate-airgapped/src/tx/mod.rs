pub(crate) mod extra;
mod generic_call;
mod mortality;

pub use self::{
	generic_call::{CallIndex, GenericCall},
	mortality::{MortalConfig, Mortality},
};

use self::extra::{Extra, SignedExtra};
use crate::{
	frame::{balances::Balances, system::System},
	runtimes::Runtime,
	Error,
};
use codec::{Decode, Encode};
use sp_core::Pair;
use sp_runtime::generic::Era;

/// Local `UncheckedExtrinsic` convenience type. This is a transaction.
pub type UncheckedExtrinsic<C, R> = sp_runtime::generic::UncheckedExtrinsic<
	<R as System>::Address,
	GenericCall<C>,
	<R as Runtime>::Signature,
	Extra<R>,
>;

/// Local `SignedPayload` convenience type. This is the payload that gets signed.
pub type SignedPayload<C, R> = sp_runtime::generic::SignedPayload<GenericCall<C>, Extra<R>>;

/// Configuration options for a Tx
#[derive(Clone, PartialEq, Debug)]
pub struct TxConfig<C: Encode + Decode + Clone, R: System + Balances + Runtime> {
	/// Call with all info for encoding and decoding.
	pub call: GenericCall<C>,
	/// Signers Address.
	pub address: R::Address,
	/// Signers nonce.
	pub nonce: R::Index,
	/// Transaction version associated with the runtime.
	pub tx_version: u32,
	/// API specification version of the runtime.
	pub spec_version: u32,
	/// Hash of the networks genesis block.
	pub genesis_hash: R::Hash,
	/// The mortality of the transaction.
	pub mortality: Mortality<R>,
	/// Tip, used for transaction priority.
	pub tip: R::Balance,
}

/// Transaction builder with all the components to create a signing payload.
#[derive(Clone, PartialEq, Debug)]
pub struct Tx<C: Encode + Decode + Clone, R: System + Balances + Runtime> {
	/// Call with all info for encoding and decoding.
	call: GenericCall<C>,
	/// Signers Address.
	address: R::Address,
	/// Signers nonce.
	nonce: R::Index,
	/// Transaction version associated with the runtime.
	tx_version: u32,
	/// API specification version of the runtime.
	spec_version: u32,
	/// Hash of the networks genesis block.
	genesis_hash: R::Hash,
	/// The mortality of the transaction.
	mortality: Mortality<R>,
	/// Tip, used for transaction priority.
	tip: R::Balance,
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

	UncheckedExtrinsic::<C, R>::new_signed(call, sender, signature, extra)
}

impl<C: Encode + Decode + Clone, R: System + Balances + Runtime> Tx<C, R> {
	/// Create a transaction builder from TxConfig
	pub fn new(config: TxConfig<C, R>) -> Self {
		Tx {
			call: config.call,
			address: config.address,
			nonce: config.nonce,
			tx_version: config.tx_version,
			spec_version: config.spec_version,
			genesis_hash: config.genesis_hash,
			mortality: config.mortality,
			tip: config.tip,
		}
	}

	/// Transaction's call, including arguments and call index.
	pub fn call(&self) -> &GenericCall<C> {
		&self.call
	}

	/// Address of the transaction's signer.
	pub fn address(&self) -> &R::Address {
		&self.address
	}

	/// Nonce of the signer.
	pub fn nonce(&self) -> &R::Index {
		&self.nonce
	}

	/// Transaction version associated with the runtime.
	pub fn tx_version(&self) -> &u32 {
		&self.tx_version
	}

	/// Api specification version of the runtime.
	pub fn spec_version(&self) -> &u32 {
		&self.spec_version
	}

	/// Hash of the networks genesis block.
	pub fn genesis_hash(&self) -> &R::Hash {
		&self.genesis_hash
	}

	/// Mortality of the transaction, including mortal period, checkpoint block
	/// number, and checkpoint block hash.
	pub fn mortality(&self) -> &Mortality<R> {
		&self.mortality
	}

	/// Tip, used to determine transaction priority.
	pub fn tip(&self) -> &R::Balance {
		&self.tip
	}

	fn extra(&self) -> <R as Runtime>::Extra {
		let era_info = match &self.mortality {
			Mortality::Mortal(config) => (
				Era::mortal(config.period, config.checkpoint_block_number),
				Some(config.checkpoint_block_hash),
			),
			Mortality::Immortal => (Era::immortal(), None),
		};

		R::Extra::new(
			self.spec_version,
			self.tx_version,
			self.nonce,
			self.genesis_hash,
			era_info,
			self.tip,
		)
	}

	/// Create a `SignedPayload`, the payload to sign.
	pub fn signed_payload(&self) -> Result<SignedPayload<C, R>, Error> {
		let extra = self.extra();

		SignedPayload::<C, R>::new(self.call.clone(), extra.extra()).map_err(Into::into)
	}

	/// Create a signed `UncheckedExtrinsic` (AKA transaction) using the given keyring pair to sign.
	pub fn signed_tx_from_pair<P>(&self, pair: P) -> Result<UncheckedExtrinsic<C, R>, Error>
	where
		P: Pair,
		<R as Runtime>::Signature: From<<P as sp_core::Pair>::Signature>,
	{
		let payload = self.signed_payload()?;
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
		let transfer_call = GenericCall::new(CallIndex::new(5, 0), transfer_args);
		let genesis_hash = [
			221, 185, 147, 77, 30, 241, 157, 155, 28, 177, 225, 8, 87, 182, 228, 162, 79, 230, 196,
			149, 215, 168, 99, 34, 136, 35, 92, 20, 18, 83, 139, 132,
		];
		let genesis_hash = sp_core::H256::from_slice(&genesis_hash[..]);
		let tx_config = TxConfig {
			call: transfer_call,
			address: alice_addr,
			nonce: 0,
			tx_version: 4,
			spec_version: 26,
			genesis_hash,
			mortality: Mortality::Immortal,
			tip: 0,
		};
		let tx: Tx<TransferType, KusamaRuntime> = Tx::new(tx_config);

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
		let signed_payload = tx.signed_payload().expect("test case works");
		assert_eq!(signed_payload_encoded_expected.to_vec(), signed_payload.encode());
	}

	#[test]
	fn tx_correctly_constructs_encoded_transaction_from_keyring_pair() {
		let tx = test_tx_instance();

		let signed_tx =
			tx.signed_tx_from_pair(AccountKeyring::Alice.pair()).expect("test case works");
		let signed_tx_encoded = signed_tx.encode().to_vec();

		let version_and_address = [
			33u8, 2, 132, 212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214,
			130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125, 1,
		];
		assert_eq!(version_and_address, signed_tx_encoded[0..36]);

		// Sig is non-deterministic so we do not assert equivalence
		let _sig = [
			236, 253, 48, 98, 178, 30, 37, 245, 91, 58, 158, 88, 180, 224, 236, 97, 249, 154, 143,
			229, 160, 134, 158, 219, 102, 51, 37, 186, 255, 101, 61, 83, 200, 8, 163, 93, 146, 54,
			28, 210, 53, 81, 147, 241, 170, 51, 213, 219, 27, 16, 45, 221, 53, 114, 174, 112, 175,
			48, 10, 243, 52, 80, 143, 1, 32,
		];

		let extra = [0, 0, 0];
		assert_eq!(extra, signed_tx_encoded[100..103]);

		let call = [
			5, 0, 142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97,
			54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72, 48,
		];
		assert_eq!(call, signed_tx_encoded[103..]);
	}

	#[test]
	fn tx_attribute_getters_work() {
		let tx = test_tx_instance();

		let transfer_args: TransferType =
			Transfer { to: AccountKeyring::Bob.to_account_id().into(), amount: 12 };
		let transfer_call = GenericCall::new(CallIndex::new(5, 0), transfer_args);
		assert_eq!(tx.call(), &transfer_call);
		assert_eq!(tx.address(), &AccountKeyring::Alice.to_account_id());
	}
}
