# CKB Global Registry

[![License]](#license)
[![GitHub Actions]](https://github.com/yangby-cryptape/ckb-global-registry/actions)

Libraries to build global registry on [CKB].

[License]: https://img.shields.io/badge/License-MIT-blue.svg
[GitHub Actions]: https://github.com/yangby-cryptape/ckb-global-registry/workflows/CI/badge.svg

## Description

Libraries to register data on [CKB] and guarantee no duplicates.

## Background

In a blockchain which uses a UTXO based model, there is no built-in global
state which could be modified parallelly.

For example, if the whole global state is stored in an unspent UTXO, when
two transactions want to update the state at a same time, there will be only
one of them could success.
Because those two transactions are conflicted due to double-spending.
The failed transaction requires updates: at least one of its inputs has to
be replaced with a new out point, and itself has to be re-signed.

This repository provides methods to help users to build a global registry in
their own contracts to store global state in several CKB cells, so that to
modify the global state parallelly.

## Crates

- [CKB Linked List Tool]

  A tool to create a linked list between CKB cells.

## Examples

There is one demo contracts:

- [Global Registry based on Linked List]

  This contract is used to manage a global registry instance and items in it
  based on a linked list.

  It should be used as a type script.

## License

Licensed under [MIT License].

[CKB]: https://github.com/nervosnetwork/ckb

[CKB Linked List Tool]: crates/ckb-linked-list-tool
[Global Registry based on Linked List]: contracts/demo-linked-list-type

[MIT License]: LICENSE
