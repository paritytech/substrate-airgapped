use crate::frame::system::System;

/// Mortal period configuration options,
///
/// Read here for conceptual details: https://docs.rs/sp-runtime/2.0.0/sp_runtime/generic/enum.Era.html
#[derive(Clone, Eq, PartialEq, Debug, Copy)]
pub struct MortalConfig<R: System> {
	/// Duration of the transactions validity, measured in blocks, starting from the checkpoint block.
	pub period: u64,
	/// Hash of the block where the transaction's mortality period starts.
	pub checkpoint_block_hash: R::Hash,
	/// Block number where the transaction mortality period starts.
	pub checkpoint_block_number: u64,
}

/// Specify the mortality of a transaction.
///
/// Read here for conceptual details: https://docs.rs/sp-runtime/2.0.0/sp_runtime/generic/enum.Era.html
#[derive(Clone, Eq, PartialEq, Debug, Copy)]
pub enum Mortality<R: System> {
	/// Specify a mortal transaction with period, checkpoint block number, and checkpoint block hash.
	Mortal(MortalConfig<R>),
	/// Specify an immortal transaction.
	Immortal,
}
