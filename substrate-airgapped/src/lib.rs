//! Substrate airgapped transaction construction
#![warn(missing_docs)]

use codec::{Decode, Encode, Input};

mod extrinsic;
mod frame;
mod metadata;
mod runtimes;

pub use crate::{
	extrinsic::{CallIndex, GenericCall, TxBuilder},
	frame::{balances, system},
	metadata::Metadata,
	runtimes::PolkadotRuntime,
};

