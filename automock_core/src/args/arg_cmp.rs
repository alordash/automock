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

#[cfg_attr(test, automock::mock)]
impl<T> ArgCmp<T> {
    pub fn new_eq(value: T, print_arg: String) -> Self
    where
        T: PartialEq,
    {
        Self {
            print_arg,
            value: Box::new(value),
            comparator: <T as PartialEq>::eq,
            maybe_deref_info: None,
        }
    }

    pub fn new_ref_eq<U: ?Sized>(value: T, print_arg: String) -> Self
    where
        T: Deref<Target = U>,
    {
        let deref_info = DerefInfo::from_ref(&value);
        Self {
            print_arg,
            value: Box::new(value),
            comparator: ptr_cmp,
            maybe_deref_info: Some(deref_info),
        }
    }
}

fn ptr_cmp<U: ?Sized, T: Deref<Target = U>>(a: &T, b: &T) -> bool {
    core::ptr::eq(a.deref(), b.deref())
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
            expected_ptr_info_suffix: "".to_string(),
            actual_ptr_info_suffix: ":  ".to_string(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use std::rc::Rc;

    pub fn arg_cmp_mock<T: Default>() -> ArgCmp<T> {
        ArgCmp {
            print_arg: "mock".to_owned(),
            value: Box::new(T::default()),
            comparator: |_, _| false,
            maybe_deref_info: None,
            __rs_data: Default::default(),
        }
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
}
