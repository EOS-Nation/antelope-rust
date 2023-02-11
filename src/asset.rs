use crate::{check, Symbol};
use std::cmp::{Ord, PartialEq, PartialOrd};
// use std::convert::From;
// use std::fmt::{Display, Formatter, Result};

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
    pub const MAX_AMOUNT: i64 = (1 << 62) - 1;

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
        -Asset::MAX_AMOUNT <= self.amount && self.amount <= Asset::MAX_AMOUNT
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

impl AsRef<Asset> for Asset {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &Asset {
        self
    }
}

impl std::ops::Neg for Asset {
    type Output = Asset;
    /**
     * Negate the amount of the asset
     *
     * @return a new asset with the negated amount
     */
    fn neg(self) -> Asset {
        Asset {
            amount: -self.amount,
            symbol: self.symbol,
        }
    }
}

impl std::ops::SubAssign for Asset {
    /**
     * Subtraction assignment operator
     *
     * @param other - Another asset to subtract this asset with
     * @post The amount of this asset is subtracted by the amount of asset `other`
     */
    fn sub_assign(&mut self, other: Asset) {
        assert_eq!(self.symbol, other.symbol, "attempt to subtract asset with different symbol");
        self.amount -= other.amount;
        check(-Asset::MAX_AMOUNT <= self.amount, "subtraction underflow");
        check(self.amount <= Asset::MAX_AMOUNT, "subtraction overflow");
    }
}

impl std::ops::AddAssign for Asset {
    /**
     * Addition Assignment  operator
     *
     * @param a - Another asset to add with this asset
     * @post The amount of this asset is added with the amount of asset a
     */
    fn add_assign(&mut self, a: Self) {
        assert_eq!(self.symbol, a.symbol, "attempt to add asset with different symbol");
        self.amount += a.amount;
        assert!(-Self::MAX_AMOUNT <= self.amount, "addition underflow");
        assert!(self.amount <= Self::MAX_AMOUNT, "addition overflow");
    }
}

impl std::ops::MulAssign<i64> for Asset {
    /**
     * Multiplication assignment operator, with a number
     *
     * @details Multiplication assignment operator. Multiply the amount of this asset with a number and then assign the value to itself.
     * @param a - The multiplier for the asset's amount
     * @return asset - Reference to this asset
     * @post The amount of this asset is multiplied by a
     */
    fn mul_assign(&mut self, a: i64) {
        let tmp = (self.amount as i128) * (a as i128);
        assert!(tmp <= Self::MAX_AMOUNT as i128, "multiplication overflow");
        assert!(tmp >= -(Self::MAX_AMOUNT as i128), "multiplication underflow");
        self.amount = tmp as i64;
    }
}

impl std::ops::Add for Asset {
    type Output = Self;

    /**
     * Addition operator
     *
     * @param other - The second asset to be added to the first asset
     * @return asset - New asset as the result of addition
     */
    fn add(self, other: Self) -> Self {
        let mut result = self;
        result += other;
        result
    }
}

impl std::ops::Sub for Asset {
    type Output = Self;

    /**
     * Subtraction operator
     *
     * @param other - The asset used to subtract from the first asset
     * @return asset - New asset as the result of subtraction
     */
    fn sub(self, other: Self) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdt_1() {
        assert_eq!(Asset::new().symbol.raw(), 0);
        assert_eq!(Asset::new().amount, 0);
    }

    #[test]
    fn test_asset_creation() {
        let asset = Asset {
            amount: 1000,
            symbol: Symbol::from("4,SYS"),
        };
        assert_eq!(asset.amount, 1000);
        assert_eq!(asset.symbol, Symbol::from("4,SYS"));
    }

    #[test]
    fn test_asset_equality() {
        let asset1 = Asset {
            amount: 1000,
            symbol: Symbol::from("4,SYS"),
        };
        let asset2 = Asset {
            amount: 1000,
            symbol: Symbol::from("4,SYS"),
        };
        assert_eq!(asset1, asset2);
    }

