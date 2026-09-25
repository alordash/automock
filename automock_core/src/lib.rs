#![cfg_attr(feature = "debug_naming", allow(incomplete_features))]
#![cfg_attr(feature = "debug_naming", allow(unstable_features))]
#![cfg_attr(feature = "debug_naming", feature(specialization))]

pub mod args;
pub mod fn_parameters;
pub mod infrastructure;

mod lifetimes_transmutation;
pub mod settings;
pub mod times;

pub use infrastructure::verify_call_order;
