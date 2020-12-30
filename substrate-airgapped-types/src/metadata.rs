use crate::{ frame::CallMethod, Encoded};
use codec::{Encode, Decode};
use codec::alloc::collections::HashMap;
use core::marker::PhantomData;

/// Runtime metadata.
pub struct Metadata{
	modules_with_calls: HashMap<String, ModuleWithCalls>
}

impl Metadata {
	    /// Returns `ModuleWithCalls`.
    pub fn module_with_calls<S: ToString>(&self, name: S) -> Result<&ModuleWithCalls, String> {
        let name = name.to_string();
        self.modules_with_calls
            .get(&name)
            .ok_or(format!("Module not found {}", name))
	}
}

#[derive(Clone, Debug)]
pub struct ModuleWithCalls {
	index: u8,
	name: String,
    calls: HashMap<String, u8>,
}

impl ModuleWithCalls {
    pub fn encode_call<T: Encode + Decode, U: CallMethod + Encode>(
        &self,
       	call: U,
    ) -> Result<Encoded<T>,  String> {
        let fn_index = self
            .calls
            .get(call.method())
            .ok_or(format!("Call not found {}", call.method()))?;
        let mut bytes = vec![self.index, *fn_index];
        bytes.extend(call.encode());
        Ok(Encoded::<T>(bytes, PhantomData::<T>))
    }
}