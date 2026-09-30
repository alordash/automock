use crate::args::*;

pub(crate) struct OrderedCallsCheckResult {
    pub calls_args_check_results: Vec<OrderedCallCheckResult>,
}

pub(crate) struct OrderedCallCheckResult {
    pub call_order_number: usize,
    pub args_check_results: Vec<ArgCheckResult>,
}

