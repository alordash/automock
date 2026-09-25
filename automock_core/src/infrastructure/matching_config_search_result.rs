use crate::args::CallsCheckResult;
use crate::infrastructure::FnConfig;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) enum MatchingConfigSearchResult<'rs, TMock> {
    Ok(Rc<RefCell<FnConfig<'rs, TMock>>>),
    Err(MatchingConfigSearchErr),
}

pub(crate) struct MatchingConfigSearchErr {
    pub args_check_results_sorted_by_number_of_correctly_matched_args_descending: CallsCheckResult,
    pub needed_return_value: bool,
}

impl MatchingConfigSearchErr {
    pub fn empty() -> Self {
        Self {
            args_check_results_sorted_by_number_of_correctly_matched_args_descending:
                CallsCheckResult::empty(),
            needed_return_value: false,
        }
    }
}
