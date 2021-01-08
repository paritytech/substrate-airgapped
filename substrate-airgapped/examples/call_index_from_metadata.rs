use ::core::convert::TryInto;
use codec::Decode;
use substrate_airgapped::{balances::Transfer, GenericCall, KusamaRuntime};

use metadata::{RuntimeMetadataPrefixed, Metadata};
use serde::{Deserialize, Serialize};
use sp_keyring::AccountKeyring;
use sp_runtime::DeserializeOwned;

/// To get a local development node started, follow the instructions in the
/// paritytech/polkadot README and then start the dev node with the command
/// described [here](https://github.com/paritytech/polkadot#development).
///
/// For this example, we assume the nodes http RPC port is accessible via
///`http://localhost:9933`, which is the default.
///
/// Prior to running the following example, you will need to start up a polkadot
/// `--dev` node, as it is queried to get the runtime metadata.
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let metadata_bytes = rpc_to_local_node::<(), String>("state_getMetadata", vec![])
		.and_then(|rpc_res| {
			// Remove the leading "0x"
			let no_prefix = &rpc_res.result[2..];
			Ok(hex::decode(no_prefix)?)
		})?;

	let metadata_prefixed = RuntimeMetadataPrefixed::decode(&mut &metadata_bytes[..])?;
	let metadata: Metadata = metadata_prefixed.try_into()?;

	let args: Transfer<KusamaRuntime> = Transfer { to: AccountKeyring::Bob.to_account_id().into(), amount: 123_456_789 };
	// Use the `CallMethod` to fetch the `CallIndex` of the dispatchable
	let call_index = metadata.find_call_index(&args)?;
	println!("Call index for balances::Transfer: {:#?}", call_index);

	// You can then use the returned call index to construct a `GenericCall` - the type needed to
	// construct a `UncheckedExtrinsic`
	let transfer_call = GenericCall::new(call_index, args);
	println!("SCALE Encode-able balances::Transfer call: {:#?}", transfer_call);

	Ok(())
}

// These are redundant utils as they are also used in the other example.
// There should be a better solution.
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
