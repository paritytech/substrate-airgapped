use codec::Decode;
use core::convert::TryInto;
use substrate_airgapped::{balances::Transfer, GenericCall};

use metadata::{Metadata, RuntimeMetadataPrefixed};
use serde::{Deserialize, Serialize};
use sp_keyring::AccountKeyring;
use sp_runtime::DeserializeOwned;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let metadata_bytes = rpc_to_local_node::<(), String>("chain_getBlockHash", vec![])
		.and_then(|rpc_res| Ok(hex::decode(rpc_res.result)?))?;

	let metadata_prefixed: RuntimeMetadataPrefixed = Decode::decode(&mut &metadata_bytes[..])?;
	let metadata: Metadata = metadata_prefixed.try_into()?;

	let args = Transfer { to: AccountKeyring::Bob.to_account_id().into(), amount: 123_456_789 };
	let call_index = metadata.find_call_index(args)?;
	let transfer_call = GenericCall::new(call_index, args);

	Ok(())
}

/// RPC response JSON object
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRes<T> {
	jsonrpc: String,
	/// Result of the RPC execution
	pub result: T,
}

/// RPC request JSON object
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcReq<T: Serialize> {
	jsonrpc: String,
	id: usize,
	method: String,
	params: Vec<T>,
}

/// Send an RPC request to the default local node http address.
pub fn rpc_to_local_node<T: Serialize, U: DeserializeOwned>(
	method: &str,
	params: Vec<T>,
) -> Result<RpcRes<U>, Box<dyn std::error::Error>> {
	let local_node_url = "http://localhost:9933";
	let client = reqwest::blocking::Client::new();

	let req_body = RpcReq { jsonrpc: "2.0".to_owned(), id: 1, method: method.to_owned(), params };
	let res = client.post(local_node_url).json(&req_body).send()?.json()?;

	Ok(res)
}
