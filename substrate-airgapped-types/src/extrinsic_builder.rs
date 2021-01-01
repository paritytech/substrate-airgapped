use crate::frame::{balances::Balances, system::System, CallMethod};
use crate::{
	// unchecked_extrinsic::UncheckedExtrinsic,
	// signed_payload::SignedPayload,
	metadata::{Metadata, RuntimeMetadataPrefixed},
	Encoded,
};
use codec::{Decode, Encode};
use core::convert::TryInto;
use hex;

// 1) Figure out how to encode call
// 2) Use encode call to create SignedPayload
// 3) Generate signed payload and signature
// 4) Use signature to create unchecked extrinsic

/// Airgapped Substrate Call variants // TODO does this need to be public?
pub enum AirCall<C: Encode + Decode> {
	/// Encoded call method arguments
	Encoded(Encoded<C>),
	/// Call method arguments
	Plain(C),
}

/// Options for specifying mortality
pub enum Mortality<R: System> {
	/// Specify mortal period and hash of the block the period should st art
	Mortal(u64, R::Hash),
	/// Specify a immortal transaction
	Immortal,
}

/// TODO should this or similar be public
pub struct CallOptions<C: CallMethod + Clone, R: System + Balances> {
	call: Option<AirCall<C>>,
	nonce: Option<R::Index>,
	era: Option<Mortality<R>>
}

/// Metadata necessary for extrinsic construction
pub struct ExtrinsicClient<C: CallMethod + Clone, R: System + Balances> {
	/// Decoded runtime metadata
	pub metadata: Metadata, /// TODO make getters?
	/// Runtime spec version
	pub spec_version: u32,
	/// Tx version used in the runtime
	pub tx_version: u32,
	/// Genesis hash of network
	pub genesis_hash: R::Hash,
	call_options: CallOptions<C, R>,
}

impl<C: CallMethod + Clone, R: System + Balances> ExtrinsicClient<C, R> {
	/// encoded_meta: Hex string of metadata
	pub fn new(
		encoded_meta: &str,
		spec_version: u32,
		tx_version: u32,
		genesis_hash: R::Hash
	) -> Result<ExtrinsicClient<C, R>, String> {
		Ok(ExtrinsicClient {
			metadata: Self::metadata_from_hex(encoded_meta)?,
			spec_version,
			tx_version,
			genesis_hash,
			call_options: CallOptions { call: None, nonce: None, era: None },
		})
	}

	/// Build the call TODO this should create a new thing, not return Self
	/// should take call struct and nonce, era and tip are optional so can be set later
	pub fn set_call_options(
		&mut self,
		call_struct: C,
		nonce: R::Index,
		era: Mortality<R>,
	) -> &Self {
		self.call_options.call = Some(AirCall::Plain(call_struct));
		self.call_options.nonce = Some(nonce);
		self.call_options.era = Some(era);

		self
	}

	// TODO create a struct for tx metadata, then have ExtrinsicBuilder Point to it so they can share metadata
	/// Metadata hex string must not have leading 0x
	fn metadata_from_hex(encoded_meta: &str) -> Result<Metadata, String> {
		let bytes = hex::decode(encoded_meta).expect("TODO");
		let metadata_prefixed: RuntimeMetadataPrefixed =
			Decode::decode(&mut &bytes[..]).expect("TODO");

		metadata_prefixed.try_into()
	}

	/// Encode the call method specified by the call_options
	pub fn encode_call(&self, call_struct: C) -> Result<Encoded<C>, String> {
		self.metadata.encode_call(call_struct.clone())
	}

	/// Decode a call that is wrapped in Encoded
	pub fn decode_call(&self, encoded_call: Encoded<C>) -> Result<C, String> {
		let bytes = encoded_call.0;
		Ok(C::decode(&mut &bytes[..]).expect("TODO make better errors"))
	}
}
