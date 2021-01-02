use crate::{
	era::Era,
	extra::SignedExtra,
	frame::{balances::Balances, system::System, CallMethod, CallWithIndex},
	runtime::Runtime,
	signed_payload::SignedPayload,
};
use crate::{
	// signed_payload::SignedPayload,
	metadata::{Metadata, RuntimeMetadataPrefixed},
	unchecked_extrinsic::UncheckedExtrinsic,
	Encoded,
};
use codec::{Decode, Encode};
use core::convert::TryInto;
use core::fmt::Debug;
use hex;
use sp_core::Pair;
use sp_runtime::traits::SignedExtension;

// 1) Figure out how to encode call
// 2) Use encode call to create SignedPayload
// 3) Generate signed payload and signature
// 4) Use signature to create unchecked extrinsic

/// Airgapped Substrate Call variants // TODO does this need to be public?
#[derive(PartialEq, Eq, Clone, Debug)]
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
	/// Already built substrate Era variant
	Built(Era),
}

/// TODO should this or similar be public
pub struct CallOptions<C: CallMethod + Clone, R: System + Balances> {
	pub call: AirCall<C>,
	pub nonce: R::Index,
	pub signed: R::Address,
	// era: Mortality<R>,
}

/// Metadata necessary for extrinsic construction
pub struct ExtrinsicClient<C: CallMethod + Clone, R: System + Balances> {
	/// Decoded runtime metadata
	pub metadata: Metadata,
	/// TODO make getters?
	/// Runtime spec version
	pub spec_version: u32,
	/// Tx version used in the runtime
	pub tx_version: u32,
	/// Genesis hash of network
	pub genesis_hash: R::Hash,
	call_options: CallOptions<C, R>,
}

impl<C, R> ExtrinsicClient<C, R>
where
	C: CallMethod + Clone,
	R: System + Balances + Runtime + Clone + Debug + Eq + Send + Sync,
	<<R::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync,
{
	/// encoded_meta: Hex string of metadata
	pub fn new(
		encoded_meta: &str,
		spec_version: u32,
		tx_version: u32,
		genesis_hash: R::Hash,
		call_options: CallOptions<C, R>,
	) -> Result<ExtrinsicClient<C, R>, String> {
		Ok(ExtrinsicClient {
			metadata: Self::metadata_from_hex(encoded_meta)?,
			spec_version,
			tx_version,
			genesis_hash,
			call_options,
		})
	}

	// TODO make extrinsic constructor builder that finds call index and then returns an object for the call
	// that uses call index to encode/decode

	// /// Build the call TODO this should create a new thing, not return Self
	// /// should take call struct and nonce, era and tip are optional so can be set later
	// pub fn set_call_options(
	// 	&mut self,
	// 	call_struct: C,
	// 	nonce: R::Index,
	// 	era: Mortality<R>,
	// ) -> &Self {
	// 	self.call_options.call = Some(AirCall::Plain(call_struct));
	// 	self.call_options.nonce = Some(nonce);
	// 	self.call_options.era = Some(era);

	// 	self
	// }

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
		// TODO find a way to display
		let _module_index = bytes[0];
		let _call_index = bytes[1];
		let mut args = &bytes[2..];
		Ok(C::decode(&mut args).expect("TODO decode_call"))
	}

	/// Decode a call and include the module and call index
	pub fn decode_call_with_index(
		&self,
		encoded_call: Encoded<C>,
	) -> Result<CallWithIndex<C>, String> {
		let bytes = encoded_call.0;

		Ok(CallWithIndex {
			module_index: bytes[0],
			call_index: bytes[1],
			args: C::decode(&mut &bytes[2..]).expect("TODO decode_call_with_index"),
		})
	}

	/// Create a `SignedPayload`, which can be signed to create a signature for the transaction.
	pub fn create_signing_payload(
		&mut self,
	) -> Result<SignedPayload<C, <<R as Runtime>::Extra as SignedExtra<R>>::Extra>, String> {
		let encoded = match self.call_options.call.clone() {
			AirCall::Plain(call) => self.encode_call(call)?,
			AirCall::Encoded(encoded_call) => encoded_call,
		};

		// TODO make this configurable
		let era_info = (Era::immortal(), None::<R::Hash>);

		let extra = R::Extra::new(
			self.spec_version,
			self.tx_version,
			self.call_options.nonce,
			self.genesis_hash,
			era_info,
		);

		Ok(SignedPayload::new(encoded, extra.extra()).expect("TODO"))
	}

	/// Create a signed unchecked extrinsic
	pub fn create_unchecked_extrinsic(
		&mut self,
		signature: R::Signature,
	) -> Result<UncheckedExtrinsic<R, C, R::Extra>, String> {
		let encoded = match self.call_options.call.clone() {
			// TODO factor this out for reuse
			AirCall::Plain(call) => self.encode_call(call)?,
			AirCall::Encoded(encoded_call) => encoded_call,
		};

		// TODO make this configurable
		let era_info = (Era::immortal(), None::<R::Hash>);

		let extra = R::Extra::new(
			// TODO factor this out for reuse
			self.spec_version,
			self.tx_version,
			self.call_options.nonce,
			self.genesis_hash,
			era_info,
		);
		let ext = UncheckedExtrinsic::new_signed(
			encoded,
			self.call_options.signed.clone(),
			signature,
			extra,
		);

		Ok(ext)
	}
}
