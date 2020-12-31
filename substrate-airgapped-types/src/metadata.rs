use crate::{frame::CallMethod, Encoded};
use codec::alloc::collections::HashMap;
use codec::{Decode, Encode};
use core::{convert::TryFrom, marker::PhantomData};

use frame_metadata::{DecodeDifferent, RuntimeMetadata, RuntimeMetadataPrefixed, META_RESERVED};

/// Runtime metadata.
pub struct Metadata {
	modules_with_calls: HashMap<String, ModuleWithCalls>,
}

impl Metadata {
	/// Returns `ModuleWithCalls`.
	fn module_with_calls<S: ToString>(&self, name: S) -> Result<&ModuleWithCalls, String> {
		let name = name.to_string();
		self.modules_with_calls.get(&name).ok_or(format!("Module not found {}", name))
	}

	/// Encode a call
	pub fn encode_call<T: Encode + Decode, U: CallMethod + Encode>(&self, call: U) -> Result<Encoded<T>, String> {
		Ok(self.module_with_calls(call.method()).and_then(|module| module.encode_call(call))?)
	}
}

#[derive(Clone, Debug)]
struct ModuleWithCalls {
	index: u8,
	name: String,
	calls: HashMap<String, u8>,
}

impl ModuleWithCalls {
	fn encode_call<T: Encode + Decode, U: CallMethod + Encode>(&self, call: U) -> Result<Encoded<T>, String> {
		let fn_index = self.calls.get(call.method()).ok_or(format!("Call not found {}", call.method()))?;
		let mut bytes = vec![self.index, *fn_index];
		bytes.extend(call.encode());
		Ok(Encoded::<T>(bytes, PhantomData::<T>))
	}
}

impl TryFrom<RuntimeMetadataPrefixed> for Metadata {
	type Error = String;

	fn try_from(metadata: RuntimeMetadataPrefixed) -> Result<Self, String> {
		if metadata.0 != META_RESERVED {
			return Err("Failed to convert".into());
		}
		let meta = match metadata.1 {
			RuntimeMetadata::V12(meta) => meta,
			_ => return Err("Invalid metadata version".into()),
		};

		let modules_with_calls =
			convert(meta.modules)?.into_iter().fold(Ok(HashMap::new()), |modules_map, module| {
				if let Some(calls_meta) = module.calls {
					let calls =
						convert(calls_meta)?.into_iter().enumerate().fold(Ok(HashMap::new()), |acc, (index, call)| {
							let call_name = convert(call.name)?;
							acc?.insert(call_name, index as u8);
							acc
						});

					let module_name = convert(module.name)?;
					modules_map?
						.insert(module_name.clone(), ModuleWithCalls { index: module.index, name: module_name, calls: calls? });
				}

				modules_map
			})?;

		Ok(Metadata { modules_with_calls })
	}
}

fn convert<B: 'static, O: 'static>(dd: DecodeDifferent<B, O>) -> Result<O, String> {
	match dd {
		DecodeDifferent::Decoded(value) => Ok(value),
		_ => Err("Expected decoded".into()),
	}
}
