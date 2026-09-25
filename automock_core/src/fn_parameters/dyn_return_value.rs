use crate::fn_parameters::IReturnValue;

pub struct DynReturnValue<'rs> {
    inner: Box<dyn IReturnValue<'rs> + 'rs>,
}

impl<'rs> DynReturnValue<'rs> {
    pub(crate) fn new<T: IReturnValue<'rs> + 'rs>(value: T) -> Self {
        Self {
            inner: Box::new(value),
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
    fn new_Ok() {
        // Arrange
        let return_value = ReturnValue(123);

        // Act
        let dyn_return_value = DynReturnValue::new(return_value.clone());

        // Assert
        let inner_ptr = dyn_return_value.inner.as_ref() as *const _ as *const ReturnValue;
        // SAFETY: DynReturnValue is intended to work with type-erased values
        let inner_ref = unsafe { inner_ptr.as_ref_unchecked() };
        assert_eq!(inner_ref, &return_value);
    }

    #[test]
    fn downcast_into_Ok() {
        // Arrange
        let return_value = ReturnValue(123);
        let dyn_return_value = DynReturnValue::new(return_value.clone());

        // Act
        let result: ReturnValue = dyn_return_value.downcast_into();

        // Assert
        assert_eq!(result, return_value);
    }

    #[derive(Clone, PartialEq, Debug)]
    struct ReturnValue(usize);
}
