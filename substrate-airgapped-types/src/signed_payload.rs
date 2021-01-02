use crate::{
	frame::{system::System, CallMethod},
	hashing::blake2_256,
	runtime::Runtime,
	unchecked_extrinsic::Extra,
	Encoded,
};
use codec::Encode;
use sp_core::Pair;
use sp_runtime::{traits::SignedExtension, transaction_validity::TransactionValidityError};

/// A payload that has been signed for an unchecked extrinsics.
///
/// Note that the payload that we sign to produce unchecked extrinsic signature
/// is going to be different than the `SignaturePayload` - so the thing the extrinsic
/// actually contains.
// #[derive(Clone, Eq, PartialEq, Debug)]
pub struct SignedPayload<Call: CallMethod, R: System + Runtime>(
	(Encoded<Call>, Extra<R>, <Extra<R> as SignedExtension>::AdditionalSigned),
);

impl<Call, R> SignedPayload<Call, R>
where
	Call: CallMethod,
	R: System + Runtime,
{
	/// Create new `SignedPayload`.
	///
	/// This function may fail if `additional_signed` of `Extra` is not available.
	pub fn new(call: Encoded<Call>, extra: Extra<R>) -> Result<Self, TransactionValidityError> {
		let additional_signed = extra.additional_signed()?;
		let raw_payload = (call, extra, additional_signed);

		Ok(Self(raw_payload))
	}

	pub fn sign<P: Pair>(&self, pair: P) -> P::Signature {
		self.using_encoded(|payload| pair.sign(payload))
	}
}

impl<Call, R> Encode for SignedPayload<Call, R>
where
	Call: CallMethod,
	R: System + Runtime,
{
	/// Get an encoded version of this payload.
	///
	/// Payloads longer than 256 bytes are going to be `blake2_256`-hashed.
	fn using_encoded<U, F: FnOnce(&[u8]) -> U>(&self, f: F) -> U {
		self.0.using_encoded(|payload| {
			if payload.len() > 256 {
				f(&blake2_256(payload)[..])
			} else {
				f(payload)
			}
		})
	}
}
