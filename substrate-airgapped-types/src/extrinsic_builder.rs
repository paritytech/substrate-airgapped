// use crate::{
// 	era::Era,
// 	extra::SignedExtra,
// 	frame::{balances::Balances, system::System, CallMethod, CallWithIndex},
// 	runtime::Runtime,
// };
// use crate::{
// 	// signed_payload::SignedPayload,
// 	metadata::{Metadata, RuntimeMetadataPrefixed},
// 	// unchecked_extrinsic::UncheckedExtrinsic,
// 	Encoded,
// };
// use codec::{Decode, Encode};
// use core::convert::TryInto;
// use core::fmt::Debug;
// use hex;
// use sp_core::Pair;
// use sp_runtime::traits::SignedExtension;

// /// UncheckedExtrinsic type.
// pub type UncheckedExtrinsic<R, C> = sp_runtime::generic::UncheckedExtrinsic<
// 	<R as System>::Address,
// 	Encoded<C>,
// 	<R as Runtime>::Signature,
// 	Extra<R>,
// >;

// // TODO move this to extra.rs
// /// Extra type.
// pub type Extra<R> = <<R as Runtime>::Extra as SignedExtra<R>>::Extra;

// /// SignedPayload type.
// pub type SignedPayload<C, R> = sp_runtime::generic::SignedPayload<Encoded<C>, Extra<R>>;

// // 1) Figure out how to encode call
// // 2) Use encode call to create SignedPayload
// // 3) Generate signed payload and signature
// // 4) Use signature to create unchecked extrinsic

// /// Airgapped Substrate Call variants // TODO does this need to be public?
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub enum AirCall<C: Encode + Decode> {
// 	/// Encoded call method arguments
// 	Encoded(Encoded<C>),
// 	/// Call method arguments
// 	Plain(C),
// }

// /// Options for specifying mortality
// pub enum Mortality<R: System> {
// 	/// Specify mortal period and hash of the block the period should st art
// 	Mortal(u64, R::Hash),
// 	/// Specify a immortal transaction
// 	Immortal,
// 	/// Already built substrate Era variant
// 	Built(Era),
// }

// /// TODO should this or similar be public
// pub struct CallOptions<C: CallMethod + Clone, R: System + Balances> {
// 	/// TODO
// 	pub call: AirCall<C>,
// 	/// TODO
// 	pub nonce: R::Index,
// 	/// TODO
// 	pub address: R::Address,
// 	// era: Mortality<R>,
// }

// /// Metadata necessary for extrinsic construction
// pub struct ExtrinsicClient<C: CallMethod + Clone, R: System + Balances> {
// 	/// Decoded runtime metadata
// 	pub metadata: Metadata,
// 	/// TODO make getters?
// 	/// Runtime spec version
// 	pub spec_version: u32,
// 	/// Tx version used in the runtime
// 	pub tx_version: u32,
// 	/// Genesis hash of network
// 	pub genesis_hash: R::Hash,
// 	call_options: CallOptions<C, R>,
// }

// impl<C, R> ExtrinsicClient<C, R>
// where
// 	C: CallMethod + Clone,
// 	R: System + Balances + Runtime + Clone + Debug + Eq + Send + Sync,
// 	<<R::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync,
// {
// 	/// encoded_meta: Hex string of metadata
// 	pub fn new(
// 		encoded_meta: &str,
// 		spec_version: u32,
// 		tx_version: u32,
// 		genesis_hash: R::Hash,
// 		call_options: CallOptions<C, R>,
// 	) -> Result<ExtrinsicClient<C, R>, String> {
// 		Ok(ExtrinsicClient {
// 			metadata: Self::metadata_from_hex(encoded_meta)?,
// 			spec_version,
// 			tx_version,
// 			genesis_hash,
// 			call_options,
// 		})
// 	}

// 	// TODO create a struct for tx metadata, then have ExtrinsicBuilder Point to it so they can share metadata
// 	/// Metadata hex string must not have leading 0x
// 	fn metadata_from_hex(encoded_meta: &str) -> Result<Metadata, String> {
// 		let bytes = hex::decode(encoded_meta).expect("TODO");
// 		let metadata_prefixed: RuntimeMetadataPrefixed =
// 			Decode::decode(&mut &bytes[..]).expect("TODO");

// 		metadata_prefixed.try_into()
// 	}

// 	/// Encode the call method specified by the call_options
// 	pub fn encode_call(&self, call_struct: C) -> Result<Encoded<C>, String> {
// 		self.metadata.encode_call(call_struct.clone())
// 	}

// 	/// Decode a call that is wrapped in Encoded
// 	pub fn decode_call(&self, encoded_call: Encoded<C>) -> Result<C, String> {
// 		let bytes = encoded_call.0;
// 		// TODO find a way to display
// 		let _module_index = bytes[0];
// 		let _call_index = bytes[1];
// 		let mut args = &bytes[2..];
// 		Ok(C::decode(&mut args).expect("TODO decode_call"))
// 	}

// 	/// Decode a call and include the module and call index
// 	pub fn decode_call_with_index(
// 		&self,
// 		encoded_call: Encoded<C>,
// 	) -> Result<CallWithIndex<C>, String> {
// 		let bytes = encoded_call.0;

// 		Ok(CallWithIndex {
// 			module_index: bytes[0],
// 			call_index: bytes[1],
// 			args: C::decode(&mut &bytes[2..]).expect("TODO decode_call_with_index"),
// 		})
// 	}

// 	/// Create an unchecked_extrinsic from sp_core::Pair
// 	pub fn create_unchecked_extrinsic_from_pair<P>(
// 		&self,
// 		pair: P,
// 	) -> Result<UncheckedExtrinsic<R, C>, String>
// 	where
// 		P: Pair,
// 		<R as Runtime>::Signature: From<<P as sp_core::Pair>::Signature>,
// 	{
// 		let encoded = match self.call_options.call.clone() {
// 			// TODO factor this out for reuse
// 			AirCall::Plain(call) => self.encode_call(call)?,
// 			AirCall::Encoded(encoded_call) => encoded_call,
// 		};

// 		let extra = R::Extra::new(
// 			// TODO factor this out for reuse
// 			self.spec_version,
// 			self.tx_version,
// 			self.call_options.nonce,
// 			self.genesis_hash,
// 		);

// 		let payload = SignedPayload::<C, R>::new(encoded, extra.extra())
// 			.expect("TODO failed to make signed payload");
// 		let signature = payload.using_encoded(|payload| pair.sign(payload));
// 		let (call, extra, _) = payload.deconstruct();

// 		let extrinsic = UncheckedExtrinsic::<R, C>::new_signed(
// 			call,
// 			self.call_options.address.clone(),
// 			signature.into(),
// 			extra,
// 		);

// 		Ok(extrinsic)
// 	}
// }
