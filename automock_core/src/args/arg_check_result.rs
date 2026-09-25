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
