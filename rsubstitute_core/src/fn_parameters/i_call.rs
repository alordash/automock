use crate::args::*;
use crate::fn_parameters::*;

pub trait ICall: IGenericsInfoProvider {
    fn get_arg_infos(&self) -> Vec<ArgInfo> {
        Vec::new()
    }

    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
        Box::leak(Box::new(())) as *mut _
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
    use super::*;
    use rsubstitute::mock;

    #[mock]
    #[derive(Clone)]
    pub struct StubCall;

    #[mock(base)]
    impl StubCall {
        pub fn new() -> Self {
            Self
        }
    }

    #[mock]
    impl IGenericsInfoProvider for StubCall {
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
    impl ICall for StubCall {
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
