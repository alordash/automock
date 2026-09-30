use automock::*;

#[mock]
trait Trait {}

#[mock]
struct Struct;

#[mock]
impl Struct {
    pub fn new() -> Self {
        Self
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn mock_id_IsSequentialRegardlessOfMockType() {
        // Act
        Struct::static_setup().new().call_base();
        let mock_1 = TraitMock::new();
        let mock_2 = Struct::new();
        let mock_3 = TraitMock::new();
        let mock_4 = Struct::new();

        // Assert
        let id_1 = mock_1.id();
        let expected_id_2 = id_1 + 1;
        let expected_id_3 = id_1 + 2;
        let expected_id_4 = id_1 + 3;

        assert_eq!(mock_2.id(), expected_id_2);
        assert_eq!(mock_3.id(), expected_id_3);
        assert_eq!(mock_4.id(), expected_id_4);
    }
}
