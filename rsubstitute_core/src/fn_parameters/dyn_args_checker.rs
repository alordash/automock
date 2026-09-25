use crate::args::*;
use crate::fn_parameters::DynCall;

pub struct DynArgsChecker<'rs> {
    inner: Box<dyn IArgsChecker + 'rs>,
}

impl<'rs> IGenericsInfoProvider for DynArgsChecker<'rs> {
    fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
        self.inner.get_generic_parameter_infos()
    }

    fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_generics_type_ids(hasher)
    }

    fn hash_const_values(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_const_values(hasher)
    }
}

impl<'rs> IArgsChecker for DynArgsChecker<'rs> {
    fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
        self.inner.check(dyn_call)
    }

    fn fmt_args(&self) -> String {
        self.inner.fmt_args()
    }
}

impl<'rs> DynArgsChecker<'rs> {
    pub(crate) fn new<T: IArgsChecker + 'rs>(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #![allow(non_snake_case)]
// 
//     use super::*;
//     use crate::args::tests::*;
//     use crate::fn_parameters::tests::*;
//     use rsubstitute::{AsTimes, Mockable};
// 
//     #[test]
//     fn IGenericsInfoProvider_get_generic_parameter_infos_ForwardsToInner() {
//         // Arrange
//         let generic_parameter_infos = vec![GenericParameterInfo::Type(GenericTypeInfo {
//             name: "quo",
//             type_name: "vadis",
//         })];
//         let mut args_checker_mock = ArgsCheckerMock::new();
//         args_checker_mock
//             .setup()
//             .as_IGenericsInfoProvider()
//             .get_generic_parameter_infos()
//             .returns(generic_parameter_infos.clone());
//         let dyn_args_checker = DynArgsChecker::new(args_checker_mock.clone());
// 
//         // Act
//         let result = dyn_args_checker.get_generic_parameter_infos();
// 
//         // Assert
//         assert_eq!(result, generic_parameter_infos);
// 
//         args_checker_mock
//             .received()
//             .as_IGenericsInfoProvider()
//             .get_generic_parameter_infos(1.time())
//             .no_other_calls();
//     }
// 
//     #[test]
//     fn IGenericsInfoProvider_hash_generics_type_ids_ForwardsToInner() {
//         // Arrange
//         let mut generics_hasher = GenericsHasher::new();
//         let mut args_checker_mock = ArgsCheckerMock::new();
//         let dyn_args_checker = DynArgsChecker::new(args_checker_mock.clone());
// 
//         // Act
//         dyn_args_checker.hash_generics_type_ids(&mut generics_hasher);
// 
//         // Assert
//         args_checker_mock
//             .received()
//             .as_IGenericsInfoProvider()
//             .hash_generics_type_ids(rsubstitute::Arg::ref_eq(&mut generics_hasher), 1.time())
//             .no_other_calls();
//     }
// 
//     #[test]
//     fn IGenericsInfoProvider_hash_const_values_ids_ForwardsToInner() {
//         // Arrange
//         let mut generics_hasher = GenericsHasher::new();
//         let mut args_checker_mock = ArgsCheckerMock::new();
//         let dyn_args_checker = DynArgsChecker::new(args_checker_mock.clone());
// 
//         // Act
//         dyn_args_checker.hash_const_values(&mut generics_hasher);
// 
//         // Assert
//         args_checker_mock
//             .received()
//             .as_IGenericsInfoProvider()
//             .hash_const_values(rsubstitute::Arg::ref_eq(&mut generics_hasher), 1.time())
//             .no_other_calls();
//     }
// 
//     #[test]
//     fn IArgsChecker_check_ForwardsToInner() {
//         // Arrange
//         let mut args_checker_mock = ArgsCheckerMock::new();
//         let arg_check_results = vec![
//             ArgCheckResult::Ok(ArgCheckResultOk {
//                 arg_info: ArgInfo::new("quo", "vadis", "veridis".to_owned()),
//             }),
//             ArgCheckResult::Err(ArgCheckResultErr {
//                 arg_info: ArgInfo::new("err quo", "err vadis", "err veridis".to_owned()),
//                 error_msg: "error msg".to_owned(),
//             }),
//         ];
//         args_checker_mock
//             .setup()
//             .as_IArgsChecker()
//             .check(rsubstitute::Arg::Any)
//             .returns(arg_check_results.clone());
// 
//         let dyn_args_checker = DynArgsChecker::new(args_checker_mock.clone());
//         let dyn_call = DynCall::new(CallMock::new());
// 
//         // Act
//         let result = dyn_args_checker.check(&dyn_call);
// 
//         // Assert
//         assert_eq!(result, arg_check_results);
//         args_checker_mock
//             .received()
//             .as_IArgsChecker()
//             .check(rsubstitute::Arg::ref_eq(&dyn_call), 1.time())
//             .no_other_calls();
//     }
// 
//     #[test]
//     fn IArgsChecker_fmt_args_ForwardsToInner() {
//         // Arrange
//         let mut args_checker_mock = ArgsCheckerMock::new();
//         let fmt_args_string = "quo vadis".to_owned();
//         args_checker_mock
//             .setup()
//             .as_IArgsChecker()
//             .fmt_args()
//             .returns(fmt_args_string.clone());
// 
//         let dyn_args_checker = DynArgsChecker::new(args_checker_mock.clone());
// 
//         // Act
//         let result = dyn_args_checker.fmt_args();
// 
//         // Assert
//         assert_eq!(result, fmt_args_string);
//         args_checker_mock
//             .received()
//             .as_IArgsChecker()
//             .fmt_args(1.time())
//             .no_other_calls();
//     }
// }
