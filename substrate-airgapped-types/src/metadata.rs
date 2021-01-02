// use crate::frame::CallMethod;
use crate::{frame::ModuleCall, extrinsic::CallIndex };
use codec::alloc::collections::HashMap;
use core::{convert::TryFrom};

pub use frame_metadata::{
	DecodeDifferent, RuntimeMetadata, RuntimeMetadataPrefixed, META_RESERVED,
};

/// Runtime metadata.
#[derive(Clone, Debug)]
pub struct Metadata {
	modules_with_calls: HashMap<String, ModuleWithCalls>,
}

impl Metadata {
	/// Returns `ModuleWithCalls`.
	fn module_with_calls<S: ToString>(&self, name: S) -> Result<&ModuleWithCalls, String> {
		let name = name.to_string();
		self.modules_with_calls.get(&name).ok_or(format!("Module not found {}", name))
	}

	/// Encode a call with the bytes wrapped in `Encoded`
	pub fn call_index<C: ModuleCall>(&self, call: C) -> Result<CallIndex, String> {
		let module_with_calls = self.module_with_calls(call.pallet())?;
		let module_index = module_with_calls.index;
		let call_index = module_with_calls.calls.get(call.call()).expect("TODO Could not find call method");

		Ok(CallIndex::new(module_index, *call_index))
	}
}

#[derive(Clone, Debug)]
struct ModuleWithCalls {
	index: u8,
	name: String,
	calls: HashMap<String, u8>,
}

// impl ModuleWithCalls {
// 	/// TODO comment
// 	fn encode_call_encoded<C: ModuleCall>(&self, call: C) -> Result<Encoded<C>, String> {
// 		let bytes = self.encode_call(call)?;

// 		Ok(Encoded::<C>(bytes, PhantomData::<C>))
// 	}

// 	/// TODO maybe delete this
// 	fn encode_call<C: ModuleCall>(&self, call: C) -> Result<Vec<u8>, String> {
// 		let fn_index =
// 			self.calls.get(call.call()).ok_or(format!("Call not found {}", call.method()))?;
// 		let mut bytes = vec![self.index, *fn_index];
// 		bytes.extend(call.encode());

// 		Ok(bytes)
// 	}
// }

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

		let mut modules_with_calls = HashMap::new();
		for module in convert(meta.modules)?.into_iter() {
			if let Some(calls_meta) = module.calls {
				let mut calls = HashMap::new();
				for (index, call) in convert(calls_meta)?.into_iter().enumerate() {
					let call_name = convert(call.name)?;
					calls.insert(call_name, index as u8);
				}

				let module_name = convert(module.name)?;
				modules_with_calls.insert(
					module_name.clone(),
					ModuleWithCalls { index: module.index, name: module_name, calls },
				);
			}
		}

		Ok(Metadata { modules_with_calls: modules_with_calls })
	}
}

fn convert<B: 'static, O: 'static>(dd: DecodeDifferent<B, O>) -> Result<O, String> {
	match dd {
		DecodeDifferent::Decoded(value) => Ok(value),
		_ => Err("Expected decoded".into()),
	}
}
