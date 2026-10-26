use super::call_order_entry::CallOrderEntry;
use crate::infrastructure::error_printing;
use std::cell::RefCell;

pub struct Sequence {
    pub(super) expected_calls_order: RefCell<Vec<CallOrderEntry>>,
}

impl Sequence {
    pub fn verify(&self) {
        let is_order_correct = self
            .expected_calls_order
            .borrow()
            .is_sorted_by(|a, b| a.call_order_number <= b.call_order_number);
        if !is_order_correct {
            panic_invalid_calls_order(core::mem::take(
                self.expected_calls_order.borrow_mut().as_mut(),
            ));
        }
    }

    pub fn add_call_number(&self, new_call_order_number: usize, call_formatted_string: String) {
        self.expected_calls_order.borrow_mut().push(CallOrderEntry {
            call_order_number: new_call_order_number,
            formatted_string: call_formatted_string,
        })
    }
}

#[cfg_attr(test, automock::mock)]
fn panic_invalid_calls_order(expected_calls_order: Vec<CallOrderEntry>) {
    error_printing::panic_invalid_calls_order(expected_calls_order);
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::record_panic;

    #[test]
    fn add_call_number_Ok() {
        // Arrange
        let sequence = Sequence {
            expected_calls_order: Default::default(),
        };
        let new_call_order_number = 5;
        let formatted_string = "quo vadis".to_owned();

        // Act
        sequence.add_call_number(new_call_order_number, formatted_string.clone());

        // Assert
        let calls = sequence.expected_calls_order.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].call_order_number, new_call_order_number);
        assert_eq!(calls[0].formatted_string, formatted_string);
    }

    #[test]
    fn verify_OrderIsCorrect_DoesNothing() {
        // Arrange
        let sequence = Sequence {
            expected_calls_order: RefCell::new(vec![
                CallOrderEntry {
                    call_order_number: 5,
                    formatted_string: "quo vadis".to_owned(),
                },
                CallOrderEntry {
                    call_order_number: 10,
                    formatted_string: "veridis quo".to_owned(),
                },
            ]),
        };

        // Act
        let panic_message = record_panic(|| sequence.verify());

        // Assert
        assert!(panic_message.is_none());
        assert_eq!(sequence.expected_calls_order.borrow().len(), 2);
    }

    #[test]
    fn verify_OrderIsIncorrect_PrintsError() {
        // Arrange
        let sequence = Sequence {
            expected_calls_order: RefCell::new(vec![
                CallOrderEntry {
                    call_order_number: 10,
                    formatted_string: "quo vadis".to_owned(),
                },
                CallOrderEntry {
                    call_order_number: 5,
                    formatted_string: "veridis quo".to_owned(),
                },
            ]),
        };
        panic_invalid_calls_order::setup(automock::Arg::Any).call_base();
        let expected_error = "Expected to receive these calls in order:\n\n\tveridis quo\n\tquo vadis\n\nActually received matching calls in this order:\n\n\tquo vadis\n\tveridis quo\n";

        // Act
        let panic_message = record_panic(|| sequence.verify());

        // Assert
        assert_eq!(panic_message, Some(expected_error.to_owned()));
    }
}
