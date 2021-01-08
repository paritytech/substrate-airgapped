use codec::{Decode, Encode, Input};
use core::fmt::Debug;

/// Call arguments with the call and module index. The indexes are needed encoding.
#[derive(Clone, Copy, Debug, PartialEq, Encode, Decode)]
pub struct CallIndex {
	module_index: u8,
	call_index: u8,
}

impl CallIndex {
	/// Get `module`, the index of the module the call is in
	pub fn module_index(&self) -> u8 {
		self.module_index
	}

	/// Get `call_index`, the index of the call in the module
	pub fn call_index(&self) -> u8 {
		self.call_index
	}

	/// Create `CallIndex`
	pub fn new(module_index: u8, call_index: u8) -> CallIndex {
		CallIndex { module_index, call_index }
	}

	/// Create a vec representing the call index
	pub fn to_vec(&self) -> Vec<u8> {
		vec![self.module_index, self.call_index]
	}
}

/// Call (a.k.a dispatchable) represented by call index and argument struct.
/// This has the ability to correctly encode and decode itself.
#[derive(Clone, Debug, PartialEq)]
pub struct GenericCall<C: Encode + Decode + Clone> {
	call_index: CallIndex,
	args: C,
}

impl<C> GenericCall<C>
where C: Encode + Decode + Clone {
	/// Create a `GenericCall`
	pub fn new(call_index: CallIndex, args: C) -> Self {
		Self { call_index, args}
	}
	/// `CallIndex` of the call
	pub fn call_index(&self) -> &CallIndex {
		&self.call_index
	}
	/// Arguments of the call
	pub fn args(&self) -> &C {
		&self.args
	}
}

impl<C: Encode + Decode + Clone> Encode for GenericCall<C> {
	fn encode(&self) -> Vec<u8> {
		let mut bytes = self.call_index.to_vec();
		bytes.extend(self.args.encode());

		bytes
	}
}

impl<C: Encode + Decode + Clone> Decode for GenericCall<C> {
	fn decode<I: Input>(value: &mut I) -> Result<Self, codec::Error> {
		let value_len = value.remaining_len()?.ok_or("Codec Error: No length")?;
		let mut buf = vec![0; value_len];
		value.read(&mut buf[..])?;

		Ok(GenericCall {
			call_index: CallIndex::new(buf[0], buf[1]),
			args: C::decode(&mut &buf[2..]).unwrap(),
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{balances::Transfer, CallIndex, KusamaRuntime};
	use sp_keyring::AccountKeyring;

	type TransferType = Transfer<KusamaRuntime>;
	type TransferCall = GenericCall<TransferType>;

	#[test]
	fn generic_call_encode_decode() {
		let bob_addr = AccountKeyring::Bob.to_account_id().into();
		let transfer_args: TransferType = Transfer { to: bob_addr, amount: 12 };
		let transfer = GenericCall::new(CallIndex::new(5, 0), transfer_args);

		// Independent parts of the call encode as expected
		assert_eq!(transfer.call_index.to_vec(), [5, 0]);
		assert_eq!(
			transfer.args.encode(),
			[
				142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97,
				54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72, 48,
			]
		);

		// The call itself encodes and decodes as expected
		let mut call_encoded_expected: &[u8] = &[
			5, 0, 142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97,
			54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72, 48,
		];
		assert_eq!(transfer.encode(), call_encoded_expected);
		let decoded_call =
			TransferCall::decode(&mut call_encoded_expected).expect("decodes from a encoded call");
		assert_eq!(decoded_call, transfer);
	}
}
