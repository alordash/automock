use crate::args::*;
use crate::transmute_lifetime;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub(crate) struct Internal;

/// Argument matcher, checks whether certain argument value matches some expectation.
///
/// `T` - type of argument.
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

const UNINITIALIZED_ARG_PRINT_STRING: &str = "[CRITICAL ERROR]: This string should represent arguments value, but if you see this it means that `ArgCmp.print_arg` was not initialized!";

impl<T: Debug> Debug for Arg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO - extract to const field when std::any::type_name becomes stabilized as const fn_info
        // https://github.com/rust-lang/rust/issues/63084
        let arg_type_name = std::any::type_name::<T>();
        match self {
            Arg::Any => write!(f, "({arg_type_name}): any"),
            Arg::Eq(ArgCmp { value, .. }, _) => {
                write!(f, "({arg_type_name}): equal to {value:?}")
            }
            Arg::NotEq(ArgCmp { value, .. }, _) => {
                write!(f, "({arg_type_name}): NOT equal to {value:?}")
            }
            Arg::Is(_, _) => write!(f, "({arg_type_name}): custom predicate"),
        }
    }
}

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
        let arg_cmp = ArgCmp {
            print_arg: UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
            value: Box::new(value),
            comparator: PartialEq::eq,
            maybe_deref_info: None,
        };
        return Self::Eq(arg_cmp, Internal);
    }

    /// Checks that argument value is NOT equal to given value.
    pub fn not_eq(value: T) -> Self
    where
        T: PartialEq,
    {
        let arg_cmp = ArgCmp {
            print_arg: UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
            value: Box::new(value),
            comparator: PartialEq::eq,
            maybe_deref_info: None,
        };
        return Self::NotEq(arg_cmp, Internal);
    }

    /// Checks that reference of argument value is equal to reference of given value.
    ///
    /// Reference is acquired from [`Deref::deref`].
    pub fn ref_eq<U: ?Sized>(value: T) -> Self
    where
        T: Deref<Target = U>,
    {
        {
            let p = &value as *const _ as *const ();
            dbg!(p);
        }
        let deref_info = DerefInfo::from_ref(&value);
        let arg_cmp = ArgCmp {
            print_arg: UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
            value: Box::new(value),
            comparator: ptr_cmp,
            maybe_deref_info: Some(deref_info),
        };
        return Self::Eq(arg_cmp, Internal);
    }

    /// Checks that reference of argument value is NOT equal to reference of given value.
    ///
    /// Reference is acquired from [`Deref::deref`].
    pub fn ref_not_eq<U: ?Sized>(value: T) -> Self
    where
        T: Deref<Target = U>,
    {
        let deref_info = DerefInfo::from_ref(&value);
        let arg_cmp = ArgCmp {
            print_arg: UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
            value: Box::new(value),
            comparator: ptr_cmp,
            maybe_deref_info: Some(deref_info),
        };
        return Self::NotEq(arg_cmp, Internal);
    }
}