    #[test]
    fn test_asset_inequality() {
        let asset1 = Asset {
            amount: 1000,
            symbol: Symbol::from("4,SYS"),
        };
        let asset2 = Asset {
            amount: 1000,
            symbol: Symbol::from("5,SYS"),
        };
        let asset3 = Asset {
            amount: 1001,
            symbol: Symbol::from("4,SYS"),
        };
        assert_ne!(asset1, asset2);
        assert_ne!(asset1, asset3);
    }

    #[test]
    fn test_neg() {
        let asset = Asset::from_amount(100, Symbol::new());
        let negated_asset = -asset;
        assert_eq!(negated_asset.amount, -100);
    }

    #[test]
    fn test_sub_assign() {
        let mut asset1 = Asset {
            amount: 100,
            symbol: Symbol::new(),
        };
        let asset2 = Asset {
            amount: 50,
            symbol: Symbol::new(),
        };

        asset1 -= asset2;

        assert_eq!(asset1.amount, 50);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract asset with different symbol")]
    fn test_sub_assign_with_different_symcode() {
        let mut asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };
        let asset2 = Asset {
            amount: 50,
            symbol: Symbol::from("4,TST"),
        };

        asset1 -= asset2;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract asset with different symbol")]
    fn test_sub_assign_with_different_precision() {
        let mut asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };
        let asset2 = Asset {
            amount: 50,
            symbol: Symbol::from("5,SYM"),
        };

        asset1 -= asset2;
    }

    #[test]
    #[should_panic(expected = "subtraction underflow")]
    fn test_sub_assign_overflow() {
        let mut asset1 = Asset {
            amount: -Asset::MAX_AMOUNT,
            symbol: Symbol::new(),
        };
        let asset2 = Asset {
            amount: 1,
            symbol: Symbol::new(),
        };

        asset1 -= asset2;
    }

    #[test]
    #[should_panic(expected = "attempt to add asset with different symbol")]
    fn test_add_assign_with_different_symcode() {
        let mut asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };
        let asset2 = Asset {
            amount: 50,
            symbol: Symbol::from("4,TST"),
        };

        asset1 += asset2;
    }

    #[test]
    #[should_panic(expected = "attempt to add asset with different symbol")]
    fn test_add_assign_with_different_precision() {
        let mut asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };
        let asset2 = Asset {
            amount: 50,
            symbol: Symbol::from("5,SYM"),
        };

        asset1 += asset2;
    }

    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_assign_overflow() {
        let mut asset1 = Asset {
            amount: Asset::MAX_AMOUNT,
            symbol: Symbol::new(),
        };
        let asset2 = Asset {
            amount: 1,
            symbol: Symbol::new(),
        };

        asset1 += asset2;
    }

    #[test]
    fn test_asset_addition() {
        let asset_a = Asset {
            symbol: Symbol::from("4,SYS"),
            amount: 1000,
        };
        let asset_b = Asset {
            symbol: Symbol::from("4,SYS"),
            amount: 2000,
        };

        let result = asset_a + asset_b;
        assert_eq!(result.symbol, Symbol::from("4,SYS"));
        assert_eq!(result.amount, 3000);
    }

    #[test]
    fn test_asset_subtraction() {
        let asset_a = Asset {
            symbol: Symbol::from("4,SYS"),
            amount: 3000,
        };
        let asset_b = Asset {
            symbol: Symbol::from("4,SYS"),
            amount: 2000,
        };

        let result = asset_a - asset_b;
        assert_eq!(result.symbol, Symbol::from("4,SYS"));
        assert_eq!(result.amount, 1000);
    }

    #[test]
    fn test_mul_assign() {
        let mut asset = Asset {
            symbol: Symbol::from("4,SYS"),
            amount: 10,
        };
        asset *= 2;
        assert_eq!(asset.amount, 20);
        asset *= 3;
        assert_eq!(asset.amount, 60);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_assign_overflow() {
        let mut asset1 = Asset {
            amount: Asset::MAX_AMOUNT,
            symbol: Symbol::from("4,SYM"),
        };
        asset1 *= 2;
    }

    #[test]
    #[should_panic(expected = "multiplication underflow")]
    fn test_mul_assign_underflow() {
        let mut asset1 = Asset {
            amount: Asset::MAX_AMOUNT,
            symbol: Symbol::from("4,SYM"),
        };
        asset1 *= -2;
    }
}
