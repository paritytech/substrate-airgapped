use codec::Encode;
use hex;
use substrate_airgapped_types::{balances::Transfer, CallIndex, PolkadotRuntime, TxBuilder};

use serde::{Deserialize, Serialize};
use sp_keyring::AccountKeyring;
use std::convert::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let base_path =
		env::current_dir()?.join("substrate-airgapped-types").join("examples").join("submit");
	let path_to_genesis = base_path.join("genesis.json");
	let genesis_hash = rpc_to_bytes(path_to_genesis)?;
	let genesis_hash = sp_core::H256::from_slice(&genesis_hash[..]); // TODO this panics

	type Runtime = PolkadotRuntime;
	type TransferType = Transfer<Runtime>;

	let alice_addr = AccountKeyring::Alice.to_account_id().into();
	let bob_addr = AccountKeyring::Bob.to_account_id().into();
	let transfer_args = Transfer { to: bob_addr, amount: 123_456 };
	let call_index = CallIndex::new(5, 0);

	let tx_builder: TxBuilder<TransferType, Runtime> =
		TxBuilder::new(call_index, transfer_args, alice_addr, 0, 4, 26, genesis_hash);

	let tx = tx_builder.unchecked_from_pair(AccountKeyring::Alice.pair())?;
	println!("tx: {:#?}", tx);

	let tx_encoded = hex::encode(tx.encode());
	println!("tx encoded: {:#?}", tx_encoded);

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
