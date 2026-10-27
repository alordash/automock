use crate::infrastructure::*;

#[doc(hidden)]
pub trait IMockGenericsInfoProvider {
    fn get_generic_parameter_infos() -> Vec<GenericParameterInfo>;
}
