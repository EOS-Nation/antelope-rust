use crate::{check, Symbol};
use std::cmp::{Ord, PartialEq, PartialOrd};
// use std::convert::From;
// use std::fmt::{Display, Formatter, Result};
// use std::ops::Not;

pub const MAX_AMOUNT: i64 = (1 << 62) - 1;

/// The `Asset` struct represents a asset
///
/// Reference: <https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/asset.hpp>
///
/// # Examples
///
/// ```
/// use antelope::{Asset, Symbol};
///
/// let quantity = Asset::from_amount(10000, Symbol::from("4,FOO"));
/// assert_eq!(10000, quantity.amount);
/// ```
#[derive(Eq, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Default)]
pub struct Asset {
    pub amount: i64,
    pub symbol: Symbol,
}

impl Asset {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            amount: 0,
            symbol: Symbol::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn from_amount(amount: i64, symbol: Symbol) -> Self {
        Asset { amount, symbol }
    }

    /**
     * Check if the amount doesn't exceed the max amount
     *
     * @return true - if the amount doesn't exceed the max amount
     * @return false - otherwise
     */
    pub fn is_amount_within_range(&self) -> bool {
        -MAX_AMOUNT <= self.amount && self.amount <= MAX_AMOUNT
    }

    /**
     * Check if the asset is valid. %A valid asset has its amount <= max_amount and its symbol name valid
     *
     * @return true - if the asset is valid
     * @return false - otherwise
     */
    pub fn is_valid(&self) -> bool {
        self.is_amount_within_range() && self.symbol.is_valid()
    }

    /**
     * Set the amount of the asset
     *
     * @param a - New amount for the asset
     */
    pub fn set_amount(mut self, amount: i64) {
        self.amount = amount;
        check(self.is_amount_within_range(), "magnitude of asset amount must be less than 2^62")
    }
}

// impl Display for Asset {
//     #[inline]
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         f.write_str(format!("{}@{}", self.sym, self.contract).as_str())
//     }
// }

// impl From<&str> for Asset {
//     #[inline]
//     #[must_use]
//     fn from(str: &str) -> Self {
//         let parts = str.split('@').collect::<Vec<&str>>();
//         check(parts.len() == 2, "invalid extended symbol format");
//         let sym = Symbol::from(parts[0]);
//         check(sym.is_valid(), "invalid symbol precision");
//         let contract = Name::from(parts[1]);
//         Asset::from_extended(sym, contract)
//     }
// }

// impl AsRef<Asset> for Asset {
//     #[inline]
//     #[must_use]
//     fn as_ref(&self) -> &Asset {
//         self
//     }
// }

// impl Not for Asset {
//     type Output = bool;

//     #[inline]
//     #[must_use]
//     fn not(self) -> bool {
//         self.contract.raw() == 0 && self.sym.raw() == 0
//     }
// }

// impl From<Asset> for bool {
//     #[inline]
//     #[must_use]
//     fn from(ext_sym: Asset) -> Self {
//         ext_sym.contract.raw() != 0 && ext_sym.sym.raw() != 0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdt_1() {
        assert_eq!(Asset::new().symbol.raw(), 0);
        assert_eq!(Asset::new().amount, 0);
    }
}
