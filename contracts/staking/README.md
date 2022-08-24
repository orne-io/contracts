# Staking

This is a port of Anchor Protocol [staking contract](https://docs.anchorprotocol.com/smart-contracts/anchor-token/staking).

**Disclaimer:** The contract sturcture has been a bit changed and actualized for current cosmwasm version but logic is ported from Anchor Protocol's [staking contract](https://docs.anchorprotocol.com/smart-contracts/anchor-token/staking).

## Interface

- [Instantiate](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/staking.rs#L10-L14)
- [Execute](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/staking.rs#L18-28)
- [Cw20 Hook](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/staking.rs#L32-35)
- [Query](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/staking.rs#L39-43)
- [Response](https://github.com/orne-io/contracts/blob/main/packages/periphery/src/staking.rs#L49-74)

## Instantiate

Given a code id:

```json
{
    "token": "terra1...",  // the original cw20 token
    "lp_token": "terra1...", // the stakable token address
    "distribution_schedule": [(1200, 1205, 100)], // array of tuple (start_block, end_block, uamount)
}
```
