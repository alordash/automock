use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{LazyLock, RwLock};

pub fn in_sequence(mut setups: impl FnMut()) -> Rc<Sequence> {
    let sequence = Sequence {
        call_numbers: Default::default(),
    };
    set_current_sequence(sequence);
    setups();
    extract_current_sequence()
}

pub struct Sequence {
    pub call_numbers: RefCell<Vec<usize>>,  // TODO - this should contain stuff like CallOrderEntry
}

impl Sequence {
    pub fn verify(&self) {
        dbg!(&self.call_numbers);
    }

    pub fn add_call_number(&self, call_number: usize) {
        self.call_numbers.borrow_mut().push(call_number)
    }
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
