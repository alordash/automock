use crate::args::*;
use crate::fn_parameters::*;

pub trait ICall: IGenericsInfoProvider {
    fn is_zst(&self) -> bool {
        true
    }

    fn get_arg_infos(&self) -> Vec<ArgInfo> {
        Vec::new()
    }

    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
        core::ptr::null_mut()
    }

    #[doc(hidden)]
    #[allow(private_interfaces)]
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
        // Act
        let result = DefaultCall.is_zst();

        // Assert
        assert!(result);
    }

    #[test]
    fn get_arg_infos_ReturnsEmptyVec() {
        // Act
        let result = DefaultCall.get_arg_infos();

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn get_ptr_to_boxed_tuple_of_refs_ReturnsNullPtr() {
        // Act
        let result = DefaultCall.get_ptr_to_boxed_tuple_of_refs();

        // Assert
        assert!(result.is_null());
    }

    #[test]
    fn get_dyn_tuple_of_refs_IsZst_ReturnsZeroSize() {
        // Arrange
        let mut call = IsZstCall::new();
        call.setup().as_ICall().is_zst().returns(true);
        let dyn_arg_refs_tuple = dyn_arg_refs_tuple_mock();
        DynArgRefsTuple::static_setup()
            .zero_size()
            .returns(dyn_arg_refs_tuple);

        // Act
        _ = call.get_dyn_tuple_of_refs();

        // Assert
        DynArgRefsTuple::static_received()
            .zero_size(automock::Times::Once)
            .no_other_calls();
        call.received()
            .as_ICall()
            .is_zst(automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn get_dyn_tuple_of_refs_IsNotZst_ReturnsFromRawPtr() {
        // Arrange
        let mut call = IsZstCall::new();
        let raw_ptr = 1234 as *mut ();
        call.setup()
            .as_ICall()
            .is_zst()
            .returns(false)
            .get_ptr_to_boxed_tuple_of_refs()
            .returns(raw_ptr);
        let dyn_arg_refs_tuple = dyn_arg_refs_tuple_mock();
        DynArgRefsTuple::static_setup()
            .from_raw(automock::Arg::Any)
            .returns(dyn_arg_refs_tuple);

        // Act
        _ = call.get_dyn_tuple_of_refs();

        // Assert
        DynArgRefsTuple::static_received()
            .from_raw(
                automock::Arg::is(|actual_raw_ptr: &*mut (dyn IArgRefsTuple + '_)| {
                    actual_raw_ptr.addr() == raw_ptr.addr()
                }),
                automock::Times::Once,
            )
            .no_other_calls();
        call.received()
            .as_ICall()
            .is_zst(automock::Times::Once)
            .get_ptr_to_boxed_tuple_of_refs(automock::Times::Once)
            .no_other_calls();
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
        pub struct CallMock {
            pub id: usize,
        }

        impl CallMock {
            pub fn new() -> Self {
                Self {
                    id: 0,
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
