use codec::Encode;
use sp_runtime::{ generic::Header, DeserializeOwned, traits::BlakeTwo256};
use substrate_airgapped::{
	balances::Transfer, CallIndex, GenericCall, KusamaRuntime, Mortality, Tx,
};
use sp_core::H256;

// Example deps
use hex;
use reqwest::{self};
use serde::{Deserialize, Serialize};
use sp_keyring::AccountKeyring;
use sp_version::RuntimeVersion;
use std::convert::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Get the latest block hash and then make all non historic queries at that block.
	let block_hash = rpc_to_local_node::<(), String>("chain_getBlockHash", vec![])?.result;
	let runtime_version = rpc_to_local_node::<String, RuntimeVersion>("chain_getRuntimeVersion", vec![block_hash.clone()])?
		.result;
	let header = rpc_to_local_node::<String, Header<u32, BlakeTwo256>>("chain_getHeader", vec![block_hash.clone()])?.result;
	let genesis_hash =
		rpc_to_local_node::<usize, String>("chain_getBlockHash", vec![0])
			.and_then(|rpc_res| Ok(string_to_h256(&rpc_res.result)))?;
	let block_hash = string_to_h256(&block_hash[..]);

	type Runtime = KusamaRuntime;
	type TransferType = Transfer<Runtime>;

	let alice_addr = AccountKeyring::Alice.to_account_id().into();
	let bob_addr = AccountKeyring::Bob.to_account_id().into();
	let transfer_args = Transfer { to: bob_addr, amount: 123_456 };
	let call_index = CallIndex::new(5, 0);
	let transfer_call = GenericCall { call_index, args: transfer_args };

	let tx: Tx<TransferType, Runtime> = Tx::new(
		transfer_call,
		alice_addr,
		2,
		runtime_version.transaction_version,
		runtime_version.spec_version,
		genesis_hash,
		Mortality::Mortal(64, header.number as u64, block_hash),
	);

	let signed_tx = tx.signed_tx_from_pair(AccountKeyring::Alice.pair())?;
	println!("tx: {:#?}", signed_tx);

	let tx_encoded = hex::encode(signed_tx.encode());
	println!("tx encoded: {:#?}", tx_encoded);

	Ok(())
}

// TODO the below utils should be moved to another file so they can be shared across examples
/// RPC response JSON object
#[derive(Serialize, Deserialize)]
struct RpcRes<T> {
	jsonrpc: String,
	result: T,
}

/// RPC request JSON object
#[derive(Serialize, Deserialize)]
struct RpcReq<T: Serialize> {
	jsonrpc: String,
	id: usize,
	method: String,
	params: Vec<T>,
}

fn rpc_to_local_node<T: Serialize, U: DeserializeOwned>(
	method: &str,
	params: Vec<T>,
) -> Result<RpcRes<U>, Box<dyn std::error::Error>> {
	let local_node_url = "http://localhost:9933";
	let client = reqwest::blocking::Client::new();

	let req_body = RpcReq { jsonrpc: "2.0".to_owned(), id: 1, method: method.to_owned(), params };
	let res = client.post(local_node_url).json(&req_body).send()?.json()?;

	Ok(res)
}

/// Convert a hex string to `H256`
fn string_to_h256(value: &str) -> H256 {
	// Slice of the "0x" prefix
	let no_prefix = &value[2..];
	let bytes = hex::decode(no_prefix).expect("only valid hex strings are passed in");

	H256::from_slice(bytes.as_slice())
}

/// Read in a scale encoded hex `result` from the response to a RPC call.
///
/// The file expected to contain a JSON object with the form:
///
/// ```no_run
/// {"jsonrpc":"2.0","result":"0xff","id":1}
/// ```
///
/// where `result` is a field representing scale encoded bytes.
#[allow(dead_code)]
fn rpc_to_bytes(path: PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let contents = file_to_string(path)?;

	let rpc_response: RpcRes<String> = serde_json::from_str(&contents)?;
	// remove `0x` from the hex string.
	let hex = &rpc_response.result[2..];
	let bytes = hex::decode(hex)?;

	Ok(bytes)
}

/// Deserialize a struct from the `result` in a JSON response to the
/// RPC `system_properties`. (Relevant structs to deserialize include
/// `SystemProperties` and `RuntimeVersion`.)
///
/// The file expected to contain a JSON object with the form:
///
/// ```no_run
/// {"jsonrpc":"2.0","result":"...","id":1}
/// ```
///
/// where `result` is a field representing a struct in JSON.
#[allow(dead_code)]
fn rpc_to<T: DeserializeOwned>(path: PathBuf) -> Result<T, Box<dyn std::error::Error>> {
	let contents = file_to_string(path)?;

	let rpc_response: RpcRes<T> = serde_json::from_str(&contents)?;

	Ok(rpc_response.result)
}

/// Read a file to a string (non-buffered).
#[allow(dead_code)]
fn file_to_string(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	Ok(contents)
}
