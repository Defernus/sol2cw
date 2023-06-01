# Solidity to cosmwasm translator

### **THIS IS A WORK IN PROGRESS! DO NOT USE IT FOR ANYTHING OTHER THAN TESTING**

## Description

This is just my experiment to see if it's possible to translate Solidity to CosmWasm.
It's not meant to be used for anything other than testing.

## Limitations

// TODO

## How to use

Just run `cargo run -p sol2cw <sol_contract_path>`

for example:

```
git clone https://github.com/samnang/solidity-examples.git
cargo run -p sol2cw gmx-contracts/contracts/getting_started_with_solidity/01_data_types/01_MyContract.sol
```

## How to test

Install cargo-make:

```
cargo install cargo-make
```

And run tests

```
cargo make test
```

## Linting and formatting

You can run clippy and rustfmt with:

```
cargo make clippy
cargo make fmt
cargo make audit
```

Use fix for auto-fixing formatting and linting issues (all changes should be committed):

```
cargo make fix
```
