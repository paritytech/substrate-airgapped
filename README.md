# [WIP] Substrate Airgapped

Tools to facilitate an air-gapped construction, decoding, and signing flow for transactions of `FRAME`-based chains.

## Crates

- substrate-airgapped-cli: CLI that combines all functionality of the available substrate-airgapped libraries.
- substrate-airgapped-create: Create a Call and additional data in preparation for signing.
- substrate-airgapped-type: Things FRAME type related.
- substrate-airgapped-sign: Create an `UncheckedExtrinsic` by signing the relevant outputs of substrate-airgapped-create.

Note: The code here is heavily inspired by [paritytech/substrate-subxt](https://github.com/paritytech/substrate-subxt)

Hopefully will eventually be able to use: https://github.com/ascjones/chameleon