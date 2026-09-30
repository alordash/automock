use crate::args::*;

#[doc(hidden)]
pub trait IntoArg<T> {
    fn into_arg(self, format_debug_string: impl Fn(&T) -> String) -> Arg<T>;
}

impl<T: PartialEq> IntoArg<T> for T {
    fn into_arg(self, format_debug_string: impl Fn(&T) -> String) -> Arg<T> {
        let print_arg = format_debug_string(&self);
        let arg_cmp = ArgCmp::new_eq(print_arg, self);
        return Arg::Eq(arg_cmp, Internal);
    }
}

impl<T> IntoArg<T> for Arg<T> {
    fn into_arg(mut self, format_debug_string: impl Fn(&T) -> String) -> Arg<T> {
        match &mut self {
            Arg::Eq(arg_cmp, _) | Arg::NotEq(arg_cmp, _) => {
                let print_arg = format_debug_string(arg_cmp.value());
                arg_cmp.set_print_arg(print_arg);
            }
            _ => (),
        }
        self
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use crate::args::arg_cmp::tests::utilities::*;
    use automock::Mockable;

    #[test]
    fn into_arg_RegularValue_Ok() {
        // Arrange
        type T = i32;
        let value: T = 5;
        let arg_cmp_mock = arg_cmp_mock::<T>();
        let arg_cmp_mock_id = arg_cmp_mock.id();
        ArgCmp::static_setup()
            .new_eq(automock::Arg::Any, automock::Arg::Any)
            .returns(arg_cmp_mock);
        let print_arg = "quo vadis".to_owned();
        let format_debug_string = |_: &T| print_arg.clone();

        // Act
        let result = value.into_arg(format_debug_string);

        // Assert
        let arg_cmp = match result {
            Arg::Eq(v, _) => v,
            _ => panic!("Expected result to be Arg::Eq, was instead: {result:?}"),
        };
        assert_eq!(arg_cmp.id(), arg_cmp_mock_id);

        ArgCmp::static_received()
            .new_eq(print_arg, value, automock::Times::Once)
            .no_other_calls();
    }
    
    // TODO - test Arg::into_arg
}
