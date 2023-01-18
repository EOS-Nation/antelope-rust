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

**Cargo.toml**

```toml
[dependencies]
antelope = "0.0.1"
```

## Quickstart

```rust
use antelope::{SymbolCode};

SymbolCode::new("FOO").raw();
// => 5197638

SymbolCode::from(5197638).to_string();
// => "FOO"

SymbolCode::new("FOO") != SymbolCode::new("BAR")
// => true

SymbolCode::new("FOO") == SymbolCode::new("BAR")
// => false
```
