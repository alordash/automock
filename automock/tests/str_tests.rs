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
        accept::setup(expected.as_str()).expect_once();

        // Act
        accept(&actual);

        // Assert
        accept::verify_calls();
    }

    #[test]
    fn accept_RefEq_ComparesByValue() {
        // Arrange
        let actual = "foo".to_owned();
        accept::setup(Arg::ref_eq(actual.as_str())).expect_once();

        // Act
        accept(&actual);

        // Assert
        accept::verify_calls();
    }
}
