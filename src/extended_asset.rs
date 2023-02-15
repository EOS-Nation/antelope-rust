use crate::{check, Asset, ExtendedSymbol, Name};
/// The `ExtendedAsset` struct represents an extended asset
///
/// Reference: <https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/asset.hpp>
///
/// # Examples
///
/// ```
/// use antelope::{ExtendedAsset, Name, Symbol, ExtendedSymbol};
///
/// let ext_asset = ExtendedAsset::from_amount(10000, ExtendedSymbol::from_extended(Symbol::from("4,FOO"), Name::from("contract")));
/// assert_eq!(10000, ext_asset.quantity.amount);
/// ```
#[derive(Eq, Copy, Clone, Debug, Default)]
pub struct ExtendedAsset {
    /**
     * The asset
     */
    pub quantity: Asset,
    /**
     * The contract for the asset
     */
    pub contract: Name,
}

impl ExtendedAsset {
    /**
     * Default constructor
     */
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            quantity: Asset::new(),
            contract: Name::new(),
        }
    }

    /**
     * Construct a new extended asset given the asset and owner name
     */
    #[inline]
    #[must_use]
    pub fn from_asset(quantity: Asset, contract: Name) -> Self {
        ExtendedAsset { quantity, contract }
    }

    /**
     * Construct a new extended asset given the amount and extended symbol
     */
    #[inline]
    #[must_use]
    pub fn from_amount(amount: i64, extended_symbol: ExtendedSymbol) -> Self {
        ExtendedAsset {
            quantity: Asset::from_amount(amount, extended_symbol.get_symbol()),
            contract: extended_symbol.get_contract(),
        }
    }

    /**
     * Get the extended symbol of the asset
     *
     * @return extended_symbol - The extended symbol of the asset
     */
    #[inline]
    #[must_use]
    pub fn get_extended_symbol(&self) -> ExtendedSymbol {
        ExtendedSymbol::from_extended(self.quantity.symbol, self.contract)
    }

    /**
     * Check if the extended asset is valid. %A valid extended asset has valid quantity and contract
     *
     * @return true - if the extended asset is valid
     * @return false - otherwise
     */
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.quantity.is_valid() && self.contract.raw() != 0
    }
}

impl std::fmt::Display for ExtendedAsset {
    /**
     * Converts the extended asset into string
     *
     * @return String in the form of "1.2345 SYM@contract" format
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}@{}", self.quantity, self.contract)
    }
}

impl From<&str> for ExtendedAsset {
    /**
     * Parse ExtendedAsset from string formatted as "1.2345 SYM@contract"
     *
     */
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('@').collect();

        check(parts.len() == 2, "invalid extended asset format");

        let quantity_str = parts[0];
        let contract_str = parts[1];

        ExtendedAsset::from_asset(Asset::from(quantity_str), Name::from(contract_str))
    }
}

impl AsRef<ExtendedAsset> for ExtendedAsset {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &ExtendedAsset {
        self
    }
}

impl std::cmp::PartialEq for ExtendedAsset {
    fn eq(&self, other: &ExtendedAsset) -> bool {
        check(self.contract == other.contract, "type mismatch");
        self.quantity == other.quantity
    }
}

impl std::cmp::PartialOrd for ExtendedAsset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        check(self.contract == other.contract, "type mismatch");
        self.quantity.partial_cmp(&other.quantity)
    }
}

impl std::cmp::Ord for ExtendedAsset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        check(self.contract == other.contract, "type mismatch");
        self.quantity.cmp(&other.quantity)
    }
}

impl std::ops::SubAssign for ExtendedAsset {
    /**
     * Subtraction assignment operator
     *
     * @param other - Another extended asset to subtract this extended asset with
     * @post The amount of this extended asset is subtracted by the amount of extended asset `other`
     */
    fn sub_assign(&mut self, other: ExtendedAsset) {
        check(self.contract == other.contract, "type mismatch");
        self.quantity -= other.quantity;
    }
}

impl std::ops::AddAssign for ExtendedAsset {
    /**
     * Addition assignment operator
     *
     * @param other - Another extended asset to add this extended asset with
     * @post The amount of this extended asset is subtracted by the amount of extended asset `other`
     */
    fn add_assign(&mut self, other: ExtendedAsset) {
        check(self.contract == other.contract, "type mismatch");
        self.quantity += other.quantity;
    }
}

