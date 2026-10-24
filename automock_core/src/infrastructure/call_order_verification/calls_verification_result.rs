use std::cell::RefCell;
use std::rc::Rc;
use crate::infrastructure::*;

pub struct CallsVerificationResult<'am, TMock> {
    pub formatted_fn_name: String,
    pub unused_configs: Vec<Rc<RefCell<FnConfig<'am, TMock>>>>,
}