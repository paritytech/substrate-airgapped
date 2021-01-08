use codec::Encode;
use sp_runtime::{generic::Header, traits::BlakeTwo256, DeserializeOwned};
use substrate_airgapped::{
	balances::Transfer, CallIndex, GenericCall, KusamaRuntime, MortalConfig, Mortality, Tx,
	TxConfig,
};

// Example deps
use hex;
use sp_keyring::AccountKeyring;
use sp_version::RuntimeVersion;
use std::convert::*;
use util::{rpc_to_local_node, string_to_h256};

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
