use automock::*;

#[mock(base)]
trait Trait {
    fn get(&self) -> i32;

    fn get_plus_one(&self) -> i32 {
        let value = self.get() + 1;
        return value;
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn get_plus_one_Ok() {
        // Arrange
        let mut mock = TraitMock::new();

        let value = 302;
        mock.setup().get_plus_one().returns(value);

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        assert_eq!(value, actual_value);

        mock.received()
            .get_plus_one(Times::Once)
            .get(Times::Never)
            .no_other_calls();
    }

    #[test]
    fn get_plus_one_CallBase_Ok() {
        // Arrange
        let mut mock = TraitMock::new();

        let value = 302;
        mock.setup().get().returns(value).get_plus_one().call_base();

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        let expected_value = value + 1;
        assert_eq!(expected_value, actual_value);

        mock.received()
            .get(Times::Once)
            .get_plus_one(Times::Once)
            .no_other_calls();
    }
}
