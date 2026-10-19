use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{LazyLock, RwLock};
use crate::infrastructure::*;

pub fn in_sequence(mut setups: impl FnMut()) -> Rc<Sequence> {
    setups();
    Rc::new(Sequence)
}

pub struct Sequence;

impl Sequence {
    pub fn verify(&self) {}
}

thread_local! {
    static CURRENT_SEQUENCE: LazyLock<RwLock<Option<Rc<Sequence>>>> = LazyLock::new(Default::default);
}

