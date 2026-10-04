use crate::infrastructure::error_printing;
use std::cell::{Cell, RefCell};

/// Checks that `received()` assertions inside `verifications` are performed sequentially relative
/// to each other.
pub fn verify_call_order(mut verifications: impl FnMut()) {
    enable();
    verifications();
    disable();
    validate_actual_calls_order();
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CallOrderEntry {
    pub call_order_number: usize,
    pub formatted_string: String,
}

struct CallOrderState {
    pub perform_call_order_verification: Cell<bool>,
    pub expected_calls_order: RefCell<Vec<CallOrderEntry>>,
}

thread_local! {
    static CALL_ORDER_STATE: CallOrderState = const {
        CallOrderState {
            perform_call_order_verification: Cell::new(false),
            expected_calls_order: RefCell::new(Vec::new()),
        }
    };
}

pub(crate) fn should_perform() -> bool {
    CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.get())
}

pub(crate) fn add_call(new_call_order_number: usize, call_formatted_string: String) {
    CALL_ORDER_STATE.with(|x| {
        x.expected_calls_order.borrow_mut().push(CallOrderEntry {
            call_order_number: new_call_order_number,
            formatted_string: call_formatted_string,
        });
    })
}

fn enable() {
    CALL_ORDER_STATE.with(|x| {
        x.expected_calls_order.borrow_mut().clear();
        x.perform_call_order_verification.set(true)
    })
}

fn disable() {
    CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.set(false))
}

fn validate_actual_calls_order() {
    CALL_ORDER_STATE.with(|x| {
        let is_order_correct = x
            .expected_calls_order
            .borrow()
            .is_sorted_by(|a, b| a.call_order_number < b.call_order_number);
        if !is_order_correct {
            panic_invalid_calls_order(x.expected_calls_order.borrow_mut().as_mut_slice());
        }
    });
}

#[cfg_attr(test, automock::mock)]
fn panic_invalid_calls_order(expected_calls_order: &mut [CallOrderEntry]) {
    error_printing::panic_invalid_calls_order(expected_calls_order);
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn should_perform_Ok() {
        // Arrange
        let should = true;
        CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.set(should));

        // Act
        let result = should_perform();

        // Assert
        assert_eq!(result, should);
    }

    #[test]
    fn add_call_Ok() {
        // Arrange
        let initial_calls_length = CALL_ORDER_STATE.with(|x| x.expected_calls_order.borrow().len());
        let new_call_order_number = 5usize;
        let call_formatted_string = "quo vadis".to_owned();

        // Act
        add_call(new_call_order_number, call_formatted_string.clone());

        // Act
        CALL_ORDER_STATE.with(|x| {
            let expected_calls_order = x.expected_calls_order.borrow();
            assert_eq!(expected_calls_order.len(), initial_calls_length + 1);

            let new_call_order_entry = &expected_calls_order[initial_calls_length];
            assert_eq!(
                new_call_order_entry.call_order_number,
                new_call_order_number
            );
            assert_eq!(new_call_order_entry.formatted_string, call_formatted_string);
        });
    }

    #[test]
    fn enable_Ok() {
        // Arrange
        CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.set(false));

        // Act
        enable();

        // Assert
        CALL_ORDER_STATE.with(|x| assert!(x.perform_call_order_verification.get()));
    }

    #[test]
    fn disable_Ok() {
        // Arrange
        CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.set(true));

        // Act
        disable();

        // Assert
        CALL_ORDER_STATE.with(|x| assert!(!x.perform_call_order_verification.get()));
    }

    #[test]
    fn validate_actual_calls_order_OrderIsCorrect_DoesNothing() {
        // Arrange
        let calls = vec![
            CallOrderEntry {
                call_order_number: 5,
                formatted_string: "quo vadis".to_owned(),
            },
            CallOrderEntry {
                call_order_number: 10,
                formatted_string: "veridis quo".to_owned(),
            },
        ];
        CALL_ORDER_STATE.with(|x| *x.expected_calls_order.borrow_mut() = calls);

        // Act
        validate_actual_calls_order();

        // Assert
        panic_invalid_calls_order::received_nothing();
    }

    #[test]
    fn validate_actual_calls_order_OrderIsIncorrect_PrintsError() {
        // Arrange
        let calls = vec![
            CallOrderEntry {
                call_order_number: 10,
                formatted_string: "quo vadis".to_owned(),
            },
            CallOrderEntry {
                call_order_number: 5,
                formatted_string: "veridis quo".to_owned(),
            },
        ];
        CALL_ORDER_STATE.with(|x| *x.expected_calls_order.borrow_mut() = calls);

        // Act
        validate_actual_calls_order();

        // Arrange
        CALL_ORDER_STATE.with(|x| {
            let mut mut_calls = x.expected_calls_order.borrow_mut();
            panic_invalid_calls_order::received(
                automock::Arg::ref_eq(mut_calls.as_mut_slice()),
                automock::Times::Once,
            )
            .no_other_calls();
        });
    }
}
