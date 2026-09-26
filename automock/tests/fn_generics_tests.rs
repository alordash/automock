#![allow(clippy::multiple_bound_locations)]
use automock::*;
use std::fmt::{Debug, Display};

mod common;

#[mock(base)]
fn get_return<T: Clone>(value: T) -> T {
    return value;
}

#[mock(base)]
fn return_constraint<T: Default>() -> T {
    return T::default();
}

#[mock(base)]
fn return_where_constraint<T: Default>() -> T
where
    T: Debug,
{
    return T::default();
}

#[mock(base)]
fn get_return_different<T1: Debug, T2>(_value: T1) -> T2
where
    T1: Clone + Display,
    T2: Default,
{
    T2::default()
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    mod get_return_tests {
        use super::*;

        #[test]
        fn get_return_Ok() {
            // Arrange
            let accepted_number = 10;
            let returned_number = 20;
            get_return::setup(accepted_number).returns(returned_number);

            // Act
            let actual_returned_number = get_return(accepted_number);

            // Assert
            assert_eq!(returned_number, actual_returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();
        }

        #[test]
        fn get_return_TwoTypes_Ok() {
            // Arrange
            let accepted_number = 10;
            let returned_number = 20;
            get_return::setup(accepted_number).returns(returned_number);

            let accepted_str = "quo vadis";
            let returned_str = "veridis quo";
            get_return::setup(accepted_str).returns(returned_str);

            // Act
            let actual_returned_number = get_return(accepted_number);
            let actual_returned_str = get_return(accepted_str);

            // Assert
            assert_eq!(returned_number, actual_returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();

            assert_eq!(returned_str, actual_returned_str);
            get_return::received(accepted_str, Times::Once).no_other_calls();
        }

        #[test]
        fn get_return_UnconfiguredType_Panics() {
            // Arrange
            let accepted_number = 10;
            let returned_number = 20;
            get_return::setup(accepted_number).returns(returned_number);
            let accepted_str = "str wasn't configured";

            // Act
            let actual_returned_number = get_return(accepted_number);
            let actual_error_msg = record_panic(|| get_return(accepted_str));

            // Assert
            assert_eq!(returned_number, actual_returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();
            let accepted_str_debug_string = common::debug_string(format!("{accepted_str:?}"));
            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call:
	get_return<&str>({accepted_str_debug_string})"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
        }

        #[test]
        fn get_return_CallBase_Ok() {
            // Arrange
            let accepted_number = 33;
            get_return::setup(accepted_number).call_base();

            // Act
            let returned_number = get_return(accepted_number);

            // Assert
            assert_eq!(accepted_number, returned_number);
            get_return::received_nothing::<i32>();
        }
    }

    mod return_constraint_tests {
        use super::*;

        #[test]
        fn return_constraint_Ok() {
            // Arrange
            let returned_number_value = 100;
            return_constraint::setup().returns(returned_number_value);
            let returned_string_value = String::from("quo vadis");
            return_constraint::setup().returns(returned_string_value.clone());

            // Act
            let actual_number_value = return_constraint();
            let actual_string_value: String = return_constraint();

            // Assert
            assert_eq!(returned_number_value, actual_number_value);
            return_constraint::received::<i32>(Times::Once).no_other_calls();

            assert_eq!(returned_string_value, actual_string_value);
            return_constraint::received::<String>(Times::Once).no_other_calls();

            return_constraint::received::<()>(Times::Never).no_other_calls();
        }

        #[test]
        fn return_constraint_CallBase_Ok() {
            // Arrange
            return_constraint::setup::<i32>().call_base();

            // Act
            let actual_number_value = return_constraint();

            // Assert
            assert_eq!(i32::default(), actual_number_value);
            return_constraint::received_nothing::<i32>();
        }
    }

    mod return_where_constraint_tests {
        use super::*;

        #[test]
        fn return_where_constraint_Ok() {
            // Arrange
            let returned_number_value = 100;
            return_where_constraint::setup().returns(returned_number_value);
            let returned_string_value = String::from("quo vadis");
            return_where_constraint::setup().returns(returned_string_value.clone());

            // Act
            let actual_number_value = return_where_constraint();
            let actual_string_value: String = return_where_constraint();

            // Assert
            assert_eq!(returned_number_value, actual_number_value);
            return_where_constraint::received::<i32>(Times::Once).no_other_calls();

            assert_eq!(returned_string_value, actual_string_value);
            return_where_constraint::received::<String>(Times::Once).no_other_calls();

            return_where_constraint::received::<()>(Times::Never).no_other_calls();
        }

        #[test]
        fn return_constraint_CallBase_Ok() {
            // Arrange
            return_where_constraint::setup::<i32>().call_base();

            // Act
            let actual_number_value = return_where_constraint();

            // Assert
            assert_eq!(i32::default(), actual_number_value);
            return_where_constraint::received_nothing::<i32>();
        }
    }

    mod get_return_different_tests {
        use super::*;

        #[test]
        fn get_return_different_Ok() {
            // Arrange
            let first_accepted_number = 10;
            let returned_string = String::from("quo vadis");
            get_return_different::setup::<i32, _>(Arg::is(|x| *x == first_accepted_number))
                .returns(returned_string.clone());

            let second_accepted_number = 20;
            let returned_vec = vec![1, 23, 456];
            get_return_different::setup(second_accepted_number).returns(returned_vec.clone());

            // Act
            let actual_returned_string: String = get_return_different(first_accepted_number);
            let actual_returned_vec: Vec<i32> = get_return_different(second_accepted_number);

            // Assert
            assert_eq!(returned_string, actual_returned_string);
            assert_eq!(returned_vec, actual_returned_vec);
            get_return_different::received::<_, String>(first_accepted_number, Times::Once)
                .no_other_calls();
            get_return_different::received::<_, Vec<i32>>(second_accepted_number, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn get_return_different_CallBase_Ok() {
            // Arrange
            let first_accepted_number = 10;
            get_return_different::setup::<i32, String>(Arg::is(|x| *x == first_accepted_number))
                .call_base();

            let second_accepted_number = 20;
            get_return_different::setup::<i32, Vec<i32>>(second_accepted_number).call_base();

            // Act
            let actual_returned_string: String = get_return_different(first_accepted_number);
            let actual_returned_vec: Vec<i32> = get_return_different(second_accepted_number);

            // Assert
            assert_eq!(String::default(), actual_returned_string);
            assert_eq!(Vec::<i32>::default(), actual_returned_vec);
            get_return_different::received_nothing::<i32, String>();
            get_return_different::received_nothing::<i32, Vec<i32>>();
        }
    }
}
