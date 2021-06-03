use crate::{frame::system::System, util::int_as_human};
use core::fmt::{self, Display};

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

impl<R: System> Display for MortalConfig<R> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{sp}period: {}\n{sp}checkpoint_block_number: {}\n{sp}checkpoint_block_hash: {:#?}",
			self.period,
			int_as_human(self.checkpoint_block_number),
			self.checkpoint_block_hash,
			sp = format!("{:indent$}", "", indent = 8)
		)
	}
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

impl<R: System> Display for Mortality<R> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Mortality::Mortal(config) => write!(f, "Mortal:\n{}", config),
			Mortality::Immortal => write!(f, "Immortal"),
		}
	}
}
