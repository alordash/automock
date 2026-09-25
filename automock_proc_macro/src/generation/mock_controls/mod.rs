pub mod models {
    mod control_type;
    mod received;
    mod setup;
    mod static_control_type;
    mod static_received;
    mod static_setup;

    pub(crate) use control_type::*;
    pub(crate) use received::*;
    pub(crate) use setup::*;
    pub(crate) use static_control_type::*;
    pub(crate) use static_received::*;
    pub(crate) use static_setup::*;
}

mod common {
    pub(crate) mod args_checker_stmt;
    pub(crate) mod control_struct;
    pub(crate) mod fn_configurator_path;
    pub(crate) mod times_arg;
}

use common::*;

pub mod fn_static_received;
pub mod fn_static_received_nothing;
pub mod fn_static_setup;
pub mod received;
pub mod received_impl;
pub mod setup;
pub mod setup_impl;
pub mod static_received;
pub mod static_setup;
