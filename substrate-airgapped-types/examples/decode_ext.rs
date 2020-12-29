use substrate_airgapped_types::{
	UncheckedExtrinsic,
	frame::balances::TransferCall,
	PolkadotRuntime
};
use codec::{Decode, Input};

fn main() -> () {
	let balances_transfer_alice = b"0x450284d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0104478ce991cc9766dfba2bf3fe102f81afea590d86a29cfbb52c3ee0bf5e535f36904071af4313061b232f49f5819ae453a7144deaade2f2894329b6649637828500000005008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4813f6ffffffffff3f01";

	type Ext = UncheckedExtrinsic<PolkadotRuntime, TransferCall<PolkadotRuntime>>;

	let result: Ext = Decode::decode(&mut &balances_transfer_alice[..]).unwrap();

	println!("Decoded ext: {:#?}", result);

    ()
}