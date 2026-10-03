use crate::args::CallsCheckResult;
use crate::infrastructure::FnConfig;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) enum MatchingConfigSearchResult<'am, TMock> {
    Ok(Rc<RefCell<FnConfig<'am, TMock>>>),
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

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn empty_Ok() {
        // Act
        let result = MatchingConfigSearchErr::empty();

        // Assert
        assert!(
            result
                .args_check_results_sorted_by_number_of_correctly_matched_args_descending
                .calls_args_check_results
                .is_empty()
        );
        assert!(!result.needed_return_value)
    }
}
