use automock::*;

#[mock]
trait Trait {
    fn work(&self) -> i32;
}

#[mock]
#[derive(Clone)]
struct Struct;

#[mock(base)]
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
    fn TraitMock_Clone_ContainsSameConfiguration() {
        // Arrange
        let mut mock = TraitMock::new();
        let return_value = 10;
        mock.setup().work().returns(return_value);

        // Act
        let mut clone = mock.clone();
        let result = clone.work();

        // Assert
        assert_eq!(return_value, result);
        mock.received().work(Times::Once);
        clone.received().work(Times::Once);
        mock.received().no_other_calls();
        clone.received().no_other_calls();
    }

    #[test]
    fn StructMock_Clone_ContainsSameConfiguration() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let return_value = 10;
        mock.setup().work().returns(return_value);

        // Act
        let mut clone = mock.clone();
        let result = clone.work();

        // Assert
        assert_eq!(return_value, result);
        mock.received().work(Times::Once);
        clone.received().work(Times::Once);
        mock.received().no_other_calls();
        clone.received().no_other_calls();
    }

    #[test]
    fn StructMockAsTrait_Clone_ContainsSameConfiguration() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let return_value = 10;
        mock.setup().as_Trait().work().returns(return_value);

        // Act
        let mut clone = mock.clone();
        let result = Trait::work(&clone);

        // Assert
        assert_eq!(return_value, result);
        mock.received().as_Trait().work(Times::Once);
        clone.received().as_Trait().work(Times::Once);
        mock.received().as_Trait().no_other_calls();
        clone.received().as_Trait().no_other_calls();
    }
}
