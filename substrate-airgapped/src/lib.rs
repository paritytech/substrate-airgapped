//! Substrate airgapped transaction construction
#![warn(missing_docs)]

mod error;
mod frame;
mod runtimes;
mod tx;

pub use crate::{
	error::Error,
	frame::{balances, system, PalletCall},
	runtimes::KusamaRuntime,
	tx::{
		tx_from_parts, CallIndex, GenericCall, MortalConfig, Mortality, SignedPayload, Tx,
		TxConfig, UncheckedExtrinsic,
	},
};
