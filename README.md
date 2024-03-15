# SPL Token

This repo contains SPL token program, which implements four interfaces:

1. Set: admin can set specified SPL token.
2. AddUser: add user info to track the balance of users.
3. Deposit: users can deposit some tokens to contract.
4. Withdraw: users can withdraw some tokens from contract.

## Installation

- Rust: 1.74.0
- Solana: 1.18.0
- Yarn: 1.22.19
- Anchor: 0.29.0

## Quickstart

Clone the repository and enter the source code directory.

```shell
git clone https://github.com/quarkneu/SPLToken.git
cd SPLToken
```

Build

```shell
anchor build
```

After building, the smart contract files are all located in the target directory.

Deploy

```shell
anchor deploy
```

Attention, check your configuration and confirm the environment you want to deploy.
