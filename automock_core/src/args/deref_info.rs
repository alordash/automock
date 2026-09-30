use crate::FatPointer;
use std::ops::Deref;

#[cfg_attr(test, automock::mock)]
#[derive(Clone)]
pub(crate) struct DerefInfo {
    expected_value_deref_ptr: *const (),
    deref_vtable_ptr: *const (),
}

#[cfg_attr(test, automock::mock)]
impl DerefInfo {
    pub fn new(expected_value_deref_ptr: *const (), deref_vtable_ptr: *const ()) -> Self {
        Self {
            expected_value_deref_ptr,
            deref_vtable_ptr,
        }
    }

    pub fn from_ref<T: Deref<Target = U>, U: ?Sized>(expected_value: &T) -> Self {
        let expected_value_deref_ptr = expected_value.deref() as *const _ as *const ();
        let dyn_ref: &dyn Deref<Target = U> = expected_value;
        // SAFETY: refer to `FatPointer` safety comment.
        let fat_ptr: FatPointer = unsafe { core::mem::transmute(dyn_ref) };
        let result = Self::new(expected_value_deref_ptr, fat_ptr.metadata_pointer);
        return result;
    }

    pub fn expected_value_deref_ptr(&self) -> *const () {
        self.expected_value_deref_ptr
    }

    pub fn get_actual_value_deref_ptr<T: ?Sized>(&self, actual_value: &T) -> *const () {
        let raw_fat_pointer = FatPointer {
            data_pointer: actual_value as *const _ as *mut (),
            metadata_pointer: self.deref_vtable_ptr,
        };
        // SAFETY: refer to `FatPointer` safety comment, also: we do not care about what actual
        // `Deref::Target` is, we just need a pointer to whatever it derefs into. This is safe
        // because `T` can have only single `Deref` implementation, so no matter what
        // `Deref::Target` is, we are able to safely get its pointer.
        let actual_value_as_dyn_ref_ref: &dyn Deref<Target = ()> =
            unsafe { core::mem::transmute(raw_fat_pointer) };
        let result = actual_value_as_dyn_ref_ref.deref() as *const ();
        return result;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use automock::Mockable;
    use std::rc::Rc;
    use utilities::*;

    #[test]
    fn new_Ok() {
        // Arrange
        DerefInfo::static_setup()
            .new(automock::Arg::Any, automock::Arg::Any)
            .call_base();
        let expected_value_deref_ptr = 1234 as *const ();
        let deref_vtable_ptr = 5678 as *const ();

        // Act
        let result = DerefInfo::new(expected_value_deref_ptr, deref_vtable_ptr);

        // Assert
        assert_eq!(result.expected_value_deref_ptr, expected_value_deref_ptr);
        assert_eq!(result.deref_vtable_ptr, deref_vtable_ptr);
    }

    #[test]
    fn from_ref_Ok() {
        // Arrange
        type T = Rc<U>;
        type U = i32;
        let deref_info_mock = deref_info_mock();
        let deref_info_mock_id = deref_info_mock.id();
        DerefInfo::static_setup()
            .new(automock::Arg::Any, automock::Arg::Any)
            .returns(deref_info_mock)
            .from_ref::<T, U>(automock::Arg::Any)
            .call_base();

        let value = T::new(5);

        // Act
        let result = DerefInfo::from_ref(&value);

        // Assert
        assert_eq!(result.id(), deref_info_mock_id);
        
        let expected_expected_value_deref_ptr = value.deref() as *const _ as *const ();
        DerefInfo::static_received()
            .new(
                expected_expected_value_deref_ptr,
                automock::Arg::is(|deref_vtable_ptr: &*const ()| {
                    let manually_constructed_fat_ptr: &dyn Deref<Target = U> = {
                        let ptr = &value as *const _ as *mut ();
                        let fat_ptr = FatPointer {
                            data_pointer: ptr,
                            metadata_pointer: *deref_vtable_ptr,
                        };
                        unsafe { core::mem::transmute(fat_ptr) }
                    };
                    let actual_reference =
                        manually_constructed_fat_ptr.deref() as *const _ as *const ();
                    let expected_reference = value.deref() as *const _ as *const ();
                    return actual_reference == expected_reference;
                }),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn expected_value_deref_ptr_Ok() {
        // Arrange
        let mut deref_info = DerefInfo {
            expected_value_deref_ptr: 1234 as *const (),
            deref_vtable_ptr: 5678 as *const (),
            __mock_data: Default::default(),
        };
        deref_info.setup().expected_value_deref_ptr().call_base();

        // Act
        let result = deref_info.expected_value_deref_ptr();

        // Assert
        assert_eq!(result, deref_info.expected_value_deref_ptr);
    }

    #[test]
    fn get_actual_value_deref_ptr_Ok() {
        // Arrange
        type T = Rc<U>;
        type U = i32;
        DerefInfo::static_setup()
            .from_ref::<T, U>(automock::Arg::Any)
            .call_base()
            .new(automock::Arg::Any, automock::Arg::Any)
            .call_base();
        let value = T::new(5);
        let mut deref_info = DerefInfo::from_ref(&value);
        deref_info
            .setup()
            .get_actual_value_deref_ptr::<T>(automock::Arg::Any)
            .call_base();
        let actual_value = Rc::new(10);

        // Act
        let result = deref_info.get_actual_value_deref_ptr(&actual_value);

        // Assert
        let expected_result = actual_value.deref() as *const _ as *const ();
        assert_eq!(result, expected_result);
    }

    pub mod utilities {
        use super::*;
        pub fn deref_info_mock() -> DerefInfo {
            DerefInfo {
                expected_value_deref_ptr: core::ptr::null(),
                deref_vtable_ptr: core::ptr::null(),
                __mock_data: Default::default(),
            }
        }
    }
}
