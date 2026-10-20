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
            .is_sorted_by(|a, b| a.call_order_number < b.call_order_number);
        if !is_order_correct {
            panic_invalid_calls_order(self.expected_calls_order.borrow_mut().as_mut_slice());
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
fn panic_invalid_calls_order(expected_calls_order: &mut [CallOrderEntry]) {
    error_printing::panic_invalid_calls_order(expected_calls_order);
}
