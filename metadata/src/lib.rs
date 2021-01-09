//! Runtime metadata decoding and lookup support for substrate-airgapped
#![warn(missing_docs)]

use codec::alloc::collections::HashMap;
use core::convert::TryFrom;
use frame_metadata::{DecodeDifferent, META_RESERVED};
use substrate_airgapped::{CallIndex, PalletCall};

pub use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};

/// Runtime metadata.
#[derive(Clone, Debug)]
pub struct Metadata {
	modules_with_calls: HashMap<String, ModuleWithCalls>,
}

impl Metadata {
	/// Returns `ModuleWithCalls`.
	fn module_with_calls<S: ToString>(
		&self,
		name: S,
	) -> Result<&ModuleWithCalls, substrate_airgapped::Error> {
		let name = name.to_string();
		self.modules_with_calls
			.get(&name)
			.ok_or_else(|| "Module could not be found in runtime metadata".into())
	}

	/// Encode a call with the bytes wrapped in `Encoded`
	pub fn find_call_index<C: PalletCall>(&self) -> Result<CallIndex, substrate_airgapped::Error> {
		let module_with_calls = self.module_with_calls(C::PALLET)?;
		let module_index = module_with_calls.index;
		let call_index = module_with_calls
			.calls
			.get(C::CALL)
			.ok_or("Call could not be found in module runtime metadata")?;

		Ok(CallIndex::new(module_index, *call_index))
	}
}

#[derive(Clone, Debug)]
struct ModuleWithCalls {
	index: u8,
	name: String,
	calls: HashMap<String, u8>,
}

impl TryFrom<RuntimeMetadataPrefixed> for Metadata {
	type Error = substrate_airgapped::Error;

	fn try_from(metadata: RuntimeMetadataPrefixed) -> Result<Self, substrate_airgapped::Error> {
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
					calls.insert(call_name.to_string(), index as u8);
				}

				let module_name = convert(module.name)?.to_string();
				modules_with_calls.insert(
					module_name.clone(),
					ModuleWithCalls { index: module.index, name: module_name, calls },
				);
			}
		}

		Ok(Metadata { modules_with_calls })
	}
}

fn convert<B: 'static, O: 'static>(
	dd: DecodeDifferent<B, O>,
) -> Result<O, substrate_airgapped::Error> {
	match dd {
		DecodeDifferent::Decoded(value) => Ok(value),
		_ => Err("Expected decoded".into()),
	}
}
