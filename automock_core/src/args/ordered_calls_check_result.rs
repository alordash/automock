use crate::args::*;

#[derive(Clone, PartialEq)]
pub(crate) struct OrderedCallsCheckResult {
    pub calls_args_check_results: Vec<OrderedCallCheckResult>,
}

#[derive(Clone, PartialEq)]
pub(crate) struct OrderedCallCheckResult {
    pub call_order_number: usize,
    pub args_check_results: Vec<ArgCheckResult>,
}

