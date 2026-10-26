use std::cell::RefCell;
use std::rc::Rc;

mod call_order_entry;
mod calls_verification_result;
mod sequence;

pub(crate) use call_order_entry::CallOrderEntry;
pub(crate) use calls_verification_result::CallsVerificationResult;
pub(crate) use sequence::Sequence;

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
