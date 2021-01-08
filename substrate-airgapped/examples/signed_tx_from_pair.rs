use codec::Encode;
use sp_core::H256;
use sp_runtime::{generic::Header, traits::BlakeTwo256, DeserializeOwned};
use substrate_airgapped::{
	balances::Transfer, CallIndex, GenericCall, KusamaRuntime, MortalConfig, Mortality, Tx,
	TxConfig,
};

// Deps not used in substrate-airgapped
use hex;
use serde::{Deserialize, Serialize};
use sp_keyring::AccountKeyring;
use sp_version::RuntimeVersion;
use std::convert::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Get the latest block hash and then make all non historic queries at that block.
	let block_hash = rpc_to_local_node::<(), String>("chain_getBlockHash", vec![])?.result;
	// Get the current runtime version.
	let runtime_version = rpc_to_local_node::<String, RuntimeVersion>(
		"chain_getRuntimeVersion",
		vec![block_hash.clone()],
	)?
	.result;
	// Fetch the header of the latest block so we can get the block number.
	let header = rpc_to_local_node::<String, Header<u32, BlakeTwo256>>(
		"chain_getHeader",
		vec![block_hash.clone()],
	)?
	.result;
	let genesis_hash = rpc_to_local_node::<usize, String>("chain_getBlockHash", vec![0])
		.and_then(|rpc_res| Ok(string_to_h256(&rpc_res.result)))?;

	// The type for the Transfer call arguments struct
	type TransferType = Transfer<KusamaRuntime>;

	let alice_addr = AccountKeyring::Alice.to_account_id().into();
	let bob_addr = AccountKeyring::Bob.to_account_id().into();
	let transfer_call = GenericCall {
		call_index: CallIndex::new(5, 0),
		args: Transfer { to: bob_addr, amount: 123_456 },
	};

	let tx: Tx<TransferType, KusamaRuntime> = Tx::new(TxConfig {
		call: transfer_call,
		address: alice_addr,
		nonce: 0,
		tx_version: runtime_version.transaction_version,
		spec_version: runtime_version.spec_version,
		genesis_hash,
		mortality: Mortality::Mortal(MortalConfig {
			period: 64,
			checkpoint_block_number: header.number as u64,
			checkpoint_block_hash: string_to_h256(&block_hash[..]),
		}),
		tip: 100,
	});

	let signed_tx = tx.signed_tx_from_pair(AccountKeyring::Alice.pair())?;
	println!("Tx (UncheckedExtrinsic): {:#?}\n", signed_tx);

	let tx_encoded = hex::encode(signed_tx.encode());
	println!("Submit this: {:#?}", tx_encoded);

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
pub fn rpc_to_local_node<T: Serialize, U: Serialize>(
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
