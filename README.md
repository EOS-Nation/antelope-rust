# [`Antelope`](https://antelope.io/) Standard Library for [`Rust`](https://www.rust-lang.org/)

[![Build Status](https://github.com/pinax-network/antelope.rs/actions/workflows/test.yml/badge.svg)](https://github.com/pinax-network/antelope.rs/actions/workflows/test.yml)
![License](https://img.shields.io/github/license/pinax-network/antelope.rs)
![Crates.io](https://img.shields.io/crates/v/antelope)

Implements most commonly used [Antelope C++ Classes](https://github.com/AntelopeIO/cdt/tree/main/libraries/eosiolib/core/eosio) into Rust.

## Feature Roadmap

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
use antelope::symbol_code::{SymbolCode};

let symcode = SymbolCode::new("FOO");
assert_eq!(5197638, symcode.raw());
assert_eq!(3, symcode.length());
assert_eq!(true, symcode.is_valid());
assert_eq!("FOO", symcode.to_string());
```
