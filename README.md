# `Antelope` Rust library

[![Build Status](https://github.com/EOS-Nation/antelope-rust/actions/workflows/test.yml/badge.svg)](https://github.com/EOS-Nation/antelope-rust/actions/workflows/test.yml)
![License](https://img.shields.io/github/license/EOS-Nation/antelope-rust)

> Antelope common library. Implements most commonly used Antelop (EOSIO) C++ Classes into Rust.

- [ ] asset
- [ ] symbol
- [x] symbol_code
- [ ] name
- [ ] extended_asset
- [ ] extended_symbol

## Install

**Cargo.toml**

```toml
[dependencies]
antelope = "0.0.1"
```

## Quickstart

```rust
use antelope::{SymbolCode};

let symcode = SymbolCode::from("EOS".to_string());
let raw: u64 = symcode.raw();
// => 5459781
```