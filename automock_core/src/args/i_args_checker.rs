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
pub(crate) mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use crate::fn_parameters::i_call::tests::utilities::*;
    use automock::mock;
    use utilities::*;

    #[test]
    fn check_ReturnsEmptyVec() {
        // Arrange
        let dyn_call = DynCall::new(CallMock::new());
        let args_checker = DefaultArgsChecker::new();

        // Act
        let result = args_checker.check(&dyn_call);

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn fmt_args_ReturnsEmptyString() {
        // Arrange
        let args_checker = DefaultArgsChecker::new();

        // Act
        let result = args_checker.fmt_args();

        // Assert
        assert!(result.is_empty());
    }

    pub mod utilities {
        use super::*;
        use std::hash::Hasher;

        #[mock]
        pub struct DefaultArgsChecker;

        impl DefaultArgsChecker {
            pub fn new() -> Self {
                Self {
                    __mock_data: Default::default(),
                }
            }
        }

        impl IGenericsInfoProvider for DefaultArgsChecker {}

        impl IArgsChecker for DefaultArgsChecker {}

        #[mock]
        #[derive(Clone)]
        pub struct ArgsCheckerMock;

        impl ArgsCheckerMock {
            pub fn new() -> Self {
                Self {
                    __mock_data: Default::default(),
                }
            }
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
}
