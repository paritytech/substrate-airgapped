use crate::frame::Parameter;
use codec::{Codec, Decode};
use core::{fmt::Debug};
use sp_runtime::traits::{AtLeast32Bit};

/// Subset of the `pallet_system::Trait` the Runtime must implement.
pub trait System {
	/// Account index (aka nonce) type. This stores the number of previous transactions associated
	/// with a sender account.
	type Index: Parameter + Send + Sync + 'static + Debug + Default + AtLeast32Bit + Copy;
	// type Index: Parameter + Member + MaybeSerialize + Debug + Default + MaybeDisplay + AtLeast32Bit + Copy;

	/// The block number type used by the runtime.
	type BlockNumber: Parameter
		+ Send
		+ Sync
		+ Debug
		+ Default
		+ Copy
		+ std::hash::Hash
		+ std::str::FromStr;
		// + Member
		// + MaybeMallocSizeOf
		// + MaybeSerializeDeserialize
		// + MaybeDisplay
		// + AtLeast32BitUnsigned
		// + Bounded;

		/// The output of the `Hashing` function.
	type Hash: Parameter
		+ Send
		+ Sync
		+ std::hash::Hash
		+ AsRef<[u8]>
		+ AsMut<[u8]>
		+ Default
		+ Copy
		+ Ord
		+ Debug;
		// + Member
		// + MaybeMallocSizeOf
		// + MaybeSerializeDeserialize
		// + MaybeDisplay
		// + SimpleBitOps
		// + CheckEqual;

	/// The user account identifier type for the runtime.
	type AccountId: Parameter + Ord + Default;
	// type AccountId: Parameter + Member + MaybeSerialize + MaybeDisplay + Ord + Default;

	/// The address type. This instead of `<frame_system::Trait::Lookup as StaticLookup>::Source`.
	type Address: Codec + Clone + PartialEq + Debug + Send + Sync + Eq + Decode;
}
