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
| [token](contracts/token/)     | `376`   | `2995`  |
| [airdrop](contracts/airdrop/) | `377`   | `3428`  |
| [staking](contracts/staking/) | -       | -       |

### Instances

| Name                          | Mainnet                                                                                                                            | Testnet                                                                                                                              |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| [token](contracts/token/)     | [terra19p20m...fenjw](https://finder.terra.money/mainnet/address/terra19p20mfnvwh9yvyr7aus3a6z6g6uk28fv4jhx9kmnc2m7krg27q2qkfenjw) | [`terra1f4mp7...gqu78`](https://finder.terra.money/testnet/address/terra1f4mp7uxaq2je5c0mrxe4akd984487lxv4nu2lsyw0gvpr6l63yqsngqu78) |
| [airdrop](contracts/airdrop/) | [terra1kgkts...ku3vz](https://finder.terra.money/mainnet/address/terra1kgktsdm3j3hw6ces3fxjs58rft5c7cq9dpsrdg2lrqfdgsrs5cjshku3vz) | [`terra1h3qxl...8nh9m`](https://finder.terra.money/testnet/address/terra1h3qxlz5fzl8hht4z8sxc00hzs8mnd34djle5tl0hclwkkxpx8tasa8nh9m) |
| [staking](contracts/staking/) | -                                                                                                                                  | -                                                                                                                                    |

## Packages

### [`orne-periphery`](packages/periphery/)

Contains types (msgs + responses) and helpers for periphery contracts, including:

* `airdrop`
* `staking`
