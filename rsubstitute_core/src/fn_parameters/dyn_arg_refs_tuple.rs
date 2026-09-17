use crate::fn_parameters::*;

pub struct DynArgRefsTuple<'rs> {
    inner: Box<dyn IArgRefsTuple<'rs> + 'rs>,
}

impl<'rs> DynArgRefsTuple<'rs> {
    pub(crate) fn from_raw(raw_ptr: *mut (dyn IArgRefsTuple<'rs> + 'rs)) -> Self {
        Self {
            // SAFETY: for justification refer to module level documentation.
            inner: unsafe { Box::from_raw(raw_ptr) },
        }
    }

    pub fn downcast_into<'a, T: IReturnValue<'a>>(self) -> T {
        let raw_ptr = Box::into_raw(self.inner) as *mut T;
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

    #[test]
    fn from_raw_Ok() {
        // Arrange
        let arg_refs_tuple = (1, 2, 3);
        let boxed: Box<dyn IArgRefsTuple> = Box::new(arg_refs_tuple);
        let ptr = Box::leak(boxed) as *mut _;

        // Act
        let result = DynArgRefsTuple::from_raw(ptr);

        // Assert
        let actual_ptr = result.inner.as_ref() as *const _;
        assert_eq!(actual_ptr, ptr);
    }

    #[test]
    fn downcast_into_Ok() {
        // Arrange
        let arg_refs_tuple = (1, 2, 3);
        let boxed: Box<dyn IArgRefsTuple> = Box::new(arg_refs_tuple);
        let ptr = Box::leak(boxed) as *mut _;
        let dyn_arg_refs_tuple = DynArgRefsTuple::from_raw(ptr);

        // Act
        let result: (i32, i32, i32) = dyn_arg_refs_tuple.downcast_into();

        // Assert
        assert_eq!(result, arg_refs_tuple);
    }
}
