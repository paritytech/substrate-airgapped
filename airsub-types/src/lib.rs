//! Air-gapped Substrate type support
#![warn(missing_docs)]
#![allow(type_alias_bounds)]

use core::fmt::Debug;

mod extra;
mod runtime;

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
	fn encode(&self) -> Vec<u8> {
		self.0.to_owned()
	}
}

/// UncheckedExtrinsic type.
pub type UncheckedExtrinsic<T> = sp_runtime::generic::UncheckedExtrinsic<
	<T as runtime::System>::Address,
	Encoded,
	<T as runtime::Runtime>::Signature,
	extra::DefaultExtra<T>,
>;

// pub fn decode_unchecked_extrinsic(unchecked_extrinsic: &mut b[..]) -> UncheckedExtrinsic {
// }