fn ptr_cmp<U: ?Sized, T: Deref<Target = U>>(a: &T, b: &T) -> bool {
    core::ptr::eq(a.deref(), b.deref())
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
            Arg::Eq(arg_cmp, _) => {
                if !arg_cmp.is_arg_equal_to(actual_value) {
                    let expected_value_str = &arg_cmp.print_arg;
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
                    let not_expected_value_str = &arg_cmp.print_arg;
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
                            "\t\tCustom predicate didn't match passed value. Received value: {actual_value_str}",
                        ),
                    });
                }
            }
            Arg::Any => (),
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
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
        let arg = Arg::<CustomType>::Eq(custom_type_stub_arg_cmp(5), Internal);

        // Act
        let result = format!("{arg:?}");

        // Assert
        let expected = format!("({}): equal to {:?}", *CUSTOM_TYPE_NAME, CustomType(5));
        assert_eq!(result, expected);
    }

    #[test]
    fn Debug_fmt_NotEq_Ok() {
        // Arrange
        let arg = Arg::<CustomType>::NotEq(custom_type_stub_arg_cmp(5), Internal);

        // Act
        let result = format!("{arg:?}");

        // Assert
        let expected = format!("({}): NOT equal to {:?}", *CUSTOM_TYPE_NAME, CustomType(5));
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
        let custom_type = CustomType(5);

        // Act
        let arg = Arg::eq(custom_type.clone());

        // Assert
        let arg_cmp = match arg {
            Arg::Eq(ac, _) => ac,
            _ => panic!("`arg` must be `Arg::Eq`, instead is: {arg:?}"),
        };

        assert_eq!(arg_cmp.print_arg, UNINITIALIZED_ARG_PRINT_STRING);
        assert_eq!(*arg_cmp.value, custom_type);

        let actual_comparator_ptr = arg_cmp.comparator as *const ();
        let expected_comparator_ptr = <CustomType as PartialEq>::eq as *const ();
        assert!(!actual_comparator_ptr.is_null());
        assert_eq!(actual_comparator_ptr, expected_comparator_ptr);

        assert!(arg_cmp.maybe_deref_info.is_none());
    }

    #[test]
    fn not_eq_Ok() {
        // Arrange
        let custom_type = CustomType(5);

        // Act
        let arg = Arg::not_eq(custom_type.clone());

        // Assert
        let arg_cmp = match arg {
            Arg::NotEq(ac, _) => ac,
            _ => panic!("`arg` must be `Arg::NotEq`, instead is: {arg:?}"),
        };

        assert_eq!(arg_cmp.print_arg, UNINITIALIZED_ARG_PRINT_STRING);
        assert_eq!(*arg_cmp.value, custom_type);

        let actual_comparator_ptr = arg_cmp.comparator as *const ();
        let expected_comparator_ptr = <CustomType as PartialEq>::eq as *const ();
        assert!(!actual_comparator_ptr.is_null());
        assert_eq!(actual_comparator_ptr, expected_comparator_ptr);

        assert!(arg_cmp.maybe_deref_info.is_none());
    }

    #[test]
    fn ref_eq_Ok() {
        // Arrange
        let custom_type = Rc::new(CustomType(5));
        let mut deref_info_mock = DerefInfo::new(1234usize as *const (), 5678usize as *const ());
        deref_info_mock
            .setup()
            .expected_value_deref_ptr()
            .call_base()
            .deref_vtable_ptr()
            .call_base();
        DerefInfo::static_setup()
            .from_ref(automock::Arg::<&Rc<CustomType>>::Any)
            .returns(deref_info_mock.clone());

        // Act
        let arg = Arg::ref_eq(custom_type.clone());

        // Assert
        let arg_cmp = match arg {
            Arg::Eq(ac, _) => ac,
            _ => panic!("`arg` must be `Arg::Eq`, instead is: {arg:?}"),
        };

        assert_eq!(arg_cmp.print_arg, UNINITIALIZED_ARG_PRINT_STRING);
        assert!(Rc::ptr_eq(arg_cmp.value.as_ref(), &custom_type));

        let actual_comparator_ptr = arg_cmp.comparator as *const ();
        let expected_comparator_ptr = ptr_cmp::<CustomType, Rc<CustomType>> as *const ();
        assert!(!actual_comparator_ptr.is_null());
        assert_eq!(actual_comparator_ptr, expected_comparator_ptr);

        let deref_info = arg_cmp
            .maybe_deref_info
            .expect("DerefInfo must be set for `Arg::ref_eq`.");
        assert_eq!(
            deref_info.expected_value_deref_ptr(),
            1234usize as *const ()
        );
        assert_eq!(deref_info.deref_vtable_ptr(), 5678usize as *const ());
    }

    #[test]
    fn ref_not_eq_Ok() {
        // Arrange
        let custom_type = Rc::new(CustomType(5));
        let mut deref_info_mock = DerefInfo::new(1234usize as *const (), 5678usize as *const ());
        deref_info_mock
            .setup()
            .expected_value_deref_ptr()
            .call_base()
            .deref_vtable_ptr()
            .call_base();
        DerefInfo::static_setup()
            .from_ref(automock::Arg::<&Rc<CustomType>>::Any)
            .returns(deref_info_mock.clone());

        // Act
        let arg = Arg::ref_not_eq(custom_type.clone());

        // Assert
        let arg_cmp = match arg {
            Arg::NotEq(ac, _) => ac,
            _ => panic!("`arg` must be `Arg::Eq`, instead is: {arg:?}"),
        };

        assert_eq!(arg_cmp.print_arg, UNINITIALIZED_ARG_PRINT_STRING);
        assert!(Rc::ptr_eq(arg_cmp.value.as_ref(), &custom_type));

        let actual_comparator_ptr = arg_cmp.comparator as *const ();
        let expected_comparator_ptr = ptr_cmp::<CustomType, Rc<CustomType>> as *const ();
        assert!(!actual_comparator_ptr.is_null());
        assert_eq!(actual_comparator_ptr, expected_comparator_ptr);

        let deref_info = arg_cmp
            .maybe_deref_info
            .expect("DerefInfo must be set for `Arg::ref_eq`.");
        assert_eq!(
            deref_info.expected_value_deref_ptr(),
            1234usize as *const ()
        );
        assert_eq!(deref_info.deref_vtable_ptr(), 5678usize as *const ());
    }

    #[test]
    fn ptr_cmp_DerefsToSame_ReturnsTrue() {
        // Arrange
        let a = Rc::new(1);
        let b = a.clone();

        // Act
        let result = ptr_cmp(&a, &b);

        // Assert
        assert!(result);
    }

    #[test]
    fn ptr_cmp_DerefsToDifferent_ReturnsFalse() {
        // Arrange
        let a = Rc::new(1);
        let b = Rc::new(1);

        // Act
        let result = ptr_cmp(&a, &b);

        // Assert
        assert!(!result);
    }

    mod utilities {
        use super::*;

        #[derive(Debug, PartialEq, Clone)]
        pub struct CustomType(pub i32);
        pub static CUSTOM_TYPE_NAME: std::sync::LazyLock<&'static str> =
            std::sync::LazyLock::new(std::any::type_name::<CustomType>);

        pub fn custom_type_stub_arg_cmp(v: i32) -> ArgCmp<CustomType> {
            ArgCmp {
                print_arg: String::new(),
                value: Box::new(CustomType(v)),
                comparator: |_, _| false,
                maybe_deref_info: None,
            }
        }

        #[automock::mock]
        pub fn predicate(_: &CustomType) -> bool {
            unreachable!()
        }
    }
}
