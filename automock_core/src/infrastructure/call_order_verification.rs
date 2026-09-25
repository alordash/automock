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

pub(crate) struct CallOrderEntry {
    pub call_order_number: usize,
    pub formatted_string: String,
}

struct CallOrderState {
    pub perform_call_order_verification: Cell<bool>,
    pub expected_calls_order: RefCell<Vec<CallOrderEntry>>,
}

thread_local! {
    static CALL_ORDER_STATE: CallOrderState = const {CallOrderState {
        perform_call_order_verification: Cell::new(false),
        expected_calls_order: RefCell::new(Vec::new()),
    }};
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
            error_printing::panic_invalid_calls_order(
                x.expected_calls_order.borrow_mut().as_mut_slice(),
            );
        }
    });
}