impl std::ops::MulAssign<i64> for ExtendedAsset {
    /**
     * Multiplication assignment operator, with a number
     *
     * @details Multiplication assignment operator. Multiply the amount of this asset with a number and then assign the value to itself.
     * @param a - The multiplier for the asset's amount
     * @return asset - Reference to this asset
     * @post The amount of this asset is multiplied by a
     */
    fn mul_assign(&mut self, a: i64) {
        self.quantity *= a;
    }
}

impl std::ops::DivAssign<i64> for ExtendedAsset {
    /**
     * Division assignment operator, with a number proceeding
     *
     * @brief Division assignment operator, with a number proceeding
     * @param self - The asset to be divided
     * @param a - The divisor for the asset's amount
     * @return asset - Reference to the asset, which has been divided
     */
    fn div_assign(&mut self, a: i64) {
        self.quantity /= a;
    }
}

impl std::ops::Neg for ExtendedAsset {
    type Output = ExtendedAsset;
    /**
     * Negate the amount of the asset
     *
     * @return a new asset with the negated amount
     */
    fn neg(self) -> ExtendedAsset {
        ExtendedAsset {
            quantity: -self.quantity,
            contract: self.contract,
        }
    }
}

impl std::ops::Add for ExtendedAsset {
    type Output = Self;

    /**
     * Addition operator
     *
     * @param other - The second asset to be added to the first asset
     * @return asset - New asset as the result of addition
     */
    fn add(self, other: Self) -> Self {
        check(self.contract == other.contract, "type mismatch");
        let mut result = self;
        result += other;
        result
    }
}

impl std::ops::Sub for ExtendedAsset {
    type Output = Self;

    /**
     * Addition operator
     *
     * @param other - The second asset to be added to the first asset
     * @return asset - New asset as the result of addition
     */
    fn sub(self, other: Self) -> Self {
        check(self.contract == other.contract, "type mismatch");
        let mut result = self;
        result -= other;
        result
    }
}

impl std::ops::Mul<i64> for ExtendedAsset {
    type Output = ExtendedAsset;

    /**
     * Multiplication operator, with a number proceeding
     *
     * @brief Multiplication operator, with a number proceeding
     * @param a - The asset to be multiplied
     * @param b - The multiplier for the asset's amount
     * @return asset - New asset as the result of multiplication
     */
    fn mul(self, b: i64) -> ExtendedAsset {
        let mut result = self;
        result *= b;
        result
    }
}

impl std::ops::Mul<ExtendedAsset> for i64 {
    type Output = ExtendedAsset;

    /**
     * Multiplication operator, with a number preceeding
     *
     * @param a - The multiplier for the asset's amount
     * @param b - The asset to be multiplied
     * @return asset - New asset as the result of multiplication
     */
    fn mul(self, a: ExtendedAsset) -> ExtendedAsset {
        a * self
    }
}

impl std::ops::Div<i64> for ExtendedAsset {
    type Output = ExtendedAsset;

    /**
     * Division operator, with a number proceeding
     *
     * @param a - The asset to be divided
     * @param b - The divisor for the asset's amount
     * @return asset - New asset as the result of division
     */
    fn div(self, b: i64) -> ExtendedAsset {
        let mut result = self;
        result /= b;
        result
    }
}

impl std::ops::Div<ExtendedAsset> for ExtendedAsset {
    type Output = i64;

