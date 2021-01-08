use codec::{Encode, Decode};
use core::convert::TryInto;
use substrate_airgapped_metadata::{Metadata, RuntimeMetadataPrefixed};
use sp_keyring::AccountKeyring;
use substrate_airgapped::{GenericCall, balances::Transfer};
use util::rpc_to_local_node;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let metadata_bytes = rpc_to_local_node::<(), String>("chain_getBlockHash", vec![])
		.and_then(|rpc_res| Ok(hex::decode(rpc_res.result)?))?;

	let metadata_prefixed: RuntimeMetadataPrefixed = Decode::decode(&mut &metadata_bytes[..])?;
	let metadata: Metadata = metadata_prefixed.try_into()?;

	let args = Transfer {
		to: AccountKeyring::Bob.to_account_id().into(),
		amount: 123_456_789,
	};
	let call_index = metadata.find_call_index(args)?;
	let transfer_call = GenericCall::new(call_index, args);



	Ok(())
}
