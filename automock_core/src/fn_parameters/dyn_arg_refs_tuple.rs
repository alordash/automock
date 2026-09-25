use crate::fn_parameters::*;

#[cfg_attr(test, automock::mock)]
pub struct DynArgRefsTuple<'rs> {
    inner: Option<Box<dyn IArgRefsTuple<'rs> + 'rs>>,
}

#[cfg_attr(test, automock::mock(base))]
impl<'rs> DynArgRefsTuple<'rs> {
    pub(crate) fn zero_size() -> Self {
        Self { inner: None }
    }

    pub(crate) fn from_raw(raw_ptr: *mut (dyn IArgRefsTuple<'rs> + 'rs)) -> Self {
        Self {
            // SAFETY: for justification refer to module level documentation.
            inner: unsafe { Some(Box::from_raw(raw_ptr)) },
        }
    }

    pub fn downcast_into<'a, T: IReturnValue<'a>>(self) -> T {
        if size_of::<T>() == 0 {
            // SAFETY: target type is ZST, it is safe to initialize it using zeroed memory
            return unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
        }
        let raw_ptr = Box::into_raw(
            self.inner
                .unwrap_or_else(|| panic!("[ERROR] Tuple of function arguments is null!")),
        ) as *mut T;
        // SAFETY: for justification refer to module level documentation.
        let boxed = unsafe { Box::from_raw(raw_ptr) };
        let value = *boxed;
        return value;
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use automock::Mockable;

    #[test]
    fn from_raw_Ok() {
        // Arrange
        let arg_refs_tuple = (1, 2, 3);
        let boxed: Box<dyn IArgRefsTuple> = Box::new(arg_refs_tuple);
        let ptr = Box::leak(boxed) as *mut _;

        // Act
        let result = DynArgRefsTuple::from_raw(ptr);

        // Assert
        let actual_ptr = result.inner.expect("from_raw -> must be Some").as_ref() as *const _;
        assert_eq!(actual_ptr, ptr);
    }

    #[test]
    fn downcast_into_Ok() {
        // Arrange
        type ArgRefsTupleType = (i32, i32, i32);
        let arg_refs_tuple: ArgRefsTupleType = (1, 2, 3);
        let boxed: Box<dyn IArgRefsTuple> = Box::new(arg_refs_tuple);
        let ptr = Box::leak(boxed) as *mut _;
        let mut dyn_arg_refs_tuple = DynArgRefsTuple::from_raw(ptr);
        dyn_arg_refs_tuple
            .setup()
            .downcast_into::<ArgRefsTupleType>()
            .call_base();

        // Act
        let result: ArgRefsTupleType = dyn_arg_refs_tuple.downcast_into();

        // Assert
        assert_eq!(result, arg_refs_tuple);
    }
}
