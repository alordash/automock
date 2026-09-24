use crate::args::*;
use crate::fn_parameters::DynCall;

#[doc(hidden)]
pub trait IArgsChecker: IGenericsInfoProvider {
    fn check(&self, #[allow(unused_variables)] dyn_call: &DynCall) -> Vec<ArgCheckResult> {
        Vec::new()
    }

    fn fmt_args(&self) -> String {
        String::new()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rsubstitute::mock;

    #[mock]
    #[derive(Clone)]
    pub struct ArgsCheckerMock;
    
    #[mock(base)]
    impl ArgsCheckerMock {
        pub fn new() -> Self { Self }
    }

    #[mock]
    impl IGenericsInfoProvider for ArgsCheckerMock {
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
    impl IArgsChecker for ArgsCheckerMock {
        fn check(&self, dyn_call: &DynCall<'_>) -> Vec<ArgCheckResult> {
            unreachable!()
        }
        fn fmt_args(&self) -> String {
            unreachable!()
        }
    }
}
