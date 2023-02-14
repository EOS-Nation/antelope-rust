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
    pub fn get_extended_symbol(&self) -> ExtendedSymbol {
        ExtendedSymbol::from_extended(self.quantity.symbol, self.contract)
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
}
