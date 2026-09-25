#![allow(clippy::arc_with_non_send_sync)]
use automock::*;

#[mock]
struct Struct;

#[mock(base)]
impl Struct {
    pub fn new() -> Self {
        Self
    }
}

#[mock]
impl Struct {
    pub fn accept_value(&self, v: i32) {}

    pub fn return_value(&self) -> i32 {
        unreachable!()
    }

    pub fn accept_value_return_value(&self, v: i32) -> f32 {
        unreachable!()
    }

    pub fn accept_two_values(&self, v1: i32, v2: f32) {}

    pub fn accept_two_values_return_value(&self, v1: i32, v2: f32) -> String {
        unreachable!()
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;
    use std::cell::RefCell;
    use std::sync::Arc;

    mod accept_value_tests {
        use super::*;

        #[test]
        fn accept_value_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let first_value = 10;
            let second_value = 22;

            // Act
            mock.accept_value(first_value);
            mock.accept_value(second_value);
            mock.accept_value(second_value);

            // Assert
            mock.received()
                .accept_value(Arg::Any, Times::Exactly(3))
                .accept_value(first_value, Times::Once)
                .accept_value(
                    Arg::is(|actual_value| *actual_value == first_value),
                    Times::Once,
                )
                .accept_value(Arg::eq(second_value), Times::Exactly(2))
                .accept_value(
                    Arg::is(|actual_value| *actual_value == second_value),
                    Times::Exactly(2),
                )
                .no_other_calls();
        }

        #[test]
        fn accept_value_Callback_ok() {
            // Arrange
            let mut mock = Struct::new();
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            mock.setup()
                .accept_value(Arg::Any)
                .does(move |_, _| *callback_flag_clone.borrow_mut() = true);

            // Act
            mock.accept_value(1);

            // Assert
            assert!(*callback_flag.borrow());
        }

        #[test]
        fn accept_value_ArgAny_Panics() {
            // Arrange
            let mut mock = Struct::new();
            let first_value = 10;
            let second_value = 22;

            // Act
            mock.accept_value(first_value);
            mock.accept_value(second_value);

            // Assert
            assert_panics(
                || mock.received().accept_value(Arg::Any, Times::Never),
                format!(
                    r#"Expected to never receive a call matching:
	Struct::accept_value((i32): any)
Actually received 2 matching calls:
	accept_value({first_value})
	accept_value({second_value})
Received no non-matching calls"#
                ),
            );
            assert_panics(
                || mock.received().accept_value(Arg::Any, Times::Once),
                format!(
                    r#"Expected to receive a call exactly once matching:
	Struct::accept_value((i32): any)
Actually received 2 matching calls:
	accept_value({first_value})
	accept_value({second_value})
Received no non-matching calls"#
                ),
            );
            assert_panics(
                || mock.received().accept_value(Arg::Any, Times::Exactly(3)),
                format!(
                    r#"Expected to receive a call 3 times matching:
	Struct::accept_value((i32): any)
Actually received 2 matching calls:
	accept_value({first_value})
	accept_value({second_value})
Received no non-matching calls"#
                ),
            );
        }

        #[test]
        fn accept_value_ArgEq_Panics() {
            // Arrange
            let mut mock = Struct::new();
            let first_value = 10;
            let second_value = 22;

            // Act
            mock.accept_value(first_value);
            mock.accept_value(second_value);

            // Assert
            assert_panics(
                || {
                    mock.received()
                        .accept_value(Arg::eq(first_value), Times::Never)
                },
                format!(
                    r#"Expected to never receive a call matching:
	Struct::accept_value((i32): equal to {first_value})
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
	1. v (i32):
		Expected: {first_value}
		Actual:   {second_value}"#
                ),
            );
            assert_panics(
                || {
                    mock.received()
                        .accept_value(Arg::eq(first_value), Times::Exactly(3))
                },
                format!(
                    r#"Expected to receive a call 3 times matching:
	Struct::accept_value((i32): equal to {first_value})
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
	1. v (i32):
		Expected: {first_value}
		Actual:   {second_value}"#
                ),
            );
            assert_panics(
                || {
                    mock.received()
                        .accept_value(Arg::eq(second_value), Times::Never)
                },
                format!(
                    r#"Expected to never receive a call matching:
	Struct::accept_value((i32): equal to {second_value})
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Expected: {second_value}
		Actual:   {first_value}"#
                ),
            );
            assert_panics(
                || {
                    mock.received()
                        .accept_value(Arg::eq(second_value), Times::Exactly(3))
                },
                format!(
                    r#"Expected to receive a call 3 times matching:
	Struct::accept_value((i32): equal to {second_value})
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Expected: {second_value}
		Actual:   {first_value}"#
                ),
            );
        }

        #[test]
        fn accept_value_ArgIs_Panics() {
            // Arrange
            let mut mock = Struct::new();
            let first_value = 10;
            let second_value = 22;

            // Act
            mock.accept_value(first_value);
            mock.accept_value(second_value);

            // Assert
            assert_panics(
                || {
                    mock.received().accept_value(
                        Arg::is(|actual_value| *actual_value == first_value),
                        Times::Never,
                    )
                },
                format!(
                    r#"Expected to never receive a call matching:
	Struct::accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {second_value}"#
                ),
            );
            assert_panics(
                || {
                    mock.received().accept_value(
                        Arg::is(|actual_value| *actual_value == first_value),
                        Times::Exactly(3),
                    )
                },
                format!(
                    r#"Expected to receive a call 3 times matching:
	Struct::accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {second_value}"#
                ),
            );
            assert_panics(
                || {
                    mock.received().accept_value(
                        Arg::is(|actual_value| *actual_value == second_value),
                        Times::Never,
                    )
                },
                format!(
                    r#"Expected to never receive a call matching:
	Struct::accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {first_value}"#
                ),
            );
            assert_panics(
                || {
                    mock.received().accept_value(
                        Arg::is(|actual_value| *actual_value == second_value),
                        Times::Exactly(3),
                    )
                },
                format!(
                    r#"Expected to receive a call 3 times matching:
	Struct::accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {first_value}"#
                ),
            );
        }

        #[test]
        fn accept_value_NoOtherCallsWithoutOtherCalls_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let value = 10;

            // Act
            mock.accept_value(value);

            // Assert
            mock.received()
                .accept_value(value, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn accept_value_NoOtherCallsWithOneOtherCall_Panics() {
            // Arrange
            let mut mock = Struct::new();
            let first_value = 10;
            let second_value = 22;

            // Act
            mock.accept_value(first_value);
            mock.accept_value(second_value);

            // Assert
            assert_panics(
                || {
                    mock.received()
                        .accept_value(first_value, Times::Once)
                        .no_other_calls()
                },
                format!(
                    "Did not expect to receive any other calls. Received 1 unexpected call:
1. Struct::accept_value({second_value})"
                ),
            );
        }

        #[test]
        fn accept_value_NoOtherCallsWithManyOtherCalls_Panics() {
            // Arrange
            let mut mock = Struct::new();
            let first_value = 10;
            let second_value = 22;
            let third_value = 333;

            // Act
            mock.accept_value(first_value);
            mock.accept_value(second_value);
            mock.accept_value(third_value);

            // Assert
            assert_panics(
                || {
                    mock.received()
                        .accept_value(first_value, Times::Once)
                        .no_other_calls()
                },
                format!(
                    "Did not expect to receive any other calls. Received 2 unexpected calls:
1. Struct::accept_value({second_value})
2. Struct::accept_value({third_value})"
                ),
            );
        }
    }

    mod return_value_tests {
        use super::*;

        #[test]
        fn return_value_Single_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let value = 10;
            mock.setup().return_value().returns(value);

            // Act
            let actual_value = mock.return_value();

            // Assert
            assert_eq!(value, actual_value);
        }

        #[test]
        fn return_value_UsesFirstConfiguration_Ok() {
            // Arrange
            #[derive(Debug, PartialEq)]
            enum Result {
                DidNotChange,
                SecondConfigChanged,
                ThirdConfigChanged,
            }

            let mut mock = Struct::new();
            let first_value = 10;
            let second_value = 22;
            let third_value = 333;
            let callback_result = Arc::new(RefCell::new(Result::DidNotChange));
            let first_callback_counter_clone = callback_result.clone();
            let second_callback_counter_clone = callback_result.clone();
            mock.setup()
                .return_value()
                .returns(first_value)
                .return_value()
                .returns(second_value)
                .and_does(move |_, _| {
                    *first_callback_counter_clone.borrow_mut() = Result::SecondConfigChanged
                })
                .return_value()
                .returns(third_value)
                .and_does(move |_, _| {
                    *second_callback_counter_clone.borrow_mut() = Result::ThirdConfigChanged
                });

            // Act
            let actual_first_value = mock.return_value();
            let actual_second_value = mock.return_value();
            let actual_third_value = mock.return_value();

            // Assert
            assert_eq!(first_value, actual_first_value);
            assert_eq!(second_value, actual_second_value);
            assert_eq!(third_value, actual_third_value);
            assert_eq!(Result::ThirdConfigChanged, *callback_result.borrow());
        }

        #[test]
        fn return_value_Many_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let first_value = 10;
            let second_value = 22;
            let third_value = 333;
            mock.setup()
                .return_value()
                .returns_many([first_value, second_value, third_value]);

            // Act
            let actual_first_value = mock.return_value();
            let actual_second_value = mock.return_value();
            let actual_third_value = mock.return_value();

            // Assert
            assert_eq!(first_value, actual_first_value);
            assert_eq!(second_value, actual_second_value);
            assert_eq!(third_value, actual_third_value);
        }