    /**
     * Division operator, with another asset
     *
     * @param a - The asset which amount acts as the dividend
     * @param b - The asset which amount acts as the divisor
     * @return int64_t - the resulted amount after the division
     * @pre Both asset must have the same symbol
     */
    fn div(self, other: ExtendedAsset) -> Self::Output {
        check(self.contract == other.contract, "type mismatch");
        self.quantity / other.quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Symbol;

    #[test]
    fn test_new_extended_asset() {
        let asset = ExtendedAsset::new();
        assert_eq!(asset.quantity, Asset::new());
        assert_eq!(asset.contract, Name::new());
    }

    #[test]
    fn test_from_asset() {
        let quantity = Asset::from_amount(1234567, Symbol::from("4,SYM"));
        let contract = Name::from("contract");
        let asset = ExtendedAsset::from_asset(quantity, contract);
        assert_eq!(asset.quantity, quantity);
        assert_eq!(asset.contract, contract);
    }

    #[test]
    fn test_from_amount() {
        let extended_symbol = ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract"));
        let asset = ExtendedAsset::from_amount(1234567, extended_symbol);
        assert_eq!(asset.quantity, Asset::from_amount(1234567, extended_symbol.get_symbol()));
        assert_eq!(asset.contract, extended_symbol.get_contract());
        assert_eq!(asset.quantity.amount, 1234567);
    }

    #[test]
    fn test_get_extended_symbol() {
        let extended_symbol = ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract"));
        let asset = ExtendedAsset::from_amount(1234567, extended_symbol);
        assert_eq!(asset.get_extended_symbol(), extended_symbol);
    }

    #[test]
    fn test_ord() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract")));
        let b = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract")));
        let c = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract")));

        assert!(a < b);
        assert!(b > a);
        assert!(b == c);
        assert!(b != a);
    }

