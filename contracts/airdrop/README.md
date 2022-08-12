# Airdrop

Airdrop contract for orne relaunch on Terra 2.

Library and tests are based on [Mars Protocol's](https://github.com/mars-protocol) codebase [airdrop contract](https://github.com/mars-protocol/mars-periphery/tree/main/contracts/airdrop).

## Interface

- [Instantiate](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/airdrop.rs#L10-L17)
- [Execute](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/airdrop.rs#L21-L43)
- [Query](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/airdrop.rs#L47-L52)
- [Response](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/airdrop.rs#L58-L86)

## Merkle tree

### 1. Create a json distribution list

Sample format:

```json
{
    [
        {
            "address": "terra1k0jntykt7e4g3y88ltc60czgjuqdy4c9ax8tx2",
            "amount": "43454523323"
        },
        {
            "address": "terra1xzlgeyuuyqje79ma6vllregprkmgwgavjx2h6m",
            "amount": "1343252443"
        }
    ]
}
```

Use it as imput for script:

> TODO

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
