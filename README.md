# [WIP] Substrate Airgapped

Tools to facilitate an air-gapped construction, decoding, and signing flow for transactions of `FRAME`-based chains.

## Crates

- substrate-airgapped-cli: CLI that combines all functionality of the available substrate-airgapped libraries.
- substrate-airgapped: Where core components & functionality is being built out.
- substrate-metadata: A wrapper around runtime metadata that can be used to programmatically get the
call index of transaction.

## Examples

- [substrate-airgapped/examples/signed_tx_from_pair.rs](substrate-airgapped/examples/signed_tx_from_pair.rs): Construct a balance transfer, hard-coding the call index.
- [substrate-airgapped/examples/call_index_from_metadata.rs](substrate-airgapped/examples/call_index_from_metadata.rs): Dynamically get the call index from metadata.

## Questions?

Please file an issue for any questions, feature requests, or additional examples