use std::ops::Deref;

#[cfg_attr(test, automock::mock)]
#[derive(Clone)]
pub(crate) struct DerefInfo {
    expected_value_deref_ptr: *const (),
    deref_vtable_ptr: *const (),
}

#[cfg_attr(test, automock::mock(base))]
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
        // SAFETY: copy-paste from
        // https://docs.rs/tmp-typst-utils-custom-metadata/latest/tmp_typst_utils_custom_metadata/fat/index.html
        //
        // > This assumes the memory representation of fat pointers.
        // > Although it is not guaranteed by Rust, it’s improbable that it will change.
        // > Still, when the pointer metadata APIs are stable, we should definitely move to them:
        // > https://github.com/rust-lang/rust/issues/81513
        let fat_ptr: FatPointer = unsafe { core::mem::transmute(dyn_ref) };
        let result = Self {
            expected_value_deref_ptr,
            deref_vtable_ptr: fat_ptr.metadata_pointer,
        };
        return result;
    }

    pub fn expected_value_deref_ptr(&self) -> *const () {
        self.expected_value_deref_ptr
    }

    pub fn deref_vtable_ptr(&self) -> *const () {
        self.deref_vtable_ptr
    }

    pub fn get_actual_value_deref_ptr<T: ?Sized>(&self, actual_value: &T) -> *const () {
        let raw_fat_pointer = FatPointer {
            data_pointer: actual_value as *const _ as *const (),
            metadata_pointer: self.deref_vtable_ptr,
        };
        // SAFETY: copy-paste from
        // https://docs.rs/tmp-typst-utils-custom-metadata/latest/tmp_typst_utils_custom_metadata/fat/index.html
        //
        // > This assumes the memory representation of fat pointers.
        // > Although it is not guaranteed by Rust, it’s improbable that it will change.
        // > Still, when the pointer metadata APIs are stable, we should definitely move to them:
        // > https://github.com/rust-lang/rust/issues/81513
        //
        // But also: we do not care about what actual `Deref::Target` is, we just need a pointer
        // to whatever it derefs into. This is safe because `T` can have only single `Deref`
        // implementation, so no matter what `Deref::Target` is, we are able to safely get its
        // pointer.
        let actual_value_as_dyn_ref_ref: &dyn Deref<Target = ()> =
            unsafe { core::mem::transmute(raw_fat_pointer) };
        let result = actual_value_as_dyn_ref_ref.deref() as *const ();
        return result;
    }
}

#[repr(C)]
struct FatPointer {
    data_pointer: *const (),
    metadata_pointer: *const (),
}
