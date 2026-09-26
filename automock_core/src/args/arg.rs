use crate::args::*;
use crate::transmute_lifetime;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub(crate) struct Internal;

/// Argument matcher, checks whether certain argument value matches some expectation.
///
/// `T` - type of argument.
/// #[
#[allow(private_interfaces)]
#[repr(C)]
pub enum Arg<T: ?Sized> {
    /// Accepts any possible value.
    Any,
    #[doc(hidden)]
    Eq(ArgCmp<T>, Internal),
    #[doc(hidden)]
    NotEq(ArgCmp<T>, Internal),
    #[doc(hidden)]
    Is(Box<dyn Fn(*const ()) -> bool>, Internal),
}

impl<T: Debug> Debug for Arg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO - extract to const field when std::any::type_name becomes stabilized as const fn_info
        // https://github.com/rust-lang/rust/issues/63084
        let arg_type_name = std::any::type_name::<T>();
        match self {
            Arg::Any => write!(f, "({arg_type_name}): any"),
            Arg::Eq(arg_cmp, _) => {
                write!(f, "({}): equal to {:?}", arg_type_name, arg_cmp.value())
            }
            Arg::NotEq(arg_cmp, _) => {
                write!(f, "({}): NOT equal to {:?}", arg_type_name, arg_cmp.value())
            }
            Arg::Is(_, _) => write!(f, "({arg_type_name}): custom predicate"),
        }
    }
}

const UNINITIALIZED_ARG_PRINT_STRING: &str = "[CRITICAL ERROR]: This string should represent arguments value, but if you see this it means that `ArgCmp.print_arg` was not initialized!";

impl<T> Arg<T> {
    /// Checks that argument value matches some predicate.
    pub fn is<'a, TFn: Fn(&T) -> bool + 'a>(predicate: TFn) -> Self {
        let anonymous_predicate = move |ptr: *const ()| {
            // SAFETY: anonymous predicate is called only internally and passed pointer is always
            // created by casting &T.
            let t_ref = unsafe {
                let t_ptr = ptr as *const T;
                t_ptr
                    .as_ref()
                    .expect("Pointer to argument in Arg::is must not be null.")
            };
            return predicate(t_ref);
        };
        let boxed_anonymous_predicate =
            Box::new(anonymous_predicate) as Box<dyn Fn(*const ()) -> bool + 'a>;
        return Self::Is(transmute_lifetime!(boxed_anonymous_predicate), Internal);
    }

    /// Checks that argument value is equal to given value.
    pub fn eq(value: T) -> Self
    where
        T: PartialEq,
    {
        let arg_cmp = ArgCmp::new_eq(value, UNINITIALIZED_ARG_PRINT_STRING.to_owned());
        return Self::Eq(arg_cmp, Internal);
    }

    /// Checks that argument value is NOT equal to given value.
    pub fn not_eq(value: T) -> Self
    where
        T: PartialEq,
    {
        let arg_cmp = ArgCmp::new_eq(value, UNINITIALIZED_ARG_PRINT_STRING.to_owned());
        return Self::NotEq(arg_cmp, Internal);
    }

    /// Checks that reference of argument value is equal to reference of given value.
    ///
    /// Reference is acquired from [`Deref::deref`].
    pub fn ref_eq<U: ?Sized>(value: T) -> Self
    where
        T: Deref<Target = U>,
    {
        let arg_cmp = ArgCmp::new_ref_eq(value, UNINITIALIZED_ARG_PRINT_STRING.to_owned());
        return Self::Eq(arg_cmp, Internal);
    }

    /// Checks that reference of argument value is NOT equal to reference of given value.
    ///
    /// Reference is acquired from [`Deref::deref`].
    pub fn ref_not_eq<U: ?Sized>(value: T) -> Self
    where
        T: Deref<Target = U>,
    {
        let arg_cmp = ArgCmp::new_ref_eq(value, UNINITIALIZED_ARG_PRINT_STRING.to_owned());
        return Self::NotEq(arg_cmp, Internal);
    }
}

