# [WIP] Substrate Airgapped

Tools to facilitate an air-gapped construction, decoding, and signing flow for transactions of `FRAME`-based chains.

## Crates

- substrate-airgapped-cli: CLI that combines all functionality of the available substrate-airgapped libraries.
- substrate-airgapped: Where core components & functionality is being built out.
- substrate-metadata: A wrapper around runtime metadata that can be used to programmatically get the
call index of transaction.

See [substrate-airgapped/examples/signed_tx_from_pair](substrate-airgapped/examples/signed_tx_from_pair) for a working example. More examples coming soon for alternate flows.


