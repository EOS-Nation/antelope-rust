pub fn check(predicate: bool, message: &str) {
    if !predicate {
        panic!("{}", message)
    }
}
