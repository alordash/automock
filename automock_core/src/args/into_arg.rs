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
    use crate::args::arg::tests::utilities::*;
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
        format_debug_string::setup::<T>(automock::Arg::Any).returns(print_arg.clone());

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

    #[test]
    fn into_arg_ArgAny_DoesNothing() {
        // Arrange
        type T = i32;
        let arg = Arg::<T>::Any;

        // Act
        let result = arg.into_arg(format_debug_string);

        // Assert
        match result {
            Arg::Any => (),
            _ => panic!("Expected result to be `Arg::Any`, was instead: {result:?}"),
        }
    }

    #[test]
    fn into_arg_ArgEq_SetsPrintArg() {
        // Arrange
        type T = i32;
        let value: T = 5;
        let mut arg_cmp_mock = arg_cmp_mock::<T>();
        arg_cmp_mock.setup().value().returns(&value);
        let arg = Arg::Eq(arg_cmp_mock, Internal);
        let print_arg = "quo vadis".to_owned();
        format_debug_string::setup::<T>(automock::Arg::Any).returns(print_arg.clone());

        // Act
        let result = arg.into_arg(format_debug_string);

        // Assert
        let mut arg_cmp = match result {
            Arg::Eq(arg_cmp, _) => arg_cmp,
            _ => panic!("Expected result to be `Arg::Eq`, was instead: {result:?}"),
        };
        arg_cmp
            .received()
            .value(automock::Times::Once)
            .set_print_arg(print_arg, automock::Times::Once)
            .no_other_calls();

        format_debug_string::received(automock::Arg::ref_eq(&value), automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn into_arg_ArgNotEq_SetsPrintArg() {
        // Arrange
        type T = i32;
        let value: T = 5;
        let mut arg_cmp_mock = arg_cmp_mock::<T>();
        arg_cmp_mock.setup().value().returns(&value);
        let arg = Arg::NotEq(arg_cmp_mock, Internal);
        let print_arg = "quo vadis".to_owned();
        format_debug_string::setup::<T>(automock::Arg::Any).returns(print_arg.clone());

        // Act
        let result = arg.into_arg(format_debug_string);

        // Assert
        let mut arg_cmp = match result {
            Arg::NotEq(arg_cmp, _) => arg_cmp,
            _ => panic!("Expected result to be `Arg::NotEq`, was instead: {result:?}"),
        };
        arg_cmp
            .received()
            .value(automock::Times::Once)
            .set_print_arg(print_arg, automock::Times::Once)
            .no_other_calls();

        format_debug_string::received(automock::Arg::ref_eq(&value), automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn into_arg_ArgIs_DoesNothing() {
        // Arrange
        type T = i32;
        let arg = Arg::<T>::Is(Box::new(ptr_predicate), Internal);

        // Act
        let result = arg.into_arg(format_debug_string);

        // Assert
        match result {
            Arg::Is(_, _) => (),
            _ => panic!("Expected result to be `Arg::Any`, was instead: {result:?}"),
        }

        ptr_predicate::received_nothing();
    }

    #[automock::mock]
    fn format_debug_string<T>(_: &T) -> String {
        unreachable!()
    }
}
