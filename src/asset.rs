use crate::{check, Symbol, SymbolCode};
// use std::convert::From;
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
#[derive(Eq, Copy, Clone, Debug, Default)]
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

impl std::fmt::Display for Asset {
    /**
     * Converts the asset into string
     *
     * @return String in the form of "1.2345 SYM" format, where SYM symbol has precision equal to 4
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let precision = self.symbol.precision();
        let decimals = 10_i64.pow(precision.into());
        let int_part = self.amount / decimals;
        let dec_part = (self.amount % decimals).abs();

        if precision == 0 {
            write!(f, "{} {}", int_part, self.symbol.code())
        } else {
            write!(f, "{}.{:0pad$} {}", int_part, dec_part, self.symbol.code(), pad = precision.into())
        }
    }
}

impl From<&str> for Asset {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        check(parts.len() == 2, &format!("invalid asset: {s}"));
        let (amount_str, symbol_str) = (parts[0], parts[1]);
        let precision = match amount_str.find('.') {
            Some(idx) => (amount_str.len() - idx - 1) as u8,
            None => 0,
        };
        let amount = match amount_str.replace('.', "").parse::<i64>() {
            Ok(amount) => amount,
            Err(_) => panic!("invalid asset: {s}"),
        };
        let symbol = Symbol::from_precision(SymbolCode::from(symbol_str), precision);

        Asset { amount, symbol }
    }
}

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

impl std::cmp::PartialEq for Asset {
    fn eq(&self, other: &Asset) -> bool {
        check(
            self.symbol == other.symbol,
            "comparison of assets with different symbols is not allowed",
        );
        self.amount == other.amount
    }
}

impl std::cmp::PartialOrd for Asset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        check(
            self.symbol == other.symbol,
            "comparison of assets with different symbols is not allowed",
        );

        self.amount.partial_cmp(&other.amount)
    }
}

impl std::cmp::Ord for Asset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        check(
            self.symbol == other.symbol,
            "comparison of assets with different symbols is not allowed",
        );

        self.amount.cmp(&other.amount)
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

impl std::ops::DivAssign<i64> for Asset {
    /**
     * Division assignment operator, with a number proceeding
     *
     * @brief Division assignment operator, with a number proceeding
     * @param self - The asset to be divided
     * @param a - The divisor for the asset's amount
     * @return asset - Reference to the asset, which has been divided
     */
    fn div_assign(&mut self, a: i64) {
        check(a != 0, "divide by zero");
        check(!(self.amount == std::i64::MIN && a == -1), "signed division overflow");
        self.amount /= a;
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

impl std::ops::Mul<i64> for Asset {
    type Output = Asset;

    /**
     * Multiplication operator, with a number proceeding
     *
     * @brief Multiplication operator, with a number proceeding
     * @param a - The asset to be multiplied
     * @param b - The multiplier for the asset's amount
     * @return asset - New asset as the result of multiplication
     */
    fn mul(self, b: i64) -> Asset {
        let mut result = self;
        result *= b;
        result
    }
}

impl std::ops::Mul<Asset> for i64 {
    type Output = Asset;

    /**
     * Multiplication operator, with a number preceeding
     *
     * @param a - The multiplier for the asset's amount
     * @param b - The asset to be multiplied
     * @return asset - New asset as the result of multiplication
     */
    fn mul(self, a: Asset) -> Asset {
        a * self
    }
}

impl std::ops::Div<i64> for Asset {
    type Output = Asset;

    /**
     * Division operator, with a number proceeding
     *
     * @param a - The asset to be divided
     * @param b - The divisor for the asset's amount
     * @return asset - New asset as the result of division
     */
    fn div(self, b: i64) -> Asset {
        let mut result = self;
        result /= b;
        result
    }
}

impl std::ops::Div<Asset> for Asset {
    type Output = i64;

    /**
     * Division operator, with another asset
     *
     * @param a - The asset which amount acts as the dividend
     * @param b - The asset which amount acts as the divisor
     * @return int64_t - the resulted amount after the division
     * @pre Both asset must have the same symbol
     */
    fn div(self, b: Asset) -> Self::Output {
        assert_ne!(b.amount, 0, "divide by zero");
        assert_eq!(self.symbol, b.symbol, "attempt to divide assets with different symbol");
        self.amount / b.amount
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
    #[should_panic(expected = "comparison of assets with different symbols is not allowed")]
    fn test_equality_operator_panics() {
        let asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let asset2 = Asset {
            amount: 100,
            symbol: Symbol::from("5,SYM"),
        };

        let _ = asset1 == asset2;
    }

    #[test]
    fn test_inequality_operator() {
        let asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let asset2 = Asset {
            amount: 200,
            symbol: Symbol::from("4,SYM"),
        };

        assert_ne!(asset1, asset2);
    }

    #[test]
    #[should_panic(expected = "comparison of assets with different symbols is not allowed")]
    fn test_inequality_operator_panics() {
        let asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let asset2 = Asset {
            amount: 100,
            symbol: Symbol::from("5,SYM"),
        };

        let _ = asset1 != asset2;
    }

    #[test]
    fn test_ord_operator() {
        let asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let asset2 = Asset {
            amount: 200,
            symbol: Symbol::from("4,SYM"),
        };

        let asset3 = Asset {
            amount: 200,
            symbol: Symbol::from("4,SYM"),
        };

        assert!(asset1 < asset2);
        assert!(asset2 > asset1);
        assert!(asset1 <= asset2);
        assert!(asset2 >= asset1);
        assert!(asset3 >= asset2);
        assert!(asset3 <= asset2);
    }

    #[test]
    #[should_panic(expected = "comparison of assets with different symbols is not allowed")]
    fn test_ord_operator_panics() {
        let asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let asset2 = Asset {
            amount: 100,
            symbol: Symbol::from("5,SYM"),
        };

        let _ = asset1 > asset2;
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

    #[test]
    fn test_multiplication_operator() {
        let asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        // Test positive multiplier
        let asset2 = asset1 * 5;
        assert_eq!(asset2.amount, 500);
        assert_eq!(asset2.symbol, Symbol::from("4,SYM"));

        // Test negative multiplier
        let asset3 = -5 * asset1;
        assert_eq!(asset3.amount, -500);
        assert_eq!(asset3.symbol, Symbol::from("4,SYM"));
    }

    #[test]
    fn test_div_assign() {
        let mut asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        asset1 /= 2;
        assert_eq!(asset1.amount, 50);

        asset1 /= -5;
        assert_eq!(asset1.amount, -10);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_asset_divide_by_zero() {
        let mut asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        asset1 /= 0;
    }

    #[test]
    #[should_panic(expected = "signed division overflow")]
    fn test_asset_signed_division_overflow() {
        let mut asset1 = Asset {
            amount: std::i64::MIN,
            symbol: Symbol::from("4,SYM"),
        };

        asset1 /= -1;
    }

    #[test]
    fn test_divide_operator() {
        let asset = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let result = asset / 2;
        assert_eq!(result.amount, 50);
        assert_eq!(result.symbol, Symbol::from("4,SYM"));
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_divide_by_zero() {
        let asset = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let _ = asset / 0;
    }

    #[test]
    #[should_panic(expected = "signed division overflow")]
    fn test_signed_division_overflow() {
        let asset = Asset {
            amount: std::i64::MIN,
            symbol: Symbol::from("4,SYM"),
        };

        let _ = asset / -1;
    }

    #[test]
    fn test_asset_divide_asset_operator() {
        let asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let asset2 = Asset {
            amount: 50,
            symbol: Symbol::from("4,SYM"),
        };

        let result = asset1 / asset2;
        assert_eq!(result, 2);
    }

    #[test]
    #[should_panic(expected = "attempt to divide assets with different symbol")]
    fn test_asset_divide_asset_operator_different_symbols() {
        let asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let asset2 = Asset {
            amount: 50,
            symbol: Symbol::from("5,SYM"),
        };

        let _ = asset1 / asset2;
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_asset_divide_asset_operator_divide_by_zero() {
        let asset1 = Asset {
            amount: 100,
            symbol: Symbol::from("4,SYM"),
        };

        let asset2 = Asset {
            amount: 0,
            symbol: Symbol::from("4,SYM"),
        };

        let _ = asset1 / asset2;
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Asset::from_amount(10000, Symbol::from("4,SYM")).to_string(), "1.0000 SYM");
        assert_eq!(Asset::from_amount(0, Symbol::from("4,SYM")).to_string(), "0.0000 SYM");
        assert_eq!(Asset::from_amount(12345, Symbol::from("2,SYM")).to_string(), "123.45 SYM");
        assert_eq!(Asset::from_amount(100, Symbol::from("0,SYM")).to_string(), "100 SYM");
        assert_eq!(Asset::from_amount(-1000001, Symbol::from("4,SYM")).to_string(), "-100.0001 SYM");
        assert_eq!(Asset::from_amount(0, Symbol::from("0,SYM")).to_string(), "0 SYM");
        assert_eq!(Asset::from_amount(-100, Symbol::from("0,SYM")).to_string(), "-100 SYM");
        assert_eq!(
            Asset::from_amount(0, Symbol::from("18,SYMBOLL")).to_string(),
            "0.000000000000000000 SYMBOLL"
        );
        assert_eq!(
            Asset::from_amount(-1000000000000000000, Symbol::from("18,SYMBOLL")).to_string(),
            "-1.000000000000000000 SYMBOLL"
        );
    }

    #[test]
    fn test_display() {
        println!("{}", Asset::from_amount(10000, Symbol::from("4,SYM")))
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Asset::from_amount(10000, Symbol::from("4,SYM")), Asset::from("1.0000 SYM"));
        assert_eq!(Asset::from_amount(100, Symbol::from("0,SYM")), Asset::from("100 SYM"));
        assert_eq!(Asset::from_amount(12345, Symbol::from("2,SYM")), Asset::from("123.45 SYM"));
        assert_eq!(Asset::from_amount(-1000001, Symbol::from("4,SYM")), Asset::from("-100.0001 SYM"));
        assert_eq!(Asset::from_amount(0, Symbol::from("0,SYM")), Asset::from("0 SYM"));
        assert_eq!(Asset::from_amount(0, Symbol::from("4,SYM")), Asset::from("0.0000 SYM"));
        assert_eq!(
            Asset::from_amount(-1000000000000000000, Symbol::from("18,SYMBOLL")),
            Asset::from("-1.000000000000000000 SYMBOLL")
        );
    }
}
