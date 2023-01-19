/// Checks that a condition is true, and panics if it is not.
///
/// Reference: <https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/check.hpp>
///
/// This is a convenience function for writing tests.
///
/// # Examples
/// ```
/// use antelope::check::check;
///
/// #[test]
/// check(true, "This should not panic");
///
/// #[test]
/// #[should_panic]
/// check(false, "This should panic");
/// ```
pub fn check(predicate: bool, message: &str) {
    assert!(predicate, "{}", message)
}

#[cfg(test)]
mod check_tests {
    use super::*;

    #[test]
    fn test_check_true() {
        check(true, "This should not panic")
    }

    #[test]
    #[should_panic(expected = "This should panic")]
    fn test_check_false() {
        check(false, "This should panic");
    }
}
