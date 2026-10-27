use std::cell::RefCell;
use std::rc::Rc;

mod call_order_entry;
mod calls_verification_result;

use crate::infrastructure::error_printing;
pub(crate) use call_order_entry::CallOrderEntry;
pub(crate) use calls_verification_result::CallsVerificationResult;

pub fn in_sequence(setups: impl FnOnce()) -> Rc<Sequence> {
    let sequence = Sequence {
        expected_calls_order: Default::default(),
    };
    set_current_sequence(sequence);
    setups();
    extract_current_sequence()
}

thread_local! {
    static CURRENT_SEQUENCE: RefCell<Option<Rc<Sequence>>> = Default::default();
}

const CURRENT_SEQUENCE_IS_EMPTY_ERROR: &str =
    "[ERROR] Current sequence is empty but shouldn't be (sequence verification error).";

pub fn try_get_current_sequence() -> Option<Rc<Sequence>> {
    CURRENT_SEQUENCE.with(|x| x.borrow().clone())
}

fn set_current_sequence(sequence: Sequence) {
    CURRENT_SEQUENCE.with(|x| {
        *x.borrow_mut() = Some(Rc::new(sequence));
    });
}

fn extract_current_sequence() -> Rc<Sequence> {
    CURRENT_SEQUENCE.with(|x| {
        x.borrow_mut()
            .take()
            .expect(CURRENT_SEQUENCE_IS_EMPTY_ERROR)
    })
}

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
        sequence.verify();

        // Assert
        assert_eq!(sequence.expected_calls_order.borrow().len(), 2);
        panic_invalid_calls_order::verify_calls();
    }

    #[test]
    fn verify_OrderIsIncorrect_PrintsError() {
        // Arrange
        let expected_calls_order = vec![
            CallOrderEntry {
                call_order_number: 10,
                formatted_string: "quo vadis".to_owned(),
            },
            CallOrderEntry {
                call_order_number: 5,
                formatted_string: "veridis quo".to_owned(),
            },
        ];
        let sequence = Sequence {
            expected_calls_order: RefCell::new(expected_calls_order.clone()),
        };
        panic_invalid_calls_order::setup(expected_calls_order).expect_once();

        // Act
        sequence.verify();

        // Assert
        assert!(sequence.expected_calls_order.borrow().is_empty());
        panic_invalid_calls_order::verify_calls();
    }
}
