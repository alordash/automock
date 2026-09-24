use std::fmt::{Debug, Display, Formatter};

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub enum GenericParameterInfo {
    Type(GenericTypeInfo),
    Const(GenericConstInfo),
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub struct GenericTypeInfo {
    pub name: &'static str,
    pub type_name: &'static str,
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub struct GenericConstInfo {
    pub name: &'static str,
    pub debug_value_str: String,
}

#[doc(hidden)]
pub fn generic_type_info(name: &'static str, type_name: &'static str) -> GenericParameterInfo {
    let result = GenericParameterInfo::Type(GenericTypeInfo { name, type_name });
    return result;
}

#[doc(hidden)]
pub fn generic_const_info<T: Debug>(name: &'static str, value: T) -> GenericParameterInfo {
    let result = GenericParameterInfo::Const(GenericConstInfo {
        name,
        debug_value_str: format!("{value:?}"),
    });
    return result;
}

impl Display for GenericParameterInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericParameterInfo::Type(type_info) => {
                write!(f, "{}", type_info.type_name)
            }
            GenericParameterInfo::Const(const_info) => {
                write!(f, "{}", const_info.debug_value_str)
            }
        }
    }
}
