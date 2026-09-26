use crate::args::arg_info::ArgInfo;

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub enum ArgCheckResult {
    Ok(ArgCheckResultOk),
    Err(ArgCheckResultErr),
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub struct ArgCheckResultOk {
    pub arg_info: ArgInfo,
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub struct ArgCheckResultErr {
    pub arg_info: ArgInfo,
    pub error_msg: String,
}

impl ArgCheckResult {
    pub fn is_ok(&self) -> bool {
        match self {
            ArgCheckResult::Ok(_) => true,
            _ => false,
        }
    }

    pub fn as_err(&self) -> Option<&ArgCheckResultErr> {
        match self {
            ArgCheckResult::Err(result) => Some(result),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn Ok_is_ok_ReturnsTrue() {
        // Arrange
        let arg_check_result = ArgCheckResult::Ok(ArgCheckResultOk {
            arg_info: ArgInfo::new("quo", &1, "vadis".to_owned()),
        });

        // Act
        let is_ok = arg_check_result.is_ok();

        // Assert
        assert!(is_ok);
    }

    #[test]
    fn Err_is_ok_ReturnsFalse() {
        // Arrange
        let arg_check_result = ArgCheckResult::Err(ArgCheckResultErr {
            arg_info: ArgInfo::new("quo", &1, "vadis".to_owned()),
            error_msg: "veridis quo".to_owned(),
        });

        // Act
        let is_ok = arg_check_result.is_ok();

        // Assert
        assert!(!is_ok);
    }

    #[test]
    fn Ok_as_err_ReturnsNone() {
        // Arrange
        let arg_check_result = ArgCheckResult::Ok(ArgCheckResultOk {
            arg_info: ArgInfo::new("quo", &1, "vadis".to_owned()),
        });

        // Act
        let err = arg_check_result.as_err();

        // Assert
        assert!(err.is_none());
    }

    #[test]
    fn Err_as_err_ReturnsSome() {
        // Arrange
        let arg_check_result = ArgCheckResult::Err(ArgCheckResultErr {
            arg_info: ArgInfo::new("quo", &1, "vadis".to_owned()),
            error_msg: "veridis quo".to_owned(),
        });

        // Act
        let err = arg_check_result.as_err();

        // Assert
        let arg_check_result_err = err.expect("Must be `Some(ArgCheckResultErr)`");
        let expected_arg_check_result_err = match &arg_check_result {
            ArgCheckResult::Err(e) => e,
            _ => panic!("Must be `ArgCheckResult::Err`"),
        };
        assert!(core::ptr::eq(
            arg_check_result_err,
            expected_arg_check_result_err
        ));
    }
}
