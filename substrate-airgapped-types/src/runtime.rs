use crate::extrinsic::extra::{DefaultExtra, SignedExtra};
use crate::frame::{balances::Balances, system::System};
use codec::{Decode, Encode};
use core::fmt::Debug;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	MultiSignature,
};

/// Trait to encompassing types from a runtime and its pallets.
pub trait Runtime: System + Sized + Send + Sync + 'static {
	/// Signature type.
	type Signature: Verify + Encode + Debug + Decode + Eq + Send + Sync + Clone + 'static;
	/// Transaction extras.
	type Extra: SignedExtra<Self> + Send + Decode + Sync + 'static;
}

/// Polkadot runtime specific types (should work with Kusama as well)
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PolkadotRuntime;

impl Runtime for PolkadotRuntime {
	type Signature = MultiSignature;
	type Extra = DefaultExtra<Self>;
}

impl System for PolkadotRuntime {
	type Index = u32;
	type BlockNumber = u32;
	type Hash = sp_core::H256;
	type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
	type Address = Self::AccountId;
}

impl Balances for PolkadotRuntime {
	type Balance = u128;
}
