use crate::args::DerefInfo;
use std::ops::Deref;

#[cfg_attr(test, automock::mock)]
#[repr(C)]
pub(crate) struct ArgCmp<T: ?Sized> {
    print_arg: String,
    value: Box<T>,
    comparator: fn(&T, &T) -> bool,
    maybe_deref_info: Option<DerefInfo>,
}

fn ptr_cmp<U: ?Sized, T: Deref<Target = U>>(a: &T, b: &T) -> bool {
    core::ptr::eq(a.deref(), b.deref())
}

#[cfg_attr(test, automock::mock)]
impl<T> ArgCmp<T> {
    pub(crate) fn new(
        print_arg: String,
        value: T,
        comparator: fn(&T, &T) -> bool,
        maybe_deref_info: Option<DerefInfo>,
    ) -> Self {
        Self {
            print_arg,
            value: Box::new(value),
            comparator,
            maybe_deref_info,
        }
    }

    pub fn new_eq(print_arg: String, value: T) -> Self
    where
        T: PartialEq,
    {
        Self::new(print_arg, value, <T as PartialEq>::eq, None)
    }

    pub fn new_ref_eq<U: ?Sized>(print_arg: String, value: T) -> Self
    where
        T: Deref<Target = U>,
    {
        let deref_info = DerefInfo::from_ref(&value);
        Self::new(print_arg, value, ptr_cmp, Some(deref_info))
    }
}

#[cfg_attr(test, automock::mock)]
impl<T: ?Sized> ArgCmp<T> {
    pub fn print_arg(&self) -> &str {
        self.print_arg.as_ref()
    }

    // Deliberate temporal coupling. `print_arg` can be calculated only in user code space without
    // the loss of argument value's debug string.
    pub fn set_print_arg(&mut self, print_arg: String) {
        self.print_arg = print_arg;
    }

    pub fn value(&self) -> &T {
        self.value.as_ref()
    }

    pub fn is_arg_equal_to(&self, other: &T) -> bool {
        (self.comparator)(&self.value, other)
    }

    pub fn get_ptrs_info_suffix(&self, actual_value: &T) -> PtrInfo {
        self.maybe_deref_info
            .as_ref()
            .map(|deref_info| {
                let expected_ptr = deref_info.expected_value_deref_ptr();
                let actual_ptr = deref_info.get_actual_value_deref_ptr(actual_value);
                return PtrInfo {
                    expected_ptr_info_suffix: format!(" (ptr: {expected_ptr:?})"),
                    actual_ptr_info_suffix: format!("   (ptr: {actual_ptr:?}):"),
                };
            })
            .unwrap_or_else(PtrInfo::empty)
    }
}

pub(crate) struct PtrInfo {
    pub expected_ptr_info_suffix: String,
    pub actual_ptr_info_suffix: String,
}

