# [`Antelope`](https://antelope.io/) Standard Library for [`Rust`](https://www.rust-lang.org/)

[![Build Status](https://github.com/pinax-network/antelope-rust/actions/workflows/test.yml/badge.svg)](https://github.com/pinax-network/antelope-rust/actions/workflows/test.yml)
![License](https://img.shields.io/github/license/pinax-network/antelope-rust)
![Crates.io](https://img.shields.io/crates/v/antelope)

Implements most commonly used [Antelope C++ Classes](https://github.com/AntelopeIO/cdt/tree/main/libraries/eosiolib/core/eosio) into Rust.

## Planned support for

- [ ] asset
- [ ] symbol
- [x] symbol_code
- [ ] name
- [ ] extended_asset
- [ ] extended_symbol

## Install

```bash
$ cargo add antelope
```

## Quickstart

```rust
use antelope::symbol_code::SymbolCode;

let symcode = SymbolCode::new("FOO");
assert_eq!(5197638, symcode.raw());
assert_eq!(3, symcode.length());
assert_eq!(true, symcode.is_valid());
assert_eq!("FOO", symcode.to_string());
```
