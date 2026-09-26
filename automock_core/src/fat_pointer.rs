// SAFETY: copy-paste from
// https://docs.rs/tmp-typst-utils-custom-metadata/latest/tmp_typst_utils_custom_metadata/fat/index.html
//
// > This assumes the memory representation of fat pointers.
// > Although it is not guaranteed by Rust, it’s improbable that it will change.
// > Still, when the pointer metadata APIs are stable, we should definitely move to them:
// > https://github.com/rust-lang/rust/issues/81513
#[repr(C)]
pub struct FatPointer {
    pub data_pointer: *mut (),
    pub metadata_pointer: *const (),
}