    #[test]
    #[should_panic(expected = "comparison of assets with different symbols is not allowed")]
    fn test_ord_panic1() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract")));
        let b = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("5,SYM"), Name::from("contract")));

        assert!(a != b);
    }
    #[test]
    #[should_panic(expected = "comparison of assets with different symbols is not allowed")]
    fn test_ord_panic2() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract")));
        let b = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("5,SYM"), Name::from("contract")));

        assert!(a < b);
    }
    #[test]
    #[should_panic(expected = "comparison of assets with different symbols is not allowed")]
    fn test_ord_panic3() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract")));
        let b = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("5,SYM"), Name::from("contract")));

        assert!(a <= b);
    }
    #[test]
    #[should_panic(expected = "type mismatch")]
    fn test_ord_panic4() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract1")));
        let b = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract2")));

        assert!(a != b);
    }
    #[test]
    #[should_panic(expected = "type mismatch")]
    fn test_ord_panic5() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract1")));
        let b = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract2")));

        assert!(a < b);
    }
    #[test]
    #[should_panic(expected = "type mismatch")]
    fn test_ord_panic6() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract1")));
        let b = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract2")));

        assert!(a <= b);
    }

    #[test]
    fn test_partial_ord() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract")));
        let b = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract")));
        let c = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract")));

        assert!(a <= b);
        assert!(b >= a);
        assert!(b > a);
        assert!(b >= c);
        assert!(a < b);
        assert!(c <= b);
    }

    #[test]
    fn test_is_valid() {
        assert!(ExtendedAsset::from_amount(100, ExtendedSymbol::from("4,SYM@contract")).is_valid());
        assert!(ExtendedAsset::from_asset(Asset::from_amount(1234567, Symbol::from("4,SYM")), Name::from("contract")).is_valid());
        assert!(!ExtendedAsset::from_asset(Asset::from_amount(1234567, Symbol::from("4,SYM")), Name::new()).is_valid());
        assert!(!ExtendedAsset::from_asset(Asset::new(), Name::from("contract")).is_valid());
        assert!(!ExtendedAsset::from_asset(Asset::default(), Name::default()).is_valid());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract")).to_string(),
            "1.0000 SYM@contract"
        );
        assert_eq!(
            ExtendedAsset::from_amount(0, ExtendedSymbol::from("4,SYM@contract")).to_string(),
            "0.0000 SYM@contract"
        );
        assert_eq!(
            ExtendedAsset::from_amount(1, ExtendedSymbol::from("0,SYM@contract")).to_string(),
            "1 SYM@contract"
        );
        assert_eq!(
            ExtendedAsset::from_asset(Asset::new(), Name::from("contract")).to_string(),
            "0 @contract"
        );
        assert_eq!(ExtendedAsset::from_asset(Asset::default(), Name::default()).to_string(), "0 @");
    }

    #[test]
    fn test_sub_assign() {
        let mut asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));
        let asset2 = ExtendedAsset::from_amount(20000, ExtendedSymbol::from("4,SYM@contract"));
        asset1 -= asset2;

        assert_eq!(asset1.quantity.amount, -10000);
    }

    #[test]
    fn test_add_assign() {
        let mut asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));
        let asset2 = ExtendedAsset::from_amount(20000, ExtendedSymbol::from("4,SYM@contract"));
        asset1 += asset2;

        assert_eq!(asset1.quantity.amount, 30000);
        assert_eq!(asset1.to_string(), "3.0000 SYM@contract");
    }

    #[test]
    fn test_mul_assign() {
        let mut asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));
        asset1 /= 2;

        assert_eq!(asset1.quantity.amount, 5000);
        assert_eq!(asset1.to_string(), "0.5000 SYM@contract");
    }

    #[test]
    fn test_div_assign() {
        let mut asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));
        asset1 *= 2;

        assert_eq!(asset1.quantity.amount, 20000);
        assert_eq!(asset1.to_string(), "2.0000 SYM@contract");
    }

    #[test]
    fn test_neg() {
        let mut asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));
        asset1 = -asset1;

        assert_eq!(asset1.quantity.amount, -10000);
        assert_eq!(asset1.to_string(), "-1.0000 SYM@contract");
    }

    #[test]
    fn test_add_operator() {
        let asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));
        let asset2 = ExtendedAsset::from_amount(20000, ExtendedSymbol::from("4,SYM@contract"));

        let result = asset1 + asset2;
        assert_eq!(result.quantity.amount, 30000);
        assert_eq!(result.to_string(), "3.0000 SYM@contract");
    }

    #[test]
    fn test_sub_operator() {
        let asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));
        let asset2 = ExtendedAsset::from_amount(20000, ExtendedSymbol::from("4,SYM@contract"));

        let result = asset1 - asset2;
        assert_eq!(result.quantity.amount, -10000);
        assert_eq!(result.to_string(), "-1.0000 SYM@contract");
    }

    #[test]
    fn test_div_operator() {
        let asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));
        let asset2 = ExtendedAsset::from_amount(20000, ExtendedSymbol::from("4,SYM@contract"));

        let result = asset2 / asset1;
        assert_eq!(result, 2);
    }

    #[test]
    fn test_mul_operator() {
        let asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));

        let result = asset1 * 2;
        assert_eq!(result.quantity.amount, 20000);
        assert_eq!(result.to_string(), "2.0000 SYM@contract");
    }

    #[test]
    fn test_div_operator2() {
        let asset1 = ExtendedAsset::from_amount(10000, ExtendedSymbol::from("4,SYM@contract"));

        let result = asset1 / 2;
        assert_eq!(result.quantity.amount, 5000);
        assert_eq!(result.to_string(), "0.5000 SYM@contract");
    }

    #[test]
    #[should_panic(expected = "type mismatch")]
    fn test_add_panic() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract1")));
        let b = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract2")));

        let _ = a + b;
    }

    #[test]
    #[should_panic(expected = "type mismatch")]
    fn test_sub_panic() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract1")));
        let b = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract2")));

        let _ = a - b;
    }

    #[test]
    #[should_panic(expected = "type mismatch")]
    fn test_div_panic() {
        let a = ExtendedAsset::from_amount(100, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract1")));
        let b = ExtendedAsset::from_amount(200, ExtendedSymbol::from_extended(Symbol::from("4,SYM"), Name::from("contract2")));

        let _ = a / b;
    }

    #[test]
    fn test_from_str() {
        let expected = ExtendedAsset {
            quantity: Asset::from_amount(1_0000, Symbol::from("4,SYM")),
            contract: Name::from("contract"),
        };
        let actual = ExtendedAsset::from("1.0000 SYM@contract");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_str2() {
        let expected = ExtendedAsset {
            quantity: Asset::from_amount(1_0000, Symbol::from("4,SYM")),
            contract: Name::default(),
        };
        let actual = ExtendedAsset::from("1.0000 SYM@");
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic(expected = "invalid extended asset format")]
    fn test_from_str_with_invalid_input() {
        let _ = ExtendedAsset::from("1.0000SYM");
    }
}
