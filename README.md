# [WIP] **Air**-gapped **Sub**strate

Tools to facilitate an air-gapped construction, decoding, and signing flow for transactions of `FRAME`-based chains.

## Crates

- airsub-cli: CLI that combines all functionality of the available airsub libraries.
- airsub-create: Create a Call and additional data in preparation for signing.
- airsub-type: Things Frame type related.
- airsub-sign: Create an `UncheckedExtrinsic` by signing the relevant outputs of airsub-create.
