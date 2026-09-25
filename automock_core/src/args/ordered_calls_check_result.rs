use crate::args::*;

pub(crate) struct OrderedCallsCheckResult {
    pub calls_args_check_results: Vec<OrderedCallCheckResult>,
}

impl OrderedCallsCheckResult {
    pub fn new(calls_args_check_results: Vec<OrderedCallCheckResult>) -> Self {
        Self {
            calls_args_check_results,
        }
    }
}

pub(crate) struct OrderedCallCheckResult {
    pub call_order_number: usize,
    pub args_check_results: Vec<ArgCheckResult>,
}

impl OrderedCallCheckResult {
    pub fn new(call_order_number: usize, args_check_results: Vec<ArgCheckResult>) -> Self {
        Self {
            call_order_number,
            args_check_results,
        }
    }
}
