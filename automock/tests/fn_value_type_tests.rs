#![allow(clippy::arc_with_non_send_sync)]

use automock::*;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

thread_local! {
    pub static BASE_CALLED_FLAG: Cell<bool> = const {Cell::new(false)};
}

#[mock(base)]
fn accept_value(v: i32) {
    BASE_CALLED_FLAG.set(true);
}

const BASE_RETURN_VALUE: i32 = 12321;
#[mock(base)]
fn return_value() -> i32 {
    BASE_CALLED_FLAG.set(true);
    return BASE_RETURN_VALUE;
}

const BASE_ACCEPT_VALUE_RETURN_VALUE: f32 = 44.2;
#[mock(base)]
fn accept_value_return_value(v: i32) -> f32 {
    BASE_CALLED_FLAG.set(true);
    return BASE_ACCEPT_VALUE_RETURN_VALUE;
}

#[mock(base)]
fn accept_two_values(v1: i32, v2: f32) {
    BASE_CALLED_FLAG.set(true);
}

const BASE_ACCEPT_TWO_VALUES_RETURN_VALUE: &str = "quo vadis";
#[mock(base)]
fn accept_two_values_return_value(v1: i32, v2: f32) -> &'static str {
    BASE_CALLED_FLAG.set(true);
    return BASE_ACCEPT_TWO_VALUES_RETURN_VALUE;
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    mod accept_value_tests {
        use super::*;

        #[test]
        fn accept_value_Ok() {
            // Arrange
            let first_value = 10;
            let second_value = 22;

            // Act
            accept_value(first_value);
            accept_value(second_value);
            accept_value(second_value);

            // Assert
            accept_value::received(Arg::Any, Times::Exactly(3))
                .received(first_value, Times::Once)
                .received(
                    Arg::is(move |actual_value| *actual_value == first_value),
                    Times::Once,
                )
                .received(Arg::eq(second_value), Times::Exactly(2))
                .received(
                    Arg::is(move |actual_value| *actual_value == second_value),
                    Times::Exactly(2),
                )
                .no_other_calls();
        }

        #[test]
        fn accept_value_CallBase_Ok() {
            // Arrange
            let v = 18;
            BASE_CALLED_FLAG.set(false);
            accept_value::setup(Arg::Any).call_base();

            // Act
            accept_value(v);

            // Assert
            assert!(BASE_CALLED_FLAG.get());
            accept_value::received(v, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_value_Callback_Ok() {
            // Arrange
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            accept_value::setup(Arg::Any).does(move |_| *callback_flag_clone.borrow_mut() = true);

            // Act
            accept_value(1);

            // Assert
            assert!(*callback_flag.borrow());
        }

        #[test]
        fn accept_value_ArgAny_Panics() {
            // Arrange
            let first_value = 10;
            let second_value = 22;

            // Act
            accept_value(first_value);
            accept_value(second_value);

            // Assert
            assert_panics(
                || accept_value::received(Arg::Any, Times::Never),
                format!(
                    "Expected to never receive a call matching:
	accept_value((i32): any)
Actually received 2 matching calls:
	accept_value({first_value})
	accept_value({second_value})
Received no non-matching calls"
                ),
            );
            assert_panics(
                || accept_value::received(Arg::Any, Times::Once),
                format!(
                    "Expected to receive a call exactly once matching:
	accept_value((i32): any)
Actually received 2 matching calls:
	accept_value({first_value})
	accept_value({second_value})
Received no non-matching calls"
                ),
            );
            assert_panics(
                || accept_value::received(Arg::Any, Times::Exactly(3)),
                format!(
                    "Expected to receive a call 3 times matching:
	accept_value((i32): any)
Actually received 2 matching calls:
	accept_value({first_value})
	accept_value({second_value})
Received no non-matching calls"
                ),
            );
        }

        #[test]
        fn accept_value_ArgEq_Panics() {
            // Arrange
            let first_value = 10;
            let second_value = 22;

            // Act
            accept_value(first_value);
            accept_value(second_value);

            // Assert
            assert_panics(
                || accept_value::received(Arg::eq(first_value), Times::Never),
                format!(
                    "Expected to never receive a call matching:
	accept_value((i32): equal to {first_value})
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
	1. v (i32):
		Expected: {first_value}
		Actual:   {second_value}"
                ),
            );
            assert_panics(
                || accept_value::received(Arg::eq(first_value), Times::Exactly(3)),
                format!(
                    "Expected to receive a call 3 times matching:
	accept_value((i32): equal to {first_value})
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
	1. v (i32):
		Expected: {first_value}
		Actual:   {second_value}"
                ),
            );
            assert_panics(
                || accept_value::received(Arg::eq(second_value), Times::Never),
                format!(
                    "Expected to never receive a call matching:
	accept_value((i32): equal to {second_value})
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Expected: {second_value}
		Actual:   {first_value}"
                ),
            );
            assert_panics(
                || accept_value::received(Arg::eq(second_value), Times::Exactly(3)),
                format!(
                    "Expected to receive a call 3 times matching:
	accept_value((i32): equal to {second_value})
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Expected: {second_value}
		Actual:   {first_value}"
                ),
            );
        }

        #[test]
        fn accept_value_ArgIs_Panics() {
            // Arrange
            let first_value = 10;
            let second_value = 22;

            // Act
            accept_value(first_value);
            accept_value(second_value);

            // Assert
            assert_panics(
                || {
                    accept_value::received(
                        Arg::is(move |actual_value| *actual_value == first_value),
                        Times::Never,
                    )
                },
                format!(
                    "Expected to never receive a call matching:
	accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {second_value}"
                ),
            );
            assert_panics(
                || {
                    accept_value::received(
                        Arg::is(move |actual_value| *actual_value == first_value),
                        Times::Exactly(3),
                    )
                },
                format!(
                    "Expected to receive a call 3 times matching:
	accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {second_value}"
                ),
            );
            assert_panics(
                || {
                    accept_value::received(
                        Arg::is(move |actual_value| *actual_value == second_value),
                        Times::Never,
                    )
                },
                format!(
                    "Expected to never receive a call matching:
	accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {first_value}"
                ),
            );
            assert_panics(
                || {
                    accept_value::received(
                        Arg::is(move |actual_value| *actual_value == second_value),
                        Times::Exactly(3),
                    )
                },
                format!(
                    "Expected to receive a call 3 times matching:
	accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {first_value}"
                ),
            );
        }

        #[test]
        fn accept_value_ArgNotEq_Panics() {
            // Arrange
            let first_value = 10;
            let second_value = 22;

            // Act
            accept_value(first_value);
            accept_value(second_value);

            // Assert
            assert_panics(
                || accept_value::received(Arg::not_eq(first_value), Times::Never),
                format!(
                    "Expected to never receive a call matching:
	accept_value((i32): NOT equal to {first_value})
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Did not expect to be {first_value}"
                ),
            );
        }

        #[test]
        fn accept_value_NoOtherCallsWithoutOtherCalls_Ok() {
            // Arrange
            let value = 10;

            // Act
            accept_value(value);

            // Assert
            accept_value::received(value, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_value_NoOtherCallsWithOneOtherCall_Panics() {
            // Arrange
            let first_value = 10;
            let second_value = 22;

            // Act
            accept_value(first_value);
            accept_value(second_value);

            // Assert
            assert_panics(
                || accept_value::received(first_value, Times::Once).no_other_calls(),
                format!(
                    "Did not expect to receive any other calls. Received 1 unexpected call:
1. accept_value({second_value})"
                ),
            );
        }

        #[test]
        fn accept_value_NoOtherCallsWithManyOtherCalls_Panics() {
            // Arrange
            let first_value = 10;
            let second_value = 22;
            let third_value = 333;

            // Act
            accept_value(first_value);
            accept_value(second_value);
            accept_value(third_value);

            // Assert
            assert_panics(
                || accept_value::received(first_value, Times::Once).no_other_calls(),
                format!(
                    "Did not expect to receive any other calls. Received 2 unexpected calls:
1. accept_value({second_value})
2. accept_value({third_value})"
                ),
            );
        }
    }

    mod return_value_tests {
        use super::*;

        #[test]
        fn return_value_Single_Ok() {
            // Arrange
            let value = 10;
            return_value::setup().returns(value);

            // Act
            let actual_value = return_value();

            // Assert
            assert_eq!(value, actual_value);
        }

        #[test]
        fn return_value_CallBase_Ok() {
            // Arrange
            BASE_CALLED_FLAG.set(false);
            return_value::setup().call_base();

            // Act
            let actual_value = return_value();

            // Assert
            assert_eq!(BASE_RETURN_VALUE, actual_value);
            assert!(BASE_CALLED_FLAG.get());
            return_value::received(Times::Once).no_other_calls();
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

            let first_value = 10;
            let second_value = 22;
            let third_value = 333;
            let callback_result = Arc::new(RefCell::new(Result::DidNotChange));
            let first_callback_result = callback_result.clone();
            let second_callback_result = callback_result.clone();
            return_value::setup()
                .returns(first_value)
                .setup()
                .returns(second_value)
                .and_does(move |_| {
                    *first_callback_result.borrow_mut() = Result::SecondConfigChanged
                })
                .setup()
                .returns(third_value)
                .and_does(move |_| {
                    *second_callback_result.borrow_mut() = Result::ThirdConfigChanged
                });

            // Act
            let actual_first_value = return_value();
            let actual_second_value = return_value();
            let actual_third_value = return_value();

            // Assert
            assert_eq!(first_value, actual_first_value);
            assert_eq!(second_value, actual_second_value);
            assert_eq!(third_value, actual_third_value);
            assert_eq!(Result::ThirdConfigChanged, *callback_result.borrow());
        }

        #[test]
        fn return_value_Many_Ok() {
            // Arrange
            let first_value = 10;
            let second_value = 22;
            let third_value = 333;
            return_value::setup().returns_many([first_value, second_value, third_value]);

            // Act
            let actual_first_value = return_value();
            let actual_second_value = return_value();
            let actual_third_value = return_value();

            // Assert
            assert_eq!(first_value, actual_first_value);
            assert_eq!(second_value, actual_second_value);
            assert_eq!(third_value, actual_third_value);
        }

        #[test]
        fn return_value_ManyWithCallback_Ok() {
            // Arrange
            let callback_counter = Arc::new(RefCell::new(0));
            let callback_counter_clone = callback_counter.clone();
            let first_value = 10;
            let second_value = 22;
            return_value::setup()
                .returns_many([first_value, second_value])
                .and_does(move |_| *callback_counter_clone.borrow_mut() += 1);

            // Act
            let actual_first_value = return_value();
            let actual_second_value = return_value();

            // Assert
            assert_eq!(2, *callback_counter.borrow());

            assert_eq!(first_value, actual_first_value);
            assert_eq!(second_value, actual_second_value);
        }
    }

    mod accept_value_return_value_tests {
        use super::*;

        #[test]
        fn accept_value_return_value_Ok() {
            // Arrange
            let first_accepted_value = 10;
            let first_returned_value = 11.1;
            let second_accepted_value = 20;
            let second_returned_value = 22.2;
            let third_accepted_value = 30;
            let third_returned_value = 33.3;
            accept_value_return_value::setup(Arg::is(move |x| *x == first_accepted_value))
                .returns(first_returned_value)
                .setup(Arg::eq(second_accepted_value))
                .returns(second_returned_value)
                .setup(Arg::Any)
                .returns(third_returned_value);

            // Act
            let actual_first_returned_value = accept_value_return_value(first_accepted_value);
            let actual_second_returned_value = accept_value_return_value(second_accepted_value);
            let actual_third_returned_value = accept_value_return_value(third_accepted_value);

            // Assert
            assert_eq!(first_returned_value, actual_first_returned_value);
            assert_eq!(second_returned_value, actual_second_returned_value);
            assert_eq!(third_returned_value, actual_third_returned_value);

            accept_value_return_value::received(first_accepted_value, Times::Once)
                .received(second_accepted_value, Times::Once)
                .received(third_accepted_value, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn accept_value_return_value_CallBase_Ok() {
            // Arrange
            let v = 18;
            BASE_CALLED_FLAG.set(false);
            accept_value_return_value::setup(Arg::Any).call_base();

            // Act
            let actual_value = accept_value_return_value(v);

            // Assert
            assert_eq!(BASE_ACCEPT_VALUE_RETURN_VALUE, actual_value);
            assert!(BASE_CALLED_FLAG.get());

            accept_value_return_value::received(v, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_value_return_value_Many1_Ok() {
            // Arrange
            let single_accepted_value = 10;
            let double_accepted_value = 20;
            let first_returned_value = 11.1;
            let second_returned_value = 22.2;
            let third_returned_value = 33.3;

            accept_value_return_value::setup(Arg::Any).returns_many([
                first_returned_value,
                second_returned_value,
                third_returned_value,
            ]);

            // Act
            let actual_first_returned_value = accept_value_return_value(single_accepted_value);
            let actual_second_returned_value = accept_value_return_value(double_accepted_value);
            let actual_third_returned_value = accept_value_return_value(double_accepted_value);

            // Assert
            assert_eq!(first_returned_value, actual_first_returned_value);
            assert_eq!(second_returned_value, actual_second_returned_value);
            assert_eq!(third_returned_value, actual_third_returned_value);

            accept_value_return_value::received(single_accepted_value, Times::Once)
                .received(double_accepted_value, Times::Exactly(2))
                .no_other_calls();
        }

        #[test]
        fn accept_value_return_value_Many2_Ok() {
            // Arrange
            let first_accepted_value = 10;
            let first_first_returned_value = 11.1;
            let first_second_returned_value = 22.2;

            let second_accepted_value = 200;
            let second_first_returned_value = 201.1;
            let second_second_returned_value = 202.2;
            let second_third_returned_value = 203.3;

            accept_value_return_value::setup(Arg::eq(first_accepted_value))
                .returns_many([first_first_returned_value, first_second_returned_value])
                .setup(Arg::eq(second_accepted_value))
                .returns_many([
                    second_first_returned_value,
                    second_second_returned_value,
                    second_third_returned_value,
                ]);

            // Act
            let actual_first_first_returned_value = accept_value_return_value(first_accepted_value);
            let actual_first_second_returned_value =
                accept_value_return_value(first_accepted_value);

            let actual_second_first_returned_value =
                accept_value_return_value(second_accepted_value);
            let actual_second_second_returned_value =
                accept_value_return_value(second_accepted_value);
            let actual_second_third_returned_value =
                accept_value_return_value(second_accepted_value);

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

            accept_value_return_value::received(first_accepted_value, Times::Exactly(2))
                .received(second_accepted_value, Times::Exactly(3))
                .no_other_calls();
        }

        #[test]
        fn accept_value_return_value_Callback_Ok() {
            // Arrange
            let first_accepted_value = 10;
            let first_callback_number = Arc::new(RefCell::new(0));
            let first_callback_number_clone = first_callback_number.clone();
            let first_returned_value = 11.1;
            let second_accepted_value = 20;
            let second_callback_number = Arc::new(RefCell::new(1));
            let second_callback_number_clone = second_callback_number.clone();
            let second_returned_value = 22.2;
            accept_value_return_value::setup(Arg::eq(first_accepted_value))
                .returns(first_returned_value)
                .and_does(move |_| {
                    *first_callback_number_clone.borrow_mut() = 1;
                })
                .setup(Arg::eq(second_accepted_value))
                .returns(second_returned_value)
                .and_does(move |_| {
                    *second_callback_number_clone.borrow_mut() = 2;
                });

            // Act
            let actual_first_returned_value = accept_value_return_value(first_accepted_value);
            let actual_second_returned_value = accept_value_return_value(second_accepted_value);

            // Assert
            assert_eq!(first_returned_value, actual_first_returned_value);
            assert_eq!(second_returned_value, actual_second_returned_value);

            assert_eq!(1, *first_callback_number.borrow());
            assert_eq!(2, *second_callback_number.borrow());

            accept_value_return_value::received(first_accepted_value, Times::Once)
                .received(second_accepted_value, Times::Once)
                .no_other_calls();
        }
    }

    mod accept_two_values_tests {
        use super::*;

        #[test]
        fn accept_two_values_Ok() {
            // Arrange
            let v1 = 10;
            let v2 = 20.2;

            // Act
            accept_two_values(v1, v2);

            // Assert
            accept_two_values::received(v1, v2, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_two_value_CallBase_Ok() {
            // Arrange
            let v1 = 1;
            let v2 = 2.2;
            BASE_CALLED_FLAG.set(false);
            accept_two_values::setup(Arg::Any, Arg::Any).call_base();

            // Act
            accept_two_values(v1, v2);

            // Assert
            assert!(BASE_CALLED_FLAG.get());
            accept_two_values::received(v1, v2, Times::Once).no_other_calls();
        }
    }

    mod accept_two_values_return_value_tests {
        use super::*;

        #[test]
        fn accept_two_values_return_value_Ok() {
            // Arrange
            let v1 = 10;
            let v2 = 20.2;
            let returned_value = "quo vadis";
            accept_two_values_return_value::setup(v1, v2).returns(returned_value);

            // Act
            let actual_returned_value = accept_two_values_return_value(v1, v2);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            accept_two_values_return_value::received(v1, v2, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_two_value_return_value_Panics() {
            // Arrange
            let v1 = 10;
            let v2 = 20.2;
            let returned_value = "veridis quo";
            accept_two_values_return_value::setup(Arg::Any, Arg::Any).returns(returned_value);

            // Act
            let actual_returned_value = accept_two_values_return_value(v1, v2);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            assert_panics(
                || accept_two_values_return_value::received(v1, v2, Times::Never),
                format!(
                    "Expected to never receive a call matching:
	accept_two_values_return_value((i32): equal to {v1}, (f32): equal to {v2})
Actually received 1 matching call:
	accept_two_values_return_value({v1}, {v2})
Received no non-matching calls"
                ),
            );

            assert_panics(
                || accept_two_values_return_value::received(v1, v2, Times::Exactly(3)),
                format!(
                    "Expected to receive a call 3 times matching:
	accept_two_values_return_value((i32): equal to {v1}, (f32): equal to {v2})
Actually received 1 matching call:
	accept_two_values_return_value({v1}, {v2})
Received no non-matching calls"
                ),
            );

            let invalid_expected_v1 = v1 + 1;
            let invalid_expected_v2 = v2 + 1.0;
            assert_panics(
                || {
                    accept_two_values_return_value::received(
                        invalid_expected_v1,
                        invalid_expected_v2,
                        Times::Once,
                    )
                },
                format!(
                    "Expected to receive a call exactly once matching:
	accept_two_values_return_value((i32): equal to {invalid_expected_v1}, (f32): equal to {invalid_expected_v2})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_two_values_return_value(*10*, *20.2*)
	1. v1 (i32):
		Expected: 11
		Actual:   10
	2. v2 (f32):
		Expected: 21.2
		Actual:   20.2"
                ),
            );
        }

        #[test]
        fn accept_two_values_return_value_CallBase_Ok() {
            // Arrange
            let v1 = 10;
            let v2 = 20.2;
            BASE_CALLED_FLAG.set(false);
            accept_two_values_return_value::setup(Arg::Any, Arg::Any).call_base();

            // Act
            let actual_returned_value = accept_two_values_return_value(v1, v2);

            // Assert
            assert_eq!(BASE_ACCEPT_TWO_VALUES_RETURN_VALUE, actual_returned_value);
            assert!(BASE_CALLED_FLAG.get());

            accept_two_values_return_value::received(v1, v2, Times::Once).no_other_calls();
        }
    }
}
