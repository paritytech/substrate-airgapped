use codec::{Codec, EncodeLike};
use core::fmt::Debug;

/// pallet balances
pub mod balances;
/// pallet system
pub mod system;

/// A type that can be used as a parameter in a dispatchable function. Defined here so we don't have
// to require frame-system (TODO Not yet sure if this is an ok approach)
pub trait Parameter: Codec + EncodeLike + Clone + Eq + Debug {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + Debug {}
