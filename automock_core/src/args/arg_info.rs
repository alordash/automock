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

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn new_Ok() {
        // Arrange
        let arg_name = "quo vadis";
        type T = CustomType;
        let arg_debug_string = "veridis quo".to_owned();

        // Act
        let result = ArgInfo::new::<T>(arg_name, &CustomType, arg_debug_string.clone());

        // Assert
        let expected_arg_type_name = std::any::type_name::<T>();
        assert_eq!(result.arg_name, arg_name);
        assert_eq!(result.arg_type_name, expected_arg_type_name);
        assert_eq!(result.arg_debug_string, arg_debug_string);
    }

    #[test]
    fn arg_name_Ok() {
        // Arrange
        let arg_name = "quo vadis";
        let arg_info = ArgInfo {
            arg_name,
            arg_type_name: "veridis quo",
            arg_debug_string: "whatever".to_owned(),
        };

        // Act
        let result = arg_info.arg_name();

        // Assert
        assert_eq!(result, arg_name);
    }

    #[test]
    fn arg_type_name_Ok() {
        // Arrange
        let arg_type_name = "veridis quo";
        let arg_info = ArgInfo {
            arg_name: "quo vadis",
            arg_type_name,
            arg_debug_string: "whatever".to_owned(),
        };

        // Act
        let result = arg_info.arg_type_name();

        // Assert
        assert_eq!(result, arg_type_name);
    }

    #[test]
    fn clone_arg_debug_string_Ok() {
        // Arrange
        let arg_debug_string = "veridis quo";
        let arg_info = ArgInfo {
            arg_name: "quo vadis",
            arg_type_name: "veridis quo",
            arg_debug_string: arg_debug_string.to_owned(),
        };

        // Act
        let result = arg_info.clone_arg_debug_string();

        // Assert
        assert_eq!(result, arg_debug_string);
    }

    struct CustomType;
}
