use automock::*;

#[mock]
trait Trait {
    fn work(&self) -> i32;
}

#[mock]
struct Struct;

#[mock]
impl Struct {
    fn new() -> Self {
        Self
    }
}

#[mock]
impl Struct {
    fn work(&self) -> i32 {
        unreachable!()
    }
}

#[mock]
impl Trait for Struct {
    fn work(&self) -> i32 {
        unreachable!()
    }
}

mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn TraitMock_MockData_ContainsSameConfiguration() {
        // Arrange
        let mut mock = TraitMock::new();
        let return_value = 10;
        mock.setup().work().returns_once(return_value);
        let mock_data = mock.mock_data();

        // Act
        let result = mock.work();

        // Assert
        assert_eq!(return_value, result);
        mock.verify_calls();
        mock_data.verify_calls();
    }

    #[test]
    fn StructMock_MockData_ContainsSameConfiguration() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let return_value = 10;
        mock.setup().work().returns_once(return_value);
        let mock_data = mock.mock_data();

        // Act
        let result = mock.work();

        // Assert
        assert_eq!(return_value, result);
        mock.verify_calls();
        mock_data.verify_calls();
    }

    #[test]
    fn StructMockAsTrait_MockData_ContainsSameConfiguration() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let return_value = 10;
        mock.setup().as_Trait().work().returns_once(return_value);
        let mock_data = mock.mock_data();

        // Act
        let result = Trait::work(&mock);

        // Assert
        assert_eq!(return_value, result);
        mock.verify_calls();
        mock_data.verify_calls();
    }
}
