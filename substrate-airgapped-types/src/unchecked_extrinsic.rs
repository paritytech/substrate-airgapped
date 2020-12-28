use crate::{extra::SignedExtra, frame::system::System, runtime::Runtime};
use codec::{self, Decode, Encode, Input};

const TRANSACTION_VERSION: u8 = 4;

// type Call: Encode + Decode + Clone + PartialEq + Eq;

// TODO move this to extra.rs
/// Extra type.
pub type Extra<T> = <<T as Runtime>::Extra as SignedExtra<T>>::Extra;

/// A extrinsic right from the external world. This is unchecked and so can contain a signature.
///
/// Equivalent of `sp_runtime::generic::UncheckedExtrinsic`
///
/// Type arguments sp-runtime => airgapped:
/// Call => Encoded<Call>, Address => T as System>::Address, Signature => <T as Runtime>::Signature, Extra => Extra<T>
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UncheckedExtrinsic<T, C>
where
	T: System + Runtime,
	C: Encode + Decode,
{
	/// The signature, address, number of extrinsics have come before from the same signer and an era
	/// describing the longevity of this transaction, if this is a signed extrinsic.
	pub signature: Option<(<T as System>::Address, <T as Runtime>::Signature, Extra<T>)>,
	/// The function that should be called.
	pub function: C,
}

impl<T, C> Decode for UncheckedExtrinsic<T, C>
where
	T: System + Runtime,
	C: Encode + Decode,
{
	fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
		// This is a little more complicated than usual since the binary format must be compatible
		// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length (we don't need
		// to use this).
		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;

		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != TRANSACTION_VERSION {
			return Err("Invalid transaction version".into());
		}

		Ok(UncheckedExtrinsic {
			signature: if is_signed { Some(Decode::decode(input)?) } else { None },
			function: Decode::decode(input)?,
		})
	}
}

impl<T, C> Encode for UncheckedExtrinsic<T, C>
where
	T: System + Runtime,
	C: Encode + Decode,
{
	fn encode(&self) -> Vec<u8> {
		encode_with_vec_prefix::<Self, _>(|v| {
			// 1 byte version id.
			match self.signature.as_ref() {
				Some(s) => {
					v.push(TRANSACTION_VERSION | 0b1000_0000);
					s.encode_to(v);
				}
				None => {
					v.push(TRANSACTION_VERSION & 0b0111_1111);
				}
			}
			self.function.encode_to(v);
		})
	}
}

fn encode_with_vec_prefix<T: Encode, F: Fn(&mut Vec<u8>)>(encoder: F) -> Vec<u8> {
	// TODO can we avoid requiring sp_std?
	use sp_std::prelude::*;
	let size = ::sp_std::mem::size_of::<T>();
	let reserve = match size {
		0..=0b00111111 => 1,
		0..=0b00111111_11111111 => 2,
		_ => 4,
	};
	let mut v = Vec::with_capacity(reserve + size);
	v.resize(reserve, 0);
	encoder(&mut v);

	// need to prefix with the total length to ensure it's binary compatible with
	// Vec<u8>.
	let mut length: Vec<()> = Vec::new();
	length.resize(v.len() - reserve, ());
	length.using_encoded(|s| {
		v.splice(0..reserve, s.iter().cloned());
	});

	v
}
