use crate::args::*;

pub(crate) struct CallsCheckResult {
    pub calls_args_check_results: Vec<Vec<ArgCheckResult>>,
}

impl CallsCheckResult {
    pub fn new(calls_args_check_results: Vec<Vec<ArgCheckResult>>) -> Self {
        Self {
            calls_args_check_results,
        }
    }

    pub fn empty() -> Self {
        Self {
            calls_args_check_results: Vec::new(),
        }
    }
}
