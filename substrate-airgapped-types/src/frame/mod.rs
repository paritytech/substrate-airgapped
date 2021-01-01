use codec::{Codec, Decode, Encode, EncodeLike};
use core::fmt::Debug;

/// pallet balances
pub mod balances;
/// pallet system
pub mod system;

/// A type that can be used as a parameter in a dispatchable function. Defined here so we don't have
// to require frame-system (TODO Not yet sure if this is an ok approach)
pub trait Parameter: Codec + EncodeLike + Clone + Eq + Debug {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + Debug {}

/// Associates args struct with method name
pub trait CallMethod: Encode + Decode + Sized {
	/// Name of the method, as it appears in metadata
	fn method(&self) -> &'static str;
	/// Name of the pallet, as it appears in metadata
	fn pallet(&self) -> &'static str;
}

/// Call arguments with the module and call index
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct CallWithIndex<C: CallMethod> {
	/// Index of the module in the metadata
	pub module_index: u8,
	/// Index of the call in the module
	pub call_index: u8,
	/// Call method args
	pub args: C,
}
