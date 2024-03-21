//! [![github]](https://github.com/pinax-network/antelope.rs)&ensp;[![crates-io]](https://crates.io/crates/antelope)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! Implements most commonly used [Antelope C++ Classes](https://github.com/AntelopeIO/cdt/tree/main/libraries/eosiolib/core/eosio) into Rust.
//!
//! ## 🛠 Feature Roadmap
//!
//! - [x] [`symbol`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp)
//!     - [x] [`symbol_code`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp)
//!     - [x] [`extended_symbol`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp)
//! - [x] [`asset`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/asset.hpp)
//!     - [x] [`extended_asset`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/asset.hpp)
//! - [x] [`name`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/name.hpp)
//! - [x] [`check`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/check.hpp)
//! - [ ] [`time`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
//!     - [x] [`microseconds`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
//!     - [x] [`time_point`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
//!     - [x] [`time_point_sec`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)
//!     - [ ] [`block_timestamp`](https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/time.hpp)

/// Modules for Asserts type.
pub mod check;
pub use self::check::*;

/// Error types.
pub mod errors;
pub use self::errors::*;

/// Modules for Symbol Code type.
pub mod symbol_code;
pub use self::symbol_code::*;

/// Modules for Symbol type.
pub mod symbol;
pub use self::symbol::*;

/// Modules for Extended Symbol type.
pub mod extended_symbol;
pub use self::extended_symbol::*;

/// Modules for Name type.
pub mod name;
pub use self::name::*;

/// Modules for Asset type.
pub mod asset;
pub use self::asset::*;

/// Modules for Asset type.
pub mod extended_asset;
pub use self::extended_asset::*;

/// Modules for Microseconds type.
pub mod microseconds;
pub use self::microseconds::*;

/// Modules for TimePoint type.
pub mod time_point;
pub use self::time_point::*;

/// Modules for TimePoint type.
pub mod time_point_sec;
pub use self::time_point_sec::*;
