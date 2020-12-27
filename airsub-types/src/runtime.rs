use crate::extra::{DefaultExtra, SignedExtra};
use codec::{Codec, Encode, EncodeLike};
use core::fmt::Debug;
use sp_runtime::{
	traits::{
		AtLeast32Bit, AtLeast32BitUnsigned, Bounded, CheckEqual, IdentifyAccount, MaybeDisplay, MaybeMallocSizeOf,
		MaybeSerialize, MaybeSerializeDeserialize, Member, SimpleBitOps, Verify,
	},
	MultiSignature,
};

/// A type that can be used as a parameter in a dispatchable function. Defined here so we don't have
// to require frame-system (TODO Not yet sure if this is an ok approach)
pub trait Parameter: Codec + EncodeLike + Clone + Eq + Debug {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + Debug {}

/// The subset of the `pallet_balances::Trait` that a Runtime can implement.
pub trait Balances: System {
	/// The balance of an account.
	type Balance: Parameter
		+ Member
		+ AtLeast32Bit
		+ codec::Codec
		+ Default
		+ Copy
		+ MaybeSerialize
		+ Debug
		+ From<<Self as System>::BlockNumber>;
}

// Subset of the `pallet_system::Trait` the Runtime must implement.
pub trait System {
	/// Account index (aka nonce) type. This stores the number of previous transactions associated with a sender account.
	type Index: Parameter + Member + MaybeSerialize + Debug + Default + MaybeDisplay + AtLeast32Bit + Copy;
	/// The block number type used by the runtime.
	type BlockNumber: Parameter
		+ Member
		+ MaybeMallocSizeOf
		+ MaybeSerializeDeserialize
		+ Debug
		+ MaybeDisplay
		+ AtLeast32BitUnsigned
		+ Default
		+ Bounded
		+ Copy
		+ std::hash::Hash
		+ std::str::FromStr;
	/// The output of the `Hashing` function.
	type Hash: Parameter
		+ Member
		+ MaybeMallocSizeOf
		+ MaybeSerializeDeserialize
		+ Debug
		+ MaybeDisplay
		+ Ord
		+ SimpleBitOps
		+ Default
		+ Copy
		+ CheckEqual
		+ std::hash::Hash
		+ AsRef<[u8]>
		+ AsMut<[u8]>;
	/// The user account identifier type for the runtime.
	type AccountId: Parameter + Member + MaybeSerialize + MaybeDisplay + Ord + Default;
	/// The address type. This instead of `<frame_system::Trait::Lookup as StaticLookup>::Source`.
	type Address: Codec + Clone + PartialEq + Debug + Send + Sync;
}

/// Trait to encompassing types from a runtime and its pallets.
pub trait Runtime: System + Sized + Send + Sync + 'static {
	/// Signature type.
	type Signature: Verify + Encode + Send + Sync + 'static;
	/// Transaction extras.
	type Extra: SignedExtra<Self> + Send + Sync + 'static;
}

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