impl PtrInfo {
    pub fn empty() -> Self {
        Self {
            expected_ptr_info_suffix: "".to_owned(),
            actual_ptr_info_suffix: ":  ".to_owned(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use crate::args::deref_info::tests::utilities::*;
    use automock::Mockable;
    use std::rc::Rc;
    use utilities::*;

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

    #[test]
    fn new_Ok() {
        // Arrange
        type T = i32;
        let print_arg = "quo vadis".to_owned();
        let value: T = 5;
        fn comparator(_: &T, _: &T) -> bool {
            false
        }
        ArgCmp::<T>::static_setup()
            .new(
                automock::Arg::Any,
                automock::Arg::Any,
                automock::Arg::Any,
                automock::Arg::Any,
            )
            .call_base();

        // Act
        let result = ArgCmp::new(print_arg.clone(), value, comparator, None);

        // Assert
        assert_eq!(result.print_arg, print_arg);
        assert_eq!(*result.value, value);
        assert_eq!(result.comparator as *const (), comparator as *const ());
        assert!(result.maybe_deref_info.is_none());
    }

    #[test]
    fn new_eq_Ok() {
        // Arrange
        type T = i32;
        let print_arg = "quo vadis".to_owned();
        let value: T = 5;
        let arg_cmp_mock = arg_cmp_mock();
        ArgCmp::<T>::static_setup()
            .new(
                automock::Arg::Any,
                automock::Arg::Any,
                automock::Arg::Any,
                automock::Arg::Any,
            )
            .returns(arg_cmp_mock)
            .new_eq(automock::Arg::Any, automock::Arg::Any)
            .call_base();

        // Act
        todo!();
        _ = ArgCmp::new_eq(print_arg.clone(), value);

        // Assert
        ArgCmp::<T>::static_received()
            .new(
                print_arg,
                value,
                automock::Arg::is(|comparator| {
                    core::ptr::eq(*comparator as *const (), <T as PartialEq>::eq as *const ())
                }),
                automock::Arg::is(|maybe_deref_info: &Option<DerefInfo>| {
                    maybe_deref_info.is_none()
                }),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn new_ref_eq_Ok() {
        // Arrange
        type U = i32;
        type T = Rc<U>;
        let print_arg = "quo vadis".to_owned();
        let value: T = Rc::new(5);

        let deref_info_mock = deref_info_mock();
        DerefInfo::static_setup()
            .from_ref(automock::Arg::<&T>::Any)
            .returns(deref_info_mock);

        let arg_cmp_mock = arg_cmp_mock();
        ArgCmp::<T>::static_setup()
            .new(
                automock::Arg::Any,
                automock::Arg::Any,
                automock::Arg::Any,
                automock::Arg::Any,
            )
            .returns(arg_cmp_mock)
            .new_ref_eq(automock::Arg::Any, automock::Arg::Any)
            .call_base();

        // Act
        todo!();
        _ = ArgCmp::new_ref_eq(print_arg.clone(), value.clone());

        // Assert
        ArgCmp::<T>::static_received()
            .new(
                print_arg,
                automock::Arg::ref_eq(value.clone()),
                automock::Arg::is(|comparator| {
                    core::ptr::eq(*comparator as *const (), ptr_cmp::<U, T> as *const ())
                }),
                automock::Arg::is(|maybe_deref_info: &Option<DerefInfo>| {
                    maybe_deref_info.is_some()
                }),
                automock::Times::Once,
            )
            .no_other_calls();

        DerefInfo::static_received()
            .from_ref::<T, U>(automock::Arg::Any, automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn print_arg_Ok() {
        // Arrange
        let print_arg = "quo vadis".to_owned();
        let mut arg_cmp = ArgCmp {
            print_arg: print_arg.clone(),
            value: Box::new(1),
            comparator: |_, _| false,
            maybe_deref_info: None,
            __mock_data: Default::default(),
        };
        arg_cmp.setup().print_arg().call_base();

        // Act
        let result = arg_cmp.print_arg();

        // Assert
        assert_eq!(result, print_arg);
    }

    #[test]
    fn set_print_arg_Ok() {
        // Arrange
        let mut arg_cmp = ArgCmp {
            print_arg: "quo vadis".to_owned(),
            value: Box::new(1),
            comparator: |_, _| false,
            maybe_deref_info: None,
            __mock_data: Default::default(),
        };
        arg_cmp
            .setup()
            .set_print_arg(automock::Arg::Any)
            .call_base();
        let print_arg = "veridis quo".to_owned();

        // Act
        arg_cmp.set_print_arg(print_arg.clone());

        // Assert
        assert_eq!(arg_cmp.print_arg, print_arg);
    }

    #[test]
    fn value_Ok() {
        // Arrange
        let value = 5;
        let mut arg_cmp = ArgCmp {
            print_arg: "quo vadis".to_owned(),
            value: Box::new(value),
            comparator: |_, _| false,
            maybe_deref_info: None,
            __mock_data: Default::default(),
        };
        arg_cmp.setup().value().call_base();

        // Act
        let result = arg_cmp.value();

        // Assert
        assert_eq!(*result, value);
    }

    #[test]
    fn is_arg_equal_to_Ok() {
        // Arrange
        type T = i32;
        let own_value: T = 5;
        let mut arg_cmp = ArgCmp {
            print_arg: "quo vadis".to_owned(),
            value: Box::new(own_value),
            comparator,
            maybe_deref_info: None,
            __mock_data: Default::default(),
        };
        arg_cmp
            .setup()
            .is_arg_equal_to(automock::Arg::Any)
            .call_base();
        let expected_result = false;
        let other = 10;
        comparator::setup::<T>(automock::Arg::Any, automock::Arg::Any).returns(expected_result);

        // Act
        let result = arg_cmp.is_arg_equal_to(&other);

        // Assert
        assert_eq!(result, expected_result);
        comparator::received::<T>(
            automock::Arg::ref_eq(arg_cmp.value.as_ref()),
            automock::Arg::ref_eq(&other),
            automock::Times::Once,
        )
        .no_other_calls();
    }

    #[test]
    fn get_ptrs_info_suffix_EmptyDerefInfo_ReturnsEmptyStrings() {
        // Arrange
        let mut arg_cmp = ArgCmp {
            print_arg: "quo vadis".to_owned(),
            value: Box::new(1),
            comparator,
            maybe_deref_info: None,
            __mock_data: Default::default(),
        };
        arg_cmp
            .setup()
            .get_ptrs_info_suffix(automock::Arg::Any)
            .call_base();

        // Act
        let result = arg_cmp.get_ptrs_info_suffix(&5);

        // Assert
        let expected_expected_ptr_info_suffix = String::new();
        let expected_actual_ptr_info_suffix = ":  ".to_owned();
        assert_eq!(
            result.expected_ptr_info_suffix,
            expected_expected_ptr_info_suffix
        );
        assert_eq!(
            result.actual_ptr_info_suffix,
            expected_actual_ptr_info_suffix
        );
    }

    #[test]
    fn get_ptrs_info_suffix_WithDerefInfo_ReturnsFormattedStrings() {
        // Arrange
        type T = i32;
        let mut deref_info = deref_info_mock();
        let expected_ptr = 1234 as *const ();
        let actual_ptr = 5678 as *const ();
        deref_info
            .setup()
            .expected_value_deref_ptr()
            .returns(expected_ptr)
            .get_actual_value_deref_ptr::<T>(automock::Arg::Any)
            .returns(actual_ptr);
        let mut arg_cmp = ArgCmp {
            print_arg: "quo vadis".to_owned(),
            value: Box::new(1),
            comparator,
            maybe_deref_info: Some(deref_info.clone()),
            __mock_data: Default::default(),
        };
        arg_cmp
            .setup()
            .get_ptrs_info_suffix(automock::Arg::Any)
            .call_base();
        let actual_value: T = 5;

        // Act
        let result = arg_cmp.get_ptrs_info_suffix(&actual_value);

        // Assert
        let expected_expected_ptr_info_suffix = format!(" (ptr: {expected_ptr:?})");
        let expected_actual_ptr_info_suffix = format!("   (ptr: {actual_ptr:?}):");
        assert_eq!(
            result.expected_ptr_info_suffix,
            expected_expected_ptr_info_suffix
        );
        assert_eq!(
            result.actual_ptr_info_suffix,
            expected_actual_ptr_info_suffix
        );

        deref_info
            .received()
            .expected_value_deref_ptr(automock::Times::Once)
            .get_actual_value_deref_ptr(&actual_value, automock::Times::Once)
            .no_other_calls();
    }

    pub mod utilities {
        use super::*;

        #[automock::mock]
        pub fn comparator<T>(_: &T, _: &T) -> bool {
            unreachable!()
        }

        pub fn arg_cmp_mock<T: Default>() -> ArgCmp<T> {
            ArgCmp {
                print_arg: "mock".to_owned(),
                value: Box::new(T::default()),
                comparator: |_, _| false,
                maybe_deref_info: None,
                __mock_data: Default::default(),
            }
        }
    }
}
