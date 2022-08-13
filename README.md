# Contracts

## Tasks

Tasks are run through [`cargo-make`](https://github.com/sagiegurari/cargo-make).

* `wasm` - build wasm
* `wasm-opti` - build optimized wasm release artifacts (⚠️ requires docker)

## Deployments

### Stored code

| Name                          | Mainnet | Testnet |
| ----------------------------- | ------- | ------- |
| [token](contracts/token/)     | -       | `2995`  |
| [airdrop](contracts/airdrop/) | -       | `2996`  |

## Packages

### [`orne-periphery`](packages/periphery/)

Contains types (msgs + responses) and helpers for periphery contracts, including:

* `airdrop`
