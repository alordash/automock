use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{LazyLock, RwLock};
use super::call_order_verification::CallOrderEntry;
use super::error_printing;

pub fn in_sequence(mut setups: impl FnMut()) -> Rc<Sequence> {
    let sequence = Sequence {
        expected_calls_order: Default::default(),
    };
    set_current_sequence(sequence);
    setups();
    extract_current_sequence()
}

pub struct Sequence {
    pub expected_calls_order: RefCell<Vec<CallOrderEntry>>,
}

impl Sequence {
    pub fn verify(&self) {
        dbg!(self.expected_calls_order.borrow());
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

thread_local! {
    static CURRENT_SEQUENCE: LazyLock<RwLock<Option<Rc<Sequence>>>> = LazyLock::new(Default::default);
}

pub fn try_get_current_sequence() -> Option<Rc<Sequence>> {
    CURRENT_SEQUENCE.with(|x| {
        x.read()
            .expect("TODO generic message bla-bla can not read")
            .clone()
    })
}

fn set_current_sequence(sequence: Sequence) {
    CURRENT_SEQUENCE.with(|x| {
        *x.write()
            .expect("TODO generic message bla-bla can not write") = Some(Rc::new(sequence));
    });
}

fn extract_current_sequence() -> Rc<Sequence> {
    CURRENT_SEQUENCE.with(|x| {
        x.write()
            .expect("TODO generic message bla-bla can not write")
            .take()
            .expect("TODO error - sequence should be present")
    })
}
