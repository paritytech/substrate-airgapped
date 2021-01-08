//! Various utilities used in examples
#![warn(missing_docs)]

use reqwest::{self};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::DeserializeOwned;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

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

/// Convert a hex string to `H256`.
pub fn string_to_h256(value: &str) -> H256 {
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
pub fn rpc_to_bytes(path: PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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
pub fn rpc_to<T: DeserializeOwned>(path: PathBuf) -> Result<T, Box<dyn std::error::Error>> {
	let contents = file_to_string(path)?;

	let rpc_response: RpcRes<T> = serde_json::from_str(&contents)?;

	Ok(rpc_response.result)
}

/// Read a file to a string (non-buffered).
fn file_to_string(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	Ok(contents)
}
