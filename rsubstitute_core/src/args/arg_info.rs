#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub struct ArgInfo {
    arg_name: &'static str,
    arg_type_name: &'static str,
    arg_debug_string: String,
}

impl ArgInfo {
    pub fn new<T: ?Sized>(
        arg_name: &'static str,
        _arg_value: &T,
        arg_debug_string: String,
    ) -> Self {
        let arg_type_name = std::any::type_name::<T>();
        return Self {
            arg_name,
            arg_type_name,
            arg_debug_string,
        };
    }

    pub fn arg_name(&self) -> &'static str {
        self.arg_name
    }

    pub fn arg_type_name(&self) -> &'static str {
        self.arg_type_name
    }

    pub fn clone_arg_debug_string(&self) -> String {
        self.arg_debug_string.to_owned()
    }
}
