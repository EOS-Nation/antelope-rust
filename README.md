# [`Antelope`](https://antelope.io/) Standard Library for [`Rust`](https://www.rust-lang.org/)

[<img alt="github" src="https://img.shields.io/badge/Github-antelope.rs-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/pinax-network/antelope.rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/antelope.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/antelope)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-antelope-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/antelope)
[<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/pinax-network/antelope.rs/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/pinax-network/antelope.rs/actions?query=branch%3Amain)

Implements most commonly used [Antelope C++ Classes](https://github.com/AntelopeIO/cdt/tree/main/libraries/eosiolib/core/eosio) into Rust.

## 📖 Documentation

### https://docs.rs/antelope

## 🛠 Feature Roadmap

- [ ] [`symbol`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp)
    - [x] [`symbol_code`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp)
    - [ ] [`extended_symbol`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp)
- [ ] [`asset`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/asset.hpp)
    - [ ] [`extended_asset`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/asset.hpp)
- [ ] [`name`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/name.hpp)
- [x] [`check`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/check.hpp)
- [ ] [`time`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
    - [ ] [`microseconds`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
    - [ ] [`time_point`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
    - [ ] [`time_point_sec`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
    - [ ] [`block_timestamp`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)

## Install

```bash
$ cargo add antelope
```

## Quickstart

```rust
use antelope::{SymbolCode};

let symcode = SymbolCode::from("FOO");
assert_eq!(5197638, symcode.raw());
assert_eq!(3, symcode.length());
assert_eq!(true, symcode.is_valid());
assert_eq!("FOO", symcode.to_string());
```