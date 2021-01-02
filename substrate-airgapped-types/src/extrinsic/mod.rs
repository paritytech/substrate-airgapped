use crate::{
	frame::{balances::Balances, system::System},
	runtime::Runtime,
};
use codec::{Decode, Encode, Input};
use core::fmt::Debug;

pub(crate) mod extra;

/// Method with the method and module index
#[derive(Clone, Copy, Debug, PartialEq, Encode, Decode)]
pub struct CallIndex {
	module_index: u8,
	call_index: u8,
}

impl CallIndex {
	/// Get `module`, the index of the module the call is in
	// pub fn module_index(&self) -> u8 {
	// 	self.module_index
	// }

	/// Get `call_index`, the index of the call in the module
	// pub fn call_index(&self) -> u8 {
	// 	self.call_index
	// }

	/// Create `CallIndex`
	pub fn new(module_index: u8, call_index: u8) -> CallIndex {
		CallIndex { module_index, call_index }
	}

	/// Create a slice representing the call index
	pub fn to_vec(self) -> Vec<u8> {
		vec![self.module_index, self.call_index]
	}
}

/// Call represented by call index and argument struct.
/// This has the ability to correctly encode and decode itself without metadata.
struct CallIndexArgs<C: Encode + Decode> {
	call_index: CallIndex,
	call: C,
}

// impl<C: Encode + Decode> CallIndexArgs<C> {

// }

impl<C: Encode + Decode> Encode for CallIndexArgs<C> {
	fn encode(&self) -> Vec<u8> {
		let mut bytes = self.call_index.to_vec();
		bytes.extend(self.call.encode());

		bytes
	}
}

impl<C: Encode + Decode> Decode for CallIndexArgs<C> {
	fn decode<I: Input>(value: &mut I) -> Result<Self, codec::Error> {
		let value_len = value.remaining_len()?.ok_or("Codec Error: No length")?;
		let mut buf: Vec<u8> = Vec::with_capacity(value_len);
		value.read(&mut buf[..])?;

		Ok(CallIndexArgs {
			call_index: CallIndex::new(buf[0], buf[1]),
			call: C::decode(&mut &buf[2..]).unwrap(),
		})
	}
}

/// UncheckedExtrinsic type.
pub type UncheckedExtrinsic<C, R> = sp_runtime::generic::UncheckedExtrinsic<
    <R as System>::Address,
    CallIndexArgs<C>,
    <R as Runtime>::Signature,
    extra::Extra<R>,
>;

/// Transaction builder all the components to create a signing payload.
pub struct TxBuilder<C: Encode + Decode, R: System + Balances + Runtime> {
	/// Call with all info for encoding and decoding
	call_index_args: CallIndexArgs<C>,
	/// Signers Address
	address: R::Address,
	/// Signers nonce
	nonce: R::Index,
	// TODO tip, era_period, checkpoint_block_hash, checkpoint_block_number
}

impl<C: Encode + Decode, R: System + Balances + Runtime> TxBuilder<C, R> {
	/// Create transaction builder, all the components to create a signing payload.
	pub fn new(call_index: CallIndex, call: C, address: R::Address, nonce: R::Index) -> Self {
		TxBuilder { call_index_args: CallIndexArgs { call_index, call } , address, nonce }
	}

	pub fn tx_from_pair<P: Pair>(pair: P) -> Result<UncheckExtrinsic<C, R> {
		
	}
}
