use automock::*;

#[mock]
trait Trait: Sized {
    fn number(&self) -> i32;

    fn work(self) -> Vec<Self> {
        vec![self]
    }
}

#[mock]
#[derive(Clone)]
struct Struct {
    id: i32,
}

#[mock]
impl Struct {
    pub fn new(id: i32) -> Self {
        Self { id }
    }

    pub fn work(self) -> Vec<Self> {
        vec![self.clone(), self]
    }
}

#[mock]
impl Trait for Struct {
    fn number(&self) -> i32 {
        self.id
    }

    fn work(self) -> Vec<Self> {
        vec![self.clone(), self.clone(), self]
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    mod Trait_tests {
        use super::*;

        #[test]
        fn Trait_work_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mut returned_mock = TraitMock::new();
            let returned_mock_id = 2;
            returned_mock.setup().number().returns(returned_mock_id);
            mock.setup().work().returns(vec![returned_mock]);

            // Act
            let mut result = mock.work();

            // Assert
            assert_eq!(1, result.len());
            assert_eq!(returned_mock_id, result[0].number());
            result[0]
                .received()
                .number(Times::Once)
                .work(Times::Never)
                .no_other_calls();
        }

        #[test]
        fn Trait_work_Base_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_id = 1;
            mock.setup().number().returns(mock_id).work().call_base();

            // Act
            let mut result = mock.work();

            // Assert
            assert_eq!(1, result.len());
            assert_eq!(mock_id, result[0].number());
            result[0].received().number(Times::Once).no_other_calls();
        }
    }

    mod Struct_tests {
        use super::*;

        #[test]
        fn Struct_work_Ok() {
            // Arrange
            Struct::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(1);
            let returned_mock_id = 2;
            let returned_mock = Struct::new(returned_mock_id);
            mock.setup().work().returns(vec![returned_mock]);

            // Act
            let mut result = mock.work();

            // Assert
            assert_eq!(1, result.len());
            assert_eq!(returned_mock_id, result[0].id);
            result[0].received().work(Times::Never).no_other_calls();
        }

        #[test]
        fn Struct_work_Base_Ok() {
            // Arrange
            let mock_id = 1;
            Struct::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(mock_id);
            mock.setup().work().call_base();

            // Act
            let mut result = mock.work();

            // Assert
            assert_eq!(2, result.len());
            assert_eq!(mock_id, result[0].id);
            assert_eq!(mock_id, result[1].id);
            result[0].received().no_other_calls();
            result[1].received().no_other_calls();
        }
    }

    mod Struct_as_Trait_tests {
        use super::*;

        #[test]
        fn Struct_as_Trait_work_Ok() {
            // Arrange
            Struct::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(1);
            let returned_trait_mock_id = 20;
            let mut returned_mock = Struct::new(2);
            returned_mock
                .setup()
                .as_Trait()
                .number()
                .returns(returned_trait_mock_id);
            mock.setup().as_Trait().work().returns(vec![returned_mock]);

            // Act
            let mut result = Trait::work(mock);

            // Assert
            assert_eq!(1, result.len());
            assert_eq!(returned_trait_mock_id, result[0].number());
            result[0]
                .received()
                .as_Trait()
                .number(Times::Once)
                .work(Times::Never)
                .no_other_calls();
        }

        #[test]
        fn Struct_as_Trait_work_Base_Ok() {
            // Arrange
            let trait_mock_id = 10;
            Struct::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(1);
            mock.setup()
                .as_Trait()
                .number()
                .returns(trait_mock_id)
                .work()
                .call_base();

            // Act
            let mut result = Trait::work(mock);

            // Assert
            assert_eq!(3, result.len());
            assert_eq!(trait_mock_id, result[0].number());
            result[0]
                .received()
                .as_Trait()
                .number(Times::Once)
                .no_other_calls();
            result[1]
                .received()
                .as_Trait()
                .number(Times::Once)
                .no_other_calls();
            result[2]
                .received()
                .as_Trait()
                .number(Times::Once)
                .no_other_calls();
        }
    }
}
