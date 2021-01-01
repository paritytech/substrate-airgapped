# [WIP] Substrate Airgapped

Tools to facilitate an air-gapped construction, decoding, and signing flow for transactions of `FRAME`-based chains.

This is very much a work in progress is not yet a MVP.

## Crates

- substrate-airgapped-cli: CLI that combines all functionality of the available substrate-airgapped libraries.
- substrate-airgapped-type: Where core components & functionality is being built out


Note: The code here is heavily inspired by [paritytech/substrate-subxt](https://github.com/paritytech/substrate-subxt)

Notes:: notes:

- Hopefully will eventually be able to use: https://github.com/ascjones/chameleon
- Remove any substrate deps for signing portions
- Implement no_std

## Planned workflow

```rust
// --- Online device ---

let air_metadata = AirMetadata::<PolkadotRuntime>::new(
    metadata_hex,
    genesis_hash_hex,
    spec_version,
    tx_version
  );

let PolkadotTransfer = Transfer<PolkadotRuntime>;

// Transfer method with associated call index so we no longer need metadata
let transfer_method_index = air_metadata.create_method_index::<PolkadotTransfer>();

let my_tx_builder:  TxBuilder<PolkadotTransfer> = transfer_method_index::create_tx_builder(
    Transfer {
      to,
      amount,
    },
    nonce,
    tip,
    era_period,
    checkpoint_block_hash,
    checkpoint_block_number
  );

let my_encoded_tx = my_tx.encode()


// --- Airgapped device ---

let my_tx_builder = TxBuilder<PolkadotTransfer>::decode(my_encoded_tx);

let signing_payload = my_tx_builder.create_signing_payload();
// Display method that will decode just the already encoded call so all parts can be viewed.
println!("{:#?}", signing_payload.display());

let signature = signing_payload.sign(my_keys);

let unchecked_extrinsic = my_tx_builder.create_unchecked_extrinsic(signature);
// Display method that will decode just the already encoded call so all parts can be viewed.
println!("{:#?}", unchecked_extrinsic.display());

let encoded_unchecked_extrinsic = unchecked_extrinsic.encode();


// --- Online device ---

let unchecked_extrinsic = my_tx_builder.decode_unchecked_extrinsic(encoded_unchecked_extrinsic);
// Double check things
println!("{:#?}", unchecked_extrinsic.display());

// Broadcast encoded_unchecked_extrinsic

```
