use crate::args::*;
use crate::fn_parameters::*;

pub trait ICall: IGenericsInfoProvider {
    fn get_arg_infos(&self) -> Vec<ArgInfo> {
        Vec::new()
    }

    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
        core::ptr::null_mut()
    }

    #[doc(hidden)]
    #[allow(private_interfaces)]
    fn get_dyn_tuple_of_refs<'a>(&self) -> DynArgRefsTuple<'a> {
        let raw_ptr = self.get_ptr_to_boxed_tuple_of_refs();
        return DynArgRefsTuple::from_raw(raw_ptr);
    }
}

#[cfg(test)]
pub mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::{AsTimes, Mockable, mock};

    #[test]
    fn get_arg_infos_ReturnsEmptyVec() {
        // Act
        let result = StubCall.get_arg_infos();

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn get_ptr_to_boxed_tuple_of_refs_ReturnsNullPtr() {
        // Act
        let result = StubCall.get_ptr_to_boxed_tuple_of_refs();

        // Assert
        assert_eq!(result, core::ptr::null_mut());
    }

    #[test]
    fn get_dyn_tuple_of_refs_Ok() {
        // Arrange
        type ArgRefsTupleType = (i32, i32, i32);
        let arg_refs_tuple: ArgRefsTupleType = (1, 2, 3);
        let boxed: Box<dyn IArgRefsTuple> = Box::new(arg_refs_tuple);
        let ptr = Box::leak(boxed) as *mut _;
        let mut dyn_arg_refs_tuple_mock = DynArgRefsTuple::from_raw(ptr);
        dyn_arg_refs_tuple_mock
            .setup()
            .downcast_into::<ArgRefsTupleType>()
            .call_base();
        DynArgRefsTuple::static_setup()
            .from_raw(rsubstitute::Arg::Any)
            .returns(dyn_arg_refs_tuple_mock);

        // Act
        let result = StubCall.get_dyn_tuple_of_refs();

        // Assert
        let actual_arg_refs_tuple: ArgRefsTupleType = result.downcast_into();
        assert_eq!(actual_arg_refs_tuple, arg_refs_tuple);

        DynArgRefsTuple::static_received()
            .from_raw(
                rsubstitute::Arg::is(|actual_raw_ptr: &*mut (dyn IArgRefsTuple + '_)| {
                    let ptr = *actual_raw_ptr as *mut ();
                    ptr == core::ptr::null_mut()
                }),
                1.time(),
            )
            .no_other_calls();
    }

    struct StubCall;
    impl IGenericsInfoProvider for StubCall {}
    impl ICall for StubCall {}

    #[mock]
    #[derive(Clone)]
    pub struct CallMock {
        pub id: usize,
    }

    #[mock(base)]
    impl CallMock {
        pub fn new() -> Self {
            Self { id: 0 }
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
