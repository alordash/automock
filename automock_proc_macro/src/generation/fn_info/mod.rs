pub mod models {
    mod args_checker_struct;
    mod call_struct;
    mod fn_info;

    pub(crate) use args_checker_struct::*;
    pub(crate) use call_struct::*;
    pub(crate) use fn_info::*;
}

mod common;
mod generation;

pub(crate) use common::*;
pub(crate) use generation::*;

pub mod args_checker_struct;
pub mod call_struct;
