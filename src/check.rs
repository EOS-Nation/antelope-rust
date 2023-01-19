/// Checks that a condition is true, and panics if it is not.
///
/// This is a convenience function for writing tests.
///
/// # Examples
/// ```
/// use antelope::check::check;
///
/// #[test]
/// #[should_panic]
/// check(true, "This should not panic");
///
/// #[test]
/// check(false, "This should panic");
/// ```
#[inline]
pub fn check(predicate: bool, message: &str) {
    assert!(predicate, "{}", message)
}
