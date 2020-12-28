use crate::frame::Parameter;
use codec::Codec;
use core::fmt::Debug;
use sp_runtime::traits::{
	AtLeast32Bit, AtLeast32BitUnsigned, Bounded, CheckEqual, MaybeDisplay, MaybeMallocSizeOf, MaybeSerialize,
	MaybeSerializeDeserialize, Member, SimpleBitOps,
};

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
