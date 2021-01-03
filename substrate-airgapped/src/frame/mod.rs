/// pallet balances
pub mod balances;
/// pallet system
pub mod system;

/// Trait that call argument definitions should implement. Allows for look ups of
/// the call in metadata.
pub trait ModuleCall {
	/// Name of the cal, as it appears in metadata
	fn call(&self) -> &'static str;
	/// Name of the pallet, as it appears in metadata
	fn pallet(&self) -> &'static str;
}
