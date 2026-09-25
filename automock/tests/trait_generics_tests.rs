use automock::*;

#[mock]
trait Trait<T1, T2 = u16> {
    fn get_return(&self, value: T1) -> T1;

    fn return_where_constraint(&self) -> T1
    where
        T1: Default;

    fn get_return_different(&self, value: T1) -> T2;
}

#[mock]
struct Structs00;
#[mock(base)]
impl Structs00 {
    pub const FOO: i32 = 1;
}

trait Consumer<T> {}

#[mock]
trait Another {
    type Item;

    #[allow(unused)]
    fn work<T: Into<<Self as Another>::Item>>(&self)
    where
        Self: Consumer<<Self as Another>::Item>;
}

mod trait_generic_tests {
    #![allow(non_snake_case)]
    use super::*;

    mod get_return_tests {
        use super::*;

        #[test]
        fn get_return_first_type_i32_Ok() {
            // Arrange
            let mut mock = TraitMock::<_>::new();
            let accepted_value = 10;
            let returned_value = 200;
            mock.setup()
                .get_return(accepted_value)
                .returns(returned_value);

            // Act
            let actual_returned_value = mock.get_return(accepted_value);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            mock.received()
                .get_return(accepted_value, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn get_return_first_type_str_Ok() {
            // Arrange
            let mut mock = TraitMock::<_, ()>::new();
            let accepted_value = "quo vadis";
            let returned_value = "veridis quo";
            mock.setup()
                .get_return(accepted_value)
                .returns(returned_value);

            // Act
            let actual_returned_value = mock.get_return(accepted_value);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            mock.received()
                .get_return(accepted_value, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn get_return_first_type_explicit_type_Ok() {
            // Arrange
            let mut mock = TraitMock::<u8, Box<Vec<String>>>::new();
            let accepted_value = 10u8;
            let returned_value = 20u8;
            mock.setup()
                .get_return(accepted_value)
                .returns(returned_value);

            // Act
            let actual_returned_value = mock.get_return(accepted_value);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            mock.received()
                .get_return(accepted_value, Times::Once)
                .no_other_calls();
        }
    }

    mod return_where_constraint_tests {
        use super::*;

        #[test]
        fn return_where_constraint_Ok() {
            // Arrange
            let mut mock = TraitMock::<_, ()>::new();

            #[derive(Clone, PartialOrd, PartialEq, Debug, Default)]
            struct Foo {
                number: i32,
            }

            let returned_value = Foo { number: 100 };
            mock.setup()
                .return_where_constraint()
                .returns(returned_value.clone());

            // Act
            let actual_returned_value = mock.return_where_constraint();

            // Assert
            assert_eq!(returned_value, actual_returned_value);
            mock.received()
                .return_where_constraint(Times::Once)
                .no_other_calls();
        }
    }

    #[test]
    fn get_return_and_get_return_different_get_return_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        let accepted_get_return_value = 10;
        let returned_get_return_value = 20;
        let accepted_get_return_different_value = 15;
        let returned_get_return_different_value = "quo vadis";
        mock.setup()
            .get_return(accepted_get_return_value)
            .returns(returned_get_return_value)
            .get_return_different(accepted_get_return_different_value)
            .returns(returned_get_return_different_value);

        // Act
        let actual_returned_get_return_value = mock.get_return(accepted_get_return_value);
        let actual_returned_get_return_different_value =
            mock.get_return_different(accepted_get_return_different_value);

        // Assert
        assert_eq!(returned_get_return_value, actual_returned_get_return_value);
        assert_eq!(
            returned_get_return_different_value,
            actual_returned_get_return_different_value
        );

        mock.received()
            .get_return(accepted_get_return_value, Times::Once)
            .get_return_different(accepted_get_return_different_value, Times::Once)
            .no_other_calls();
    }
}
