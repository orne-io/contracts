# Airdrop

Airdrop contract for orne relaunch on Terra 2.

Library and tests are based on [Mars Protocol's](https://github.com/mars-protocol) codebase [airdrop contract](https://github.com/mars-protocol/mars-periphery/tree/main/contracts/airdrop).

## Interface

- [Instantiate](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/airdrop.rs#L10-L17)
- [Execute](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/airdrop.rs#L21-L43)
- [Query](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/airdrop.rs#L47-L52)
- [Response](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/airdrop.rs#L58-L86)

## Merkle tree

See [merkle-cli](./merkle-cli/).

### `airdrop.json`

* [Testnet + Mainnet](https://github.com/orne-io/contracts/files/9331613/airdrop.zip)

## Instantiate

Given a code id:

```json
{
    "owner": "terra1...",
    "token_address": "terra1...",
    "merkle_roots": ["1234"],
    "from_timestamp": 1234,
    "to_timestamp": 1234,
    "airdrop_size": 1234,
}
```

## Lifecycle

### Pre-airdrop

Claims are disabled, Config updates are enabled, Unclaimed tokens transfers are disabled.

### Airdrop

Claims are enabled, Config updates are enabled (`to_timestamp` can only be increased), Unclaimed tokens transfers are disabled.

### Post-airdrop

Claims are disabled, Config updates are enabled, Unclaimed tokens transfers are enabled.
