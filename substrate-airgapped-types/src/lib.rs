//! Substrate airgapped transaction construction
#![warn(missing_docs)]

use codec::{Decode, Encode, Input};
use core::{fmt::Debug, marker::PhantomData};

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

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoded<T: Encode + Decode>(
	pub Vec<u8>,
	// TODO need the auto Decode derive for this to work
	// #[codec(skip)]
	pub PhantomData<T>,
);

impl<T: Encode + Decode> codec::Encode for Encoded<T> {
	fn encode(&self) -> Vec<u8> {
		self.0.to_owned()
	}
}

impl<'a, T: Encode + Decode> codec::Decode for Encoded<T> {
	// TODO sanity check
	fn decode<I: Input>(value: &mut I) -> Result<Self, codec::Error> {
		let value_len = value.remaining_len()?.ok_or("Codec Error: No length")?;
		let mut buf: Vec<u8> = Vec::with_capacity(value_len);
		value.read(&mut buf[..])?;

		Ok(Encoded(buf, PhantomData))
	}
}