impl<T: ?Sized> Arg<T> {
    #[doc(hidden)]
    pub fn check<'a>(
        &self,
        arg_name: &'static str,
        actual_value: &T,
        actual_value_str: String,
    ) -> ArgCheckResult
    where
        T: 'a,
    {
        let arg_info = ArgInfo::new(arg_name, actual_value, actual_value_str.clone());
        match self {
            Arg::Any => (),
            Arg::Eq(arg_cmp, _) => {
                if !arg_cmp.is_arg_equal_to(actual_value) {
                    let expected_value_str = arg_cmp.print_arg();
                    let PtrInfo {
                        expected_ptr_info_suffix,
                        actual_ptr_info_suffix,
                    } = arg_cmp.get_ptrs_info_suffix(actual_value);
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected{expected_ptr_info_suffix}: {expected_value_str}\n\t\tActual{actual_ptr_info_suffix} {actual_value_str}"
                        ),
                    });
                }
            }
            Arg::NotEq(arg_cmp, _) => {
                if arg_cmp.is_arg_equal_to(actual_value) {
                    let not_expected_value_str = arg_cmp.print_arg();
                    let PtrInfo {
                        expected_ptr_info_suffix,
                        ..
                    } = arg_cmp.get_ptrs_info_suffix(actual_value);
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tDid not expect to be {expected_ptr_info_suffix}{not_expected_value_str}"
                        ),
                    });
                }
            }
            Arg::Is(predicate, _) => {
                if !predicate(actual_value as *const _ as *const ()) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate did not match passed value, received: {actual_value_str}"
                        ),
                    });
                }
            }
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use arg_cmp::tests::*;
    use automock::Mockable;
    use std::rc::Rc;
    use utilities::*;

    #[test]
    fn Debug_fmt_Any_Ok() {
        // Arrange
        let arg = Arg::<CustomType>::Any;

        // Act
        let result = format!("{arg:?}");

        // Assert
        let expected = format!("({}): any", *CUSTOM_TYPE_NAME);
        assert_eq!(result, expected);
    }

    #[test]
    fn Debug_fmt_Eq_Ok() {
        // Arrange
        let mut arg_cmp_mock = arg_cmp_mock();
        let custom_type = CustomType(5);
        arg_cmp_mock.setup().value().returns(&custom_type);
        let arg = Arg::Eq(arg_cmp_mock, Internal);

        // Act
        let result = format!("{arg:?}");

        // Assert
        let expected = format!("({}): equal to {:?}", *CUSTOM_TYPE_NAME, custom_type);
        assert_eq!(result, expected);
    }

    #[test]
    fn Debug_fmt_NotEq_Ok() {
        // Arrange
        let mut arg_cmp_mock = arg_cmp_mock();
        let custom_type = CustomType(5);
        arg_cmp_mock.setup().value().returns(&custom_type);
        let arg = Arg::NotEq(arg_cmp_mock, Internal);

        // Act
        let result = format!("{arg:?}");

        // Assert
        let expected = format!("({}): NOT equal to {:?}", *CUSTOM_TYPE_NAME, custom_type);
        assert_eq!(result, expected);
    }

    #[test]
    fn Debug_fmt_Is_Ok() {
        // Arrange
        let arg = Arg::<CustomType>::Is(Box::new(|_| true), Internal);

        // Act
        let result = format!("{arg:?}");

        // Assert
        let expected = format!("({}): custom predicate", *CUSTOM_TYPE_NAME);
        assert_eq!(result, expected);
    }

    #[test]
    fn is_Ok() {
        // Arrange
        predicate::setup(automock::Arg::Any).returns(true);

        // Act
        let arg = Arg::is(predicate);

        // Assert
        let actual_predicate = match arg {
            Arg::Is(p, _) => p,
            _ => panic!("`arg` must be `Arg::is`, instead is: {arg:?}"),
        };
        let actual_predicate_result = actual_predicate(&CustomType(5) as *const _ as *const ());
        assert!(actual_predicate_result);

        predicate::received(automock::Arg::Any, automock::Times::Once);
    }

    #[test]
    fn eq_Ok() {
        // Arrange
        ArgCmp::<CustomType>::static_setup()
            .new_eq(automock::Arg::Any, automock::Arg::Any)
            .returns(arg_cmp_mock());
        let custom_type = CustomType(5);

        // Act
        let arg = Arg::eq(custom_type.clone());

        // Assert
        match arg {
            Arg::Eq(_, _) => (),
            _ => panic!("`arg` must be `Arg::Eq`, instead is: {arg:?}"),
        };

        ArgCmp::static_received()
            .new_eq(
                custom_type,
                UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn not_eq_Ok() {
        // Arrange
        ArgCmp::<CustomType>::static_setup()
            .new_eq(automock::Arg::Any, automock::Arg::Any)
            .returns(arg_cmp_mock());
        let custom_type = CustomType(5);

        // Act
        let arg = Arg::not_eq(custom_type.clone());

        // Assert
        match arg {
            Arg::NotEq(_, _) => (),
            _ => panic!("`arg` must be `Arg::NotEq`, instead is: {arg:?}"),
        };

        ArgCmp::static_received()
            .new_eq(
                custom_type,
                UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn ref_eq_Ok() {
        // Arrange
        ArgCmp::<Rc<CustomType>>::static_setup()
            .new_ref_eq(automock::Arg::Any, automock::Arg::Any)
            .returns(arg_cmp_mock());
        let custom_type = Rc::new(CustomType(5));

        // Act
        let arg = Arg::ref_eq(custom_type.clone());

        // Assert
        match arg {
            Arg::Eq(_, _) => (),
            _ => panic!("`arg` must be `Arg::Eq`, instead is: {arg:?}"),
        };

        ArgCmp::<Rc<CustomType>>::static_received()
            .new_ref_eq(
                custom_type,
                UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn ref_not_eq_Ok() {
        // Arrange
        ArgCmp::<Rc<CustomType>>::static_setup()
            .new_ref_eq(automock::Arg::Any, automock::Arg::Any)
            .returns(arg_cmp_mock());
        let custom_type = Rc::new(CustomType(5));

        // Act
        let arg = Arg::ref_not_eq(custom_type.clone());

        // Assert
        match arg {
            Arg::NotEq(_, _) => (),
            _ => panic!("`arg` must be `Arg::Eq`, instead is: {arg:?}"),
        };

        ArgCmp::<Rc<CustomType>>::static_received()
            .new_ref_eq(
                custom_type,
                UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn check_Any_ReturnsOk() {
        // Arrange
        let arg = Arg::Any;
        let arg_name = "quo vadis";
        let actual_value = &CustomType(1);
        let actual_value_str = "veridis quo";

        // Act
        let result = arg.check(arg_name, actual_value, actual_value_str.to_owned());

        // Assert
        let ArgCheckResult::Ok(ArgCheckResultOk { arg_info }) = result else {
            panic!("`check` result must be ok.")
        };

        assert_eq!(arg_info.arg_name(), arg_name);
        assert_eq!(arg_info.arg_type_name(), *CUSTOM_TYPE_NAME);
        assert_eq!(arg_info.clone_arg_debug_string(), actual_value_str);
    }

    #[test]
    fn check_Eq_IsEqual_ReturnsOk() {
        // Arrange
        let mut arg_cmp = arg_cmp_mock::<CustomType>();
        arg_cmp
            .setup()
            .is_arg_equal_to(automock::Arg::Any)
            .returns(true);

        let arg_name = "quo vadis";
        let actual_value = &CustomType(1);
        let actual_value_str = "veridis quo";
        let arg = Arg::Eq(arg_cmp, Internal);

        // Act
        let result = arg.check(arg_name, actual_value, actual_value_str.to_owned());

        // Assert
        let ArgCheckResult::Ok(ArgCheckResultOk { arg_info }) = result else {
            panic!("`check` result must be ok.")
        };

        assert_eq!(arg_info.arg_name(), arg_name);
        assert_eq!(arg_info.arg_type_name(), *CUSTOM_TYPE_NAME);
        assert_eq!(arg_info.clone_arg_debug_string(), actual_value_str);

        let Arg::Eq(mut arg_cmp, _) = arg else {
            panic!("Should be Arg::Eq")
        };
        arg_cmp
            .received()
            .is_arg_equal_to(actual_value, automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn check_Eq_IsNotEqual_ReturnsError() {
        // Arrange
        let mut arg_cmp = arg_cmp_mock::<CustomType>();
        let expected_value_str = "whatever";
        let expected_ptr_info_suffix = "expected ptr suffix";
        let actual_ptr_info_suffix = "actual ptr suffix";
        arg_cmp
            .setup()
            .is_arg_equal_to(automock::Arg::Any)
            .returns(false)
            .print_arg()
            .returns(expected_value_str)
            .get_ptrs_info_suffix(automock::Arg::Any)
            .returns(PtrInfo {
                expected_ptr_info_suffix: expected_ptr_info_suffix.to_owned(),
                actual_ptr_info_suffix: actual_ptr_info_suffix.to_owned(),
            });

        let arg_name = "quo vadis";
        let actual_value = &CustomType(1);
        let actual_value_str = "veridis quo";
        let arg = Arg::Eq(arg_cmp, Internal);

        // Act
        let result = arg.check(arg_name, actual_value, actual_value_str.to_owned());

        // Assert
        let ArgCheckResult::Err(ArgCheckResultErr {
            arg_info,
            error_msg,
        }) = result
        else {
            panic!("`check` result must be error.")
        };

        assert_eq!(arg_info.arg_name(), arg_name);
        assert_eq!(arg_info.arg_type_name(), *CUSTOM_TYPE_NAME);
        assert_eq!(arg_info.clone_arg_debug_string(), actual_value_str);

        let expected_error_msg = format!(
            "\t\tExpected{expected_ptr_info_suffix}: {expected_value_str}\n\t\tActual{actual_ptr_info_suffix} {actual_value_str}"
        );
        assert_eq!(error_msg, expected_error_msg);

        let Arg::Eq(mut arg_cmp, _) = arg else {
            panic!("Should be Arg::Eq")
        };
        arg_cmp
            .received()
            .is_arg_equal_to(actual_value, automock::Times::Once)
            .print_arg(automock::Times::Once)
            .get_ptrs_info_suffix(actual_value, automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn check_NotEq_IsEqual_ReturnsError() {
        // Arrange
        let mut arg_cmp = arg_cmp_mock::<CustomType>();
        let not_expected_value_str = "whatever";
        let expected_ptr_info_suffix = "expected ptr suffix";
        arg_cmp
            .setup()
            .is_arg_equal_to(automock::Arg::Any)
            .returns(true)
            .print_arg()
            .returns(not_expected_value_str)
            .get_ptrs_info_suffix(automock::Arg::Any)
            .returns(PtrInfo {
                expected_ptr_info_suffix: expected_ptr_info_suffix.to_owned(),
                actual_ptr_info_suffix: "actual ptr suffix".to_owned(),
            });

        let arg_name = "quo vadis";
        let actual_value = &CustomType(1);
        let actual_value_str = "veridis quo";
        let arg = Arg::NotEq(arg_cmp, Internal);

        // Act
        let result = arg.check(arg_name, actual_value, actual_value_str.to_owned());

        // Assert
        let ArgCheckResult::Err(ArgCheckResultErr {
            arg_info,
            error_msg,
        }) = result
        else {
            panic!("`check` result must be error.")
        };

        assert_eq!(arg_info.arg_name(), arg_name);
        assert_eq!(arg_info.arg_type_name(), *CUSTOM_TYPE_NAME);
        assert_eq!(arg_info.clone_arg_debug_string(), actual_value_str);

        let expected_error_msg =
            format!("\t\tDid not expect to be {expected_ptr_info_suffix}{not_expected_value_str}");
        assert_eq!(error_msg, expected_error_msg);

        let Arg::NotEq(mut arg_cmp, _) = arg else {
            panic!("Should be Arg::NotEq")
        };
        arg_cmp
            .received()
            .is_arg_equal_to(actual_value, automock::Times::Once)
            .print_arg(automock::Times::Once)
            .get_ptrs_info_suffix(actual_value, automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn check_NotEq_IsNotEqual_ReturnsOk() {
        // Arrange
        let mut arg_cmp = arg_cmp_mock::<CustomType>();
        arg_cmp
            .setup()
            .is_arg_equal_to(automock::Arg::Any)
            .returns(false);

        let arg_name = "quo vadis";
        let actual_value = &CustomType(1);
        let actual_value_str = "veridis quo";
        let arg = Arg::NotEq(arg_cmp, Internal);

        // Act
        let result = arg.check(arg_name, actual_value, actual_value_str.to_owned());

        // Assert
        let ArgCheckResult::Ok(ArgCheckResultOk { arg_info }) = result else {
            panic!("`check` result must be ok.")
        };

        assert_eq!(arg_info.arg_name(), arg_name);
        assert_eq!(arg_info.arg_type_name(), *CUSTOM_TYPE_NAME);
        assert_eq!(arg_info.clone_arg_debug_string(), actual_value_str);

        let Arg::NotEq(mut arg_cmp, _) = arg else {
            panic!("Should be Arg::NotEq")
        };
        arg_cmp
            .received()
            .is_arg_equal_to(actual_value, automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn check_Is_IsEqual_ReturnsOk() {
        // Arrange
        ptr_predicate::setup(automock::Arg::Any).returns(true);

        let arg_name = "quo vadis";
        let actual_value = &CustomType(1);
        let actual_value_str = "veridis quo";
        let arg = Arg::Is(Box::new(ptr_predicate), Internal);

        // Act
        let result = arg.check(arg_name, actual_value, actual_value_str.to_owned());

        // Assert
        let ArgCheckResult::Ok(ArgCheckResultOk { arg_info }) = result else {
            panic!("`check` result must be ok.")
        };

        assert_eq!(arg_info.arg_name(), arg_name);
        assert_eq!(arg_info.arg_type_name(), *CUSTOM_TYPE_NAME);
        assert_eq!(arg_info.clone_arg_debug_string(), actual_value_str);

        ptr_predicate::received(actual_value as *const _ as *const (), automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn check_Is_IsNotEqual_ReturnsErr() {
        // Arrange
        ptr_predicate::setup(automock::Arg::Any).returns(false);

        let arg_name = "quo vadis";
        let actual_value = &CustomType(1);
        let actual_value_str = "veridis quo";
        let arg = Arg::Is(Box::new(ptr_predicate), Internal);

        // Act
        let result = arg.check(arg_name, actual_value, actual_value_str.to_owned());

        // Assert
        let ArgCheckResult::Err(ArgCheckResultErr {
            arg_info,
            error_msg,
        }) = result
        else {
            panic!("`check` result must be ok.")
        };

        assert_eq!(arg_info.arg_name(), arg_name);
        assert_eq!(arg_info.arg_type_name(), *CUSTOM_TYPE_NAME);
        assert_eq!(arg_info.clone_arg_debug_string(), actual_value_str);

        let expected_error_msg = format!(
            "\t\tCustom predicate did not match passed value, received: {actual_value_str}"
        );
        assert_eq!(error_msg, expected_error_msg);

        ptr_predicate::received(actual_value as *const _ as *const (), automock::Times::Once)
            .no_other_calls();
    }

    mod utilities {
        use super::*;

        #[derive(Debug, PartialEq, Clone, Default)]
        pub struct CustomType(pub i32);
        pub static CUSTOM_TYPE_NAME: std::sync::LazyLock<&'static str> =
            std::sync::LazyLock::new(std::any::type_name::<CustomType>);

        #[automock::mock]
        pub fn predicate(_: &CustomType) -> bool {
            unreachable!()
        }

        #[automock::mock]
        pub fn ptr_predicate(_: *const ()) -> bool {
            unreachable!()
        }
    }
}