        #[test]
        fn return_value_ManyWithCallback_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let callback_counter = Arc::new(RefCell::new(0));
            let callback_counter_clone = callback_counter.clone();
            let first_value = 10;
            let second_value = 22;
            mock.setup()
                .return_value()
                .returns_many([first_value, second_value])
                .and_does(move |_, _| *callback_counter_clone.borrow_mut() += 1);

            // Act
            let actual_first_value = mock.return_value();
            let actual_second_value = mock.return_value();

            // Assert
            assert_eq!(2, *callback_counter.borrow());

            assert_eq!(first_value, actual_first_value);
            assert_eq!(second_value, actual_second_value);
        }

        #[test]
        fn return_value_NoMatchingConfiguration_Panics() {
            // Arrange
            let mock = Struct::new();

            // Act
            let actual_error_msg = record_panic(|| mock.return_value());

            // Assert
            let expected_error_msg = "Mock wasn't configured to handle following call:
	Struct::return_value()";
            assert_eq!(Some(expected_error_msg.to_owned()), actual_error_msg);
        }
    }

    mod accept_value_return_value_tests {
        use super::*;

        #[test]
        fn accept_value_return_value_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let first_accepted_value = 10;
            let first_returned_value = 11.1;
            let second_accepted_value = 20;
            let second_returned_value = 22.2;
            let third_accepted_value = 30;
            let third_returned_value = 33.3;
            mock.setup()
                .accept_value_return_value(Arg::is(|x| *x == first_accepted_value))
                .returns(first_returned_value)
                .accept_value_return_value(Arg::eq(second_accepted_value))
                .returns(second_returned_value)
                .accept_value_return_value(Arg::Any)
                .returns(third_returned_value);

            // Act
            let actual_first_returned_value = mock.accept_value_return_value(first_accepted_value);
            let actual_second_returned_value =
                mock.accept_value_return_value(second_accepted_value);
            let actual_third_returned_value = mock.accept_value_return_value(third_accepted_value);

            // Assert
            assert_eq!(first_returned_value, actual_first_returned_value);
            assert_eq!(second_returned_value, actual_second_returned_value);
            assert_eq!(third_returned_value, actual_third_returned_value);

            mock.received()
                .accept_value_return_value(first_accepted_value, Times::Once)
                .accept_value_return_value(second_accepted_value, Times::Once)
                .accept_value_return_value(third_accepted_value, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn accept_value_return_value_Many1_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let single_accepted_value = 10;
            let double_accepted_value = 20;
            let first_returned_value = 11.1;
            let second_returned_value = 22.2;
            let third_returned_value = 33.3;

            mock.setup()
                .accept_value_return_value(Arg::Any)
                .returns_many([
                    first_returned_value,
                    second_returned_value,
                    third_returned_value,
                ]);

            // Act
            let actual_first_returned_value = mock.accept_value_return_value(single_accepted_value);
            let actual_second_returned_value =
                mock.accept_value_return_value(double_accepted_value);
            let actual_third_returned_value = mock.accept_value_return_value(double_accepted_value);

            // Assert
            assert_eq!(first_returned_value, actual_first_returned_value);
            assert_eq!(second_returned_value, actual_second_returned_value);
            assert_eq!(third_returned_value, actual_third_returned_value);

            mock.received()
                .accept_value_return_value(single_accepted_value, Times::Once)
                .accept_value_return_value(double_accepted_value, Times::Exactly(2))
                .no_other_calls();
        }

        #[test]
        fn accept_value_return_value_Many2_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let first_accepted_value = 10;
            let first_first_returned_value = 11.1;
            let first_second_returned_value = 22.2;

            let second_accepted_value = 200;
            let second_first_returned_value = 201.1;
            let second_second_returned_value = 202.2;
            let second_third_returned_value = 203.3;

            mock.setup()
                .accept_value_return_value(Arg::eq(first_accepted_value))
                .returns_many([first_first_returned_value, first_second_returned_value])
                .accept_value_return_value(Arg::eq(second_accepted_value))
                .returns_many([
                    second_first_returned_value,
                    second_second_returned_value,
                    second_third_returned_value,
                ]);

            // Act
            let actual_first_first_returned_value =
                mock.accept_value_return_value(first_accepted_value);
            let actual_first_second_returned_value =
                mock.accept_value_return_value(first_accepted_value);

            let actual_second_first_returned_value =
                mock.accept_value_return_value(second_accepted_value);
            let actual_second_second_returned_value =
                mock.accept_value_return_value(second_accepted_value);
            let actual_second_third_returned_value =
                mock.accept_value_return_value(second_accepted_value);

            // Assert
            assert_eq!(
                first_first_returned_value,
                actual_first_first_returned_value
            );
            assert_eq!(
                first_second_returned_value,
                actual_first_second_returned_value
            );

            assert_eq!(
                second_first_returned_value,
                actual_second_first_returned_value
            );
            assert_eq!(
                second_second_returned_value,
                actual_second_second_returned_value
            );
            assert_eq!(
                second_third_returned_value,
                actual_second_third_returned_value
            );

            mock.received()
                .accept_value_return_value(first_accepted_value, Times::Exactly(2))
                .accept_value_return_value(second_accepted_value, Times::Exactly(3))
                .no_other_calls();
        }

        #[test]
        fn accept_value_return_value_Callback_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let first_accepted_value = 10;
            let first_callback_number = Arc::new(RefCell::new(0));
            let first_callback_number_clone = first_callback_number.clone();
            let first_returned_value = 11.1;
            let second_accepted_value = 20;
            let second_callback_number = Arc::new(RefCell::new(1));
            let second_callback_number_clone = second_callback_number.clone();
            let second_returned_value = 22.2;
            mock.setup()
                .accept_value_return_value(Arg::eq(first_accepted_value))
                .returns(first_returned_value)
                .and_does(move |_, _| {
                    *first_callback_number_clone.borrow_mut() = 1;
                })
                .accept_value_return_value(Arg::eq(second_accepted_value))
                .returns(second_returned_value)
                .and_does(move |_, _| {
                    *second_callback_number_clone.borrow_mut() = 2;
                });

            // Act
            let actual_first_returned_value = mock.accept_value_return_value(first_accepted_value);
            let actual_second_returned_value =
                mock.accept_value_return_value(second_accepted_value);

            // Assert
            assert_eq!(first_returned_value, actual_first_returned_value);
            assert_eq!(second_returned_value, actual_second_returned_value);

            assert_eq!(1, *first_callback_number.borrow());
            assert_eq!(2, *second_callback_number.borrow());

            mock.received()
                .accept_value_return_value(first_accepted_value, Times::Once)
                .accept_value_return_value(second_accepted_value, Times::Once)
                .no_other_calls();
        }
    }

    mod accept_two_values_tests {
        use super::*;

        #[test]
        fn accept_two_values_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let v1 = 10;
            let v2 = 20.2;

            // Act
            mock.accept_two_values(v1, v2);

            // Assert
            mock.received()
                .accept_two_values(v1, v2, Times::Once)
                .no_other_calls();
        }
    }

    mod accept_two_values_return_value_tests {
        use super::*;

        #[test]
        fn accept_two_values_return_value_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let v1 = 10;
            let v2 = 20.2;
            let returned_value = String::from("quo vadis");
            mock.setup()
                .accept_two_values_return_value(v1, v2)
                .returns(returned_value.clone());

            // Act
            let actual_returned_value = mock.accept_two_values_return_value(v1, v2);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            mock.received()
                .accept_two_values_return_value(v1, v2, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn accept_two_values_return_value_Panics() {
            // Arrange
            let mut mock = Struct::new();
            let v1 = 10;
            let v2 = 20.2;
            let returned_value = String::from("veridis quo");
            mock.setup()
                .accept_two_values_return_value(Arg::Any, Arg::Any)
                .returns(returned_value.clone());

            // Act
            let actual_returned_value = mock.accept_two_values_return_value(v1, v2);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            assert_panics(
                || {
                    mock.received()
                        .accept_two_values_return_value(v1, v2, Times::Never)
                },
                format!(
                    r#"Expected to never receive a call matching:
	Struct::accept_two_values_return_value((i32): equal to {v1}, (f32): equal to {v2})
Actually received 1 matching call:
	accept_two_values_return_value({v1}, {v2})
Received no non-matching calls"#
                ),
            );

            assert_panics(
                || {
                    mock.received()
                        .accept_two_values_return_value(v1, v2, Times::Exactly(3))
                },
                format!(
                    r#"Expected to receive a call 3 times matching:
	Struct::accept_two_values_return_value((i32): equal to {v1}, (f32): equal to {v2})
Actually received 1 matching call:
	accept_two_values_return_value({v1}, {v2})
Received no non-matching calls"#
                ),
            );

            let invalid_expected_v1 = v1 + 1;
            let invalid_expected_v2 = v2 + 1.0;
            assert_panics(
                || {
                    mock.received().accept_two_values_return_value(
                        invalid_expected_v1,
                        invalid_expected_v2,
                        Times::Once,
                    )
                },
                format!(
                    r#"Expected to receive a call exactly once matching:
	Struct::accept_two_values_return_value((i32): equal to {invalid_expected_v1}, (f32): equal to {invalid_expected_v2})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_two_values_return_value(*10*, *20.2*)
	1. v1 (i32):
		Expected: 11
		Actual:   10
	2. v2 (f32):
		Expected: 21.2
		Actual:   20.2"#
                ),
            );
        }

        #[test]
        fn accept_two_values_return_value_NoReturnValue_Panics() {
            // Arrange
            let mut mock = Struct::new();
            let unexpected_v1 = 10;
            let unexpected_v2 = 22.2;
            let expected_v1 = 30;
            let expected_v2 = 44.4;
            mock.setup()
                .accept_two_values_return_value(unexpected_v1, unexpected_v2);
            mock.setup()
                .accept_two_values_return_value(expected_v1, expected_v2)
                .returns(String::from("should not be returned"));

            // Act
            let actual_error_msg =
                record_panic(|| mock.accept_two_values_return_value(unexpected_v1, unexpected_v2));

            // Assert
            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call because no return value was provided:
	Struct::accept_two_values_return_value({unexpected_v1}, {unexpected_v2})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/2 arguments: accept_two_values_return_value(*{unexpected_v1}*, *{unexpected_v2}*)"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
        }
    }
}
