use crate::infrastructure::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CallsVerificationResult<'am, TMock> {
    pub formatted_fn_name: String,
    pub unused_configs: Vec<Rc<RefCell<FnConfig<'am, TMock>>>>,
}
