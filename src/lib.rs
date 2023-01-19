//! [![github]](https://github.com/pinax-network/antelope.rs)&ensp;[![crates-io]](https://crates.io/crates/antelope)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! Implements most commonly used [Antelope C++ Classes](https://github.com/AntelopeIO/cdt/tree/main/libraries/eosiolib/core/eosio) into Rust.
//!
//! ## Feature Roadmap
//!
//! - [ ] [`symbol`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp)
//!     - [x] [`symbol_code`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp)
//!     - [ ] [`extended_symbol`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp)
//! - [ ] [`asset`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/asset.hpp)
//!     - [ ] [`extended_asset`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/asset.hpp)
//! - [ ] [`name`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/name.hpp)
//! - [x] [`check`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/check.hpp)
//! - [ ] [`time`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
//!     - [ ] [`microseconds`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
//!     - [ ] [`time_point`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
//!     - [ ] [`time_point_sec`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
//!     - [ ] [`block_timestamp`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)

/// Modules for Asserts related types.
pub mod check;

/// Modules for Symbols related types.
pub mod symbol;
