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

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn new_Ok() {
        // Arrange
        let calls_args_check_results = vec![
            vec![ArgCheckResult::Ok(ArgCheckResultOk {
                arg_info: ArgInfo::new("1", "one", "11".to_owned()),
            })],
            vec![ArgCheckResult::Err(ArgCheckResultErr {
                arg_info: ArgInfo::new("2", "two", "22".to_owned()),
                error_msg: "error msg".to_owned(),
            })],
        ];

        // Act
        let result = CallsCheckResult::new(calls_args_check_results.clone());

        // Assert
        assert_eq!(result.calls_args_check_results, calls_args_check_results);
    }

    #[test]
    fn empty_Ok() {
        // Act
        let result = CallsCheckResult::empty();

        // Assert
        assert!(result.calls_args_check_results.is_empty());
    }
}
