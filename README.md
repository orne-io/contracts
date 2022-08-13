# Contracts

[![Intergration](https://github.com/orne-io/contracts/actions/workflows/ci.yaml/badge.svg)](https://github.com/orne-io/contracts/actions/workflows/ci.yaml)

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

### Instances

| Name                          | Mainnet | Testnet                                                                                                                                                                           |
| ----------------------------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [token](contracts/token/)     | -       | [`terra17lpau...snck5`](https://finder.terra.money/testnet/address/terra17lpau4t55q48g0utuh4cf0mderjkvddv0pdu3lazm6znnp95fq4susnck5) |
| [airdrop](contracts/airdrop/) | -       | [`terra13ktv2...l9760`](https://finder.terra.money/testnet/address/terra13ktv2qjs44k8wjgkgtvseezjewdxfxv9pqsfd0t6qtkhm5w460hsml9760) |

## Packages

### [`orne-periphery`](packages/periphery/)

Contains types (msgs + responses) and helpers for periphery contracts, including:

* `airdrop`
