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
| [airdrop](contracts/airdrop/) | -       | `3428`  |
| [staking](contracts/staking/) | -       | -       |

### Instances

| Name                          | Mainnet | Testnet                                                                                                                              |
| ----------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| [token](contracts/token/)     | -       | [`terra1f4mp7...gqu78`](https://finder.terra.money/testnet/address/terra1f4mp7uxaq2je5c0mrxe4akd984487lxv4nu2lsyw0gvpr6l63yqsngqu78) |
| [airdrop](contracts/airdrop/) | -       | [`terra1gxgxs...jgx3m`](https://finder.terra.money/testnet/address/terra1gxgxsn2kdw8539gq7nzlrcjyp5glp2lnaqkqyn4ur9ttmjufg70qfjgx3m) |
| [staking](contracts/staking/) | -       | -                                                                                                                                    |

## Packages

### [`orne-periphery`](packages/periphery/)

Contains types (msgs + responses) and helpers for periphery contracts, including:

* `airdrop`
* `staking`
