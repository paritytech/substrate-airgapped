use super::Parameter;
use codec::{Codec, Decode};
use core::fmt::{Debug, Display};
use sp_core::crypto::Ss58Codec;
use sp_runtime::traits::AtLeast32Bit;

/// Subset of the `pallet_system::Trait` the Runtime must implement.
pub trait System {
	/// Account index (aka nonce) type. This stores the number of previous transactions associated
	/// with a sender account.
	type Index: Parameter + Send + Sync + 'static + Debug + Default + AtLeast32Bit + Copy + Display;

	/// The block number type used by the runtime.
	type BlockNumber: Parameter
		+ Send
		+ Sync
		+ Debug
		+ Default
		+ Copy
		+ std::hash::Hash
		+ std::str::FromStr;

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

	/// The user account identifier type for the runtime.
	type AccountId: Parameter + Ord + Default;

	/// The address type. This instead of `<frame_system::Trait::Lookup as StaticLookup>::Source`.
	type Address: Codec
		+ Clone
		+ PartialEq
		+ Debug
		+ Send
		+ Sync
		+ Eq
		+ Decode
		+ From<Self::AccountId>
		+ Ss58Codec; // TODO this should be conditional on std
}
