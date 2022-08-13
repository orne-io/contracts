# merkle-cli

A small util to generate merkle trees and proofs.

## Usage

Install deps

```bash
pnpm i
```

Generate a json file with the airdrop list

Example:

```json
[
    {
        "address": "terra1fmcjjt6yc9wqup2r06urnrd928jhrde6gcld6n",
        "amount": "10"
    },
    {
        "address": "terra1lkccuqgj6sjwjn8gsa9xlklqv4pmrqg9dx2fxc",
        "amount": "2414140"
    },
    {
        "address": "terra1333veey879eeqcff8j3gfcgwt8cfrg9mq20v6f",
        "amount": "1000"
    }
]
```

Get the merkle root

```bash
pnpm start get-root
```

Get the merkle proof for an account

```bash
pnpm start get-proof terra1lkccuqgj6sjwjn8gsa9xlklqv4pmrqg9dx2fxc 2414140
```
