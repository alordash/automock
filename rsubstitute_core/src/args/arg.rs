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
    PrivateEq(ArgCmp<T>, Internal),
    #[doc(hidden)]
    PrivateNotEq(ArgCmp<T>, Internal),
    #[doc(hidden)]
    PrivateIs(Box<dyn Fn(*const ()) -> bool>, Internal),
}

const UNINITIALIZED_ARG_PRINT_STRING: &str = "[CRITICAL ERROR]: This string should represent arguments value, but if you see this is it means that `ArgCmp.print_arg` wasn't initialized!";

impl<T: Debug> Debug for Arg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO - extract to const field when std::any::type_name becomes stabilized as const fn_info
        // https://github.com/rust-lang/rust/issues/63084
        let arg_type_name = std::any::type_name::<T>();
        match self {
            Arg::Any => write!(f, "({arg_type_name}): any"),
            Arg::PrivateEq(ArgCmp { value, .. }, _) => {
                write!(f, "({arg_type_name}): equal to {value:?}")
            }
            Arg::PrivateNotEq(ArgCmp { value, .. }, _) => {
                write!(f, "({arg_type_name}): NOT equal to {value:?}")
            }
            Arg::PrivateIs(_, _) => write!(f, "({arg_type_name}): custom predicate"),
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
        return Self::PrivateIs(transmute_lifetime!(boxed_anonymous_predicate), Internal);
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
        return Self::PrivateEq(arg_cmp, Internal);
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
        return Self::PrivateNotEq(arg_cmp, Internal);
    }

    /// Checks that reference of argument value is equal to reference of given value.
    ///
    /// Reference is acquired from [`Deref::deref`].
    pub fn ref_eq<U: ?Sized>(value: T) -> Self
    where
        T: Deref<Target = U>,
    {
        let deref_info = DerefInfo::new(&value);
        let arg_cmp = ArgCmp {
            print_arg: UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
            value: Box::new(value),
            comparator: ptr_cmp,
            maybe_deref_info: Some(deref_info),
        };
        return Self::PrivateEq(arg_cmp, Internal);
    }

    /// Checks that reference of argument value is NOT equal to reference of given value.
    ///
    /// Reference is acquired from [`Deref::deref`].
    pub fn ref_not_eq<U: ?Sized>(value: T) -> Self
    where
        T: Deref<Target = U>,
    {
        let deref_info = DerefInfo::new(&value);
        let arg_cmp = ArgCmp {
            print_arg: UNINITIALIZED_ARG_PRINT_STRING.to_owned(),
            value: Box::new(value),
            comparator: ptr_cmp,
            maybe_deref_info: Some(deref_info),
        };
        return Self::PrivateNotEq(arg_cmp, Internal);
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
            Arg::PrivateEq(arg_cmp, _) => {
                if !arg_cmp.is_arg_equal_to(actual_value) {
                    // let expected_value_str = print_arg(arg_cmp.value.as_ref());
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
            Arg::PrivateNotEq(arg_cmp, _) => {
                if arg_cmp.is_arg_equal_to(actual_value) {
                    // let not_expected_value_str = print_arg(arg_cmp.value.as_ref());
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
            Arg::PrivateIs(predicate, _) => {
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
