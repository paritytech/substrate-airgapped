/// pallet balances
pub mod balances;
/// pallet system
pub mod system;

/// A type that can be used as a parameter in a dispatchable function.
///
/// When using `decl_module` all arguments for call functions must implement this trait.
///
/// Same as `frame_support::Parameter`. Re-defined to avoid dependance on frame-support.
pub trait Parameter: codec::Codec + Clone + Eq + core::fmt::Debug {}
impl<T> Parameter for T where T: codec::Codec + Clone + Eq + core::fmt::Debug {}

/// Trait that call argument definitions should implement. Allows for look ups of
/// the call in metadata.
pub trait PalletCall {
	/// Name of the call, as it appears in metadata
	const CALL: &'static str;
	/// Name of the pallet, as it appears in metadata
	const PALLET: &'static str;
}
