use automock::*;

#[mock]
fn accept(_: &str) {}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn accept_Raw_ComparesByValue() {
        // Arrange
        let actual = "foo".to_owned();
        let expected = "foo".to_owned();

        // Act
        accept(&actual);

        // Assert
        accept::received(expected.as_str(), 1.time()).no_other_calls();
    }

    #[test]
    fn accept_RefEq_ComparesByValue() {
        // Arrange
        let actual = "foo".to_owned();
        let unexpected = "foo".to_owned();

        // Act
        accept(&actual);

        // Assert
        accept::received(Arg::ref_eq(unexpected.as_str()), Times::Never)
            .received(Arg::ref_eq(actual.as_str()), 1.time())
            .no_other_calls();
    }
}
