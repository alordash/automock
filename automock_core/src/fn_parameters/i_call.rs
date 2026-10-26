use crate::args::*;
use crate::fn_parameters::*;
use crate::infrastructure::*;

pub trait ICall: IGenericsInfoProvider {
    fn is_zst(&self) -> bool {
        size_of_val(self) == 0
    }

    fn get_arg_infos(&self) -> Vec<ArgInfo> {
        Vec::new()
    }

    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
        core::ptr::null_mut()
    }

    #[doc(hidden)]
    fn get_dyn_tuple_of_refs<'a>(&self) -> DynArgRefsTuple<'a> {
        if self.is_zst() {
            return DynArgRefsTuple::zero_size();
        }
        let raw_ptr = self.get_ptr_to_boxed_tuple_of_refs();
        return DynArgRefsTuple::from_raw(raw_ptr);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use crate::fn_parameters::dyn_arg_refs_tuple::tests::utilities::*;
    use automock::{Mockable, mock};
    use utilities::*;

    #[test]
    fn is_zst_ReturnsTrue() {
        // Arrange
        let call = DefaultCall;

        // Act
        let result = call.is_zst();

        // Assert
        assert!(result);
    }

    #[test]
    fn get_arg_infos_ReturnsEmptyVec() {
        // Arrange
        let call = DefaultCall;

        // Act
        let result = call.get_arg_infos();

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn get_ptr_to_boxed_tuple_of_refs_ReturnsNullPtr() {
        // Arrange
        let call = DefaultCall;

        // Act
        let result = call.get_ptr_to_boxed_tuple_of_refs();

        // Assert
        assert!(result.is_null());
    }

    #[test]
    fn get_dyn_tuple_of_refs_IsZst_ReturnsZeroSize() {
        // Arrange
        let mut call = IsZstCall::new();
        call.setup().as_ICall().is_zst().returns_once(true);
        let dyn_arg_refs_tuple = dyn_arg_refs_tuple_mock();
        let dyn_arg_refs_tuple_id = dyn_arg_refs_tuple.id();
        DynArgRefsTuple::static_setup()
            .zero_size()
            .returns_once(dyn_arg_refs_tuple);

        // Act
        let result = call.get_dyn_tuple_of_refs();

        // Assert
        assert_eq!(result.id(), dyn_arg_refs_tuple_id);

        DynArgRefsTuple::verify_static_calls();
        call.verify_calls();
    }

    #[test]
    fn get_dyn_tuple_of_refs_IsNotZst_ReturnsFromRawPtr() {
        // Arrange
        let mut call = IsZstCall::new();
        let raw_ptr = 1234 as *mut ();
        call.setup()
            .as_ICall()
            .is_zst()
            .returns_once(false)
            .get_ptr_to_boxed_tuple_of_refs()
            .returns_once(raw_ptr);
        let dyn_arg_refs_tuple = dyn_arg_refs_tuple_mock();
        let dyn_arg_refs_tuple_id = dyn_arg_refs_tuple.id();
        DynArgRefsTuple::static_setup()
            .from_raw(automock::Arg::is(
                |actual_raw_ptr: &*mut (dyn IArgRefsTuple + '_)| {
                    actual_raw_ptr.addr() == raw_ptr.addr()
                },
            ))
            .returns_once(dyn_arg_refs_tuple);

        // Act
        let result = call.get_dyn_tuple_of_refs();

        // Assert
        assert_eq!(result.id(), dyn_arg_refs_tuple_id);

        DynArgRefsTuple::verify_static_calls();
        call.verify_calls();
    }

    pub mod utilities {
        use super::*;

        pub struct DefaultCall;
        impl IGenericsInfoProvider for DefaultCall {}
        impl ICall for DefaultCall {}

        #[mock]
        pub struct IsZstCall;
        impl IGenericsInfoProvider for IsZstCall {}
        #[mock]
        impl ICall for IsZstCall {
            fn is_zst(&self) -> bool {
                unreachable!()
            }
            fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
                unreachable!()
            }
        }
        impl IsZstCall {
            pub fn new() -> Self {
                Self {
                    __mock_data: Default::default(),
                }
            }
        }

        #[mock]
        #[derive(Clone)]
        pub struct CallMock;

        impl CallMock {
            pub fn new() -> Self {
                Self {
                    __mock_data: Default::default(),
                }
            }
        }

        #[mock]
        impl IGenericsInfoProvider for CallMock {
            fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
                unreachable!()
            }
            fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
                unreachable!()
            }
            fn hash_const_values(&self, hasher: &mut GenericsHasher) {
                unreachable!()
            }
            fn get_generics_hash_key(&self) -> GenericsHashKey {
                unreachable!()
            }
        }

        #[mock]
        impl ICall for CallMock {
            fn is_zst(&self) -> bool {
                unreachable!()
            }
            fn get_arg_infos(&self) -> Vec<ArgInfo> {
                unreachable!()
            }
            fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
                unreachable!()
            }
            fn get_dyn_tuple_of_refs<'a>(&self) -> DynArgRefsTuple<'a> {
                unreachable!()
            }
        }
    }
}
