use crate::args::DerefInfo;

#[repr(C)]
pub(crate) struct ArgCmp<T: ?Sized> {
    pub print_arg: String,
    pub value: Box<T>,
    pub comparator: fn(&T, &T) -> bool,
    pub maybe_deref_info: Option<DerefInfo>,
}

impl<T: ?Sized> ArgCmp<T> {
    // Deliberate temporal coupling. `print_arg` can be calculated only in user code space without
    // the loss of argument values debug string.
    pub fn set_print_arg(&mut self, print_arg: String) {
        self.print_arg = print_arg;
    }

    pub fn is_arg_equal_to(&self, other: &T) -> bool {
        (self.comparator)(&self.value, other)
    }

    pub fn get_ptrs_info_suffix(&self, actual_value: &T) -> PtrInfo {
        self.maybe_deref_info
            .as_ref()
            .map(|deref_info| {
                let expected_ptr = deref_info.expected_value_deref_ptr;
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
