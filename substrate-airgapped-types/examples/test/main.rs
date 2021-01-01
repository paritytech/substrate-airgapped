use sp_keyring::AccountKeyring;
use std::env;
use substrate_airgapped_types::{
	frame::{balances::TransferArgs, CallMethod},
	PolkadotRuntime,
	extrinsic_builder::ExtrinsicClient,
};

use hex;
use serde::{Deserialize, Serialize};
use std::convert::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let dest = AccountKeyring::Bob.to_account_id().into();

	let transfer = TransferArgs::<PolkadotRuntime> { to: dest, amount: 1_000_000_000 };

	let base_path = env::current_dir()?
		.join("substrate-airgapped-types")
		.join("examples")
		.join("test");

	let path_to_metadata = base_path
		.clone()
		.join("metadata.json");

	let path_to_genesis = base_path.join("genesis.json");

	let hex_meta = rpc_to_hex(path_to_metadata)?;
	let genesis_hash = rpc_to_bytes(path_to_genesis)?;
	// TODO this panics
	let genesis_hash = sp_core::H256::from_slice(&genesis_hash[..]);

	let client: ExtrinsicClient<TransferArgs<PolkadotRuntime>, PolkadotRuntime>
		= ExtrinsicClient::new(&hex_meta[..], 26, 4, genesis_hash)?;

	let encoded = client.encode_call(transfer).unwrap();
	println!("Encoded is {:#?}", encoded);

	let decoded = client.decode_call(encoded);
	println!("Decode is {:#?}", decoded)

	// let metadata = rpc_to_bytes(path_to_metadata)?;
	// let metadata_prefixed: RuntimeMetadataPrefixed = Decode::decode(&mut &opts.metadata[..])?;
	// let metadata: Metadata = metadata_prefixed.try_into()?;

	// metadata.encode_call(transfer);


	Ok(())
}

/// The shape of an RPC JSON response object
#[derive(Serialize, Deserialize)]
struct RpcRes<T> {
	jsonrpc: String,
	result: T,
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
pub fn rpc_to_hex(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
	let contents = file_to_string(path)?;

	let rpc_response: RpcRes<String> = serde_json::from_str(&contents)?;
	// remove `0x` from the hex string.
	let hex = &rpc_response.result[2..];

	Ok(hex.to_string())
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

/// Read a file to a string (non-buffered).
fn file_to_string(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	Ok(contents)
}
