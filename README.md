# `Antelope` Rust library

[![Build Status](https://github.com/EOS-Nation/antelope-rust/actions/workflows/test.yml/badge.svg)](https://github.com/EOS-Nation/antelope-rust/actions/workflows/test.yml)
![License](https://img.shields.io/github/license/EOS-Nation/antelope-rust)
![Crates.io](https://img.shields.io/crates/v/antelope)

> Antelope common library.

Implements most commonly used Antelop (EOSIO) C++ Classes into Rust.

## Support for

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