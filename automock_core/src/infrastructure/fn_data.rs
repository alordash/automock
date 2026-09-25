use crate::args::*;
use crate::fn_parameters::*;
use crate::infrastructure::*;
use crate::times::*;
use crate::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod handle_no_return_value_no_base_calling;
mod handle_no_return_value_with_base_calling;
mod handle_with_return_value_no_base_calling;
mod handle_with_return_value_with_base_calling;

pub struct FnData<
    'rs,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> {
    fn_name: &'static str,
    formatted_fn_name: String,
    pub call_infos: RefCell<HashMap<GenericsHashKey, Vec<CallCheck<'rs>>>>,
    #[allow(clippy::type_complexity)]
    pub configs: RefCell<HashMap<GenericsHashKey, Vec<Rc<RefCell<FnConfig<'rs, TMock>>>>>>,
    force_call_base: bool,
}

impl<
    'rs,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> FnData<'rs, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
{
    pub(crate) fn new(
        maybe_owner_name: Option<&'static str>,
        fn_name: &'static str,
        for_struct: bool,
    ) -> Self {
        let formatted_fn_name = match maybe_owner_name {
            None => fn_name.to_owned(),
            Some(owner_name) => format!("{owner_name}::{fn_name}"),
        };
        Self {
            fn_name,
            formatted_fn_name,
            call_infos: RefCell::new(HashMap::new()),
            configs: RefCell::new(HashMap::new()),
            force_call_base: for_struct,
        }
    }

    #[doc(hidden)]
    pub fn reset(&self) {
        self.call_infos.borrow_mut().clear();
        self.configs.borrow_mut().clear();
    }

    pub fn add_config<
        'a,
        TArgsChecker: IArgsChecker + 'a,
        TOwner,
        TArgRefsTuple,
        TReturnValue,
        TMockArg,
    >(
        &self,
        args_checker: TArgsChecker,
        fn_configurator_owner: &'a TOwner,
    ) -> FnConfigurator<
        'a,
        TMock,
        TOwner,
        TArgRefsTuple,
        TReturnValue,
        TMockArg,
        HAS_RETURN_VALUE,
        SUPPORTS_BASE_CALLING,
        PASSES_MOCK_TO_CALLBACK,
    > {
        let dyn_args_checker: DynArgsChecker<'a> = DynArgsChecker::new(args_checker);
        let generics_hash_key = dyn_args_checker.get_generics_hash_key();
        let config = FnConfig::<'a>::new(dyn_args_checker);
        let arc_config = Rc::new(RefCell::new(config));
        self.configs
            .borrow_mut()
            .entry(generics_hash_key)
            .or_default()
            .push(transmute_lifetime!(arc_config.clone()));
        let fn_configurator = FnConfigurator::new(arc_config, fn_configurator_owner);
        return fn_configurator;
    }

    pub fn verify_received<'a, TArgsChecker: IArgsChecker + 'a>(
        &self,
        args_checker: TArgsChecker,
        times: Times,
    ) {
        let dyn_args_checker = DynArgsChecker::new(args_checker);
        let (matching_calls_check_result, non_matching_calls_check_result) =
            self.get_matching_and_non_matching_calls(&dyn_args_checker);
        let matching_calls_count = matching_calls_check_result.calls_args_check_results.len();
        let valid = times.matches(matching_calls_count);
        if !valid {
            error_printing::panic_received_verification_error(
                self.fn_name,
                &self.formatted_fn_name,
                &dyn_args_checker,
                matching_calls_check_result,
                non_matching_calls_check_result,
                times,
            );
        }
        if call_order_verification::should_perform() {
            for matching_call in matching_calls_check_result.calls_args_check_results {
                let formatted_string = fmt_call(
                    &self.formatted_fn_name,
                    matching_call.args_check_results,
                    GenericParameterInfosFormattingPolicy::Skip,
                );
                call_order_verification::add_call(
                    matching_call.call_order_number,
                    formatted_string,
                );
            }
        }
    }

    pub fn get_unexpected_calls_error_msgs(&self) -> Vec<String> {
        let all_call_infos = self.call_infos.borrow();
        let mut unexpected_call_infos: Vec<_> = all_call_infos
            .values()
            .flatten()
            .filter(|x| x.is_not_verified())
            .collect();
        unexpected_call_infos.sort_by_key(|a| a.number);
        let unexpected_call_arg_infos = unexpected_call_infos
            .into_iter()
            .map(|x| {
                let call = x.get_call();
                error_printing::format_received_unexpected_call_error(
                    &self.formatted_fn_name,
                    call.get_arg_infos(),
                    call.get_generic_parameter_infos(),
                )
            })
            .collect();
        return unexpected_call_arg_infos;
    }
}

// For static fns
impl<
    'rs,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> IMockData
    for FnData<'rs, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
{
    fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>> {
        vec![self.get_unexpected_calls_error_msgs()]
    }
}

mod internal {
    use super::*;

    impl<
        'rs,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    > FnData<'rs, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        pub(crate) fn register_call(&self, call: Rc<DynCall<'rs>>) -> &Self {
            let generics_hash_key = call.get_generics_hash_key();
            self.call_infos
                .borrow_mut()
                .entry(generics_hash_key)
                .or_default()
                .push(CallCheck::new(call));
            self
        }

        pub(crate) fn get_matching_and_non_matching_calls(
            &self,
            dyn_args_checker: &DynArgsChecker,
        ) -> (OrderedCallsCheckResult, OrderedCallsCheckResult) {
            let mut matching_calls_args_check_results = Vec::new();
            let mut non_matching_calls_args_check_results = Vec::new();
            let generics_hash_key = dyn_args_checker.get_generics_hash_key();
            let mut all_call_infos = self.call_infos.borrow_mut();
            let specific_call_infos = all_call_infos.entry(generics_hash_key).or_default();
            for call_info in specific_call_infos.iter_mut() {
                let call_args_check_results = dyn_args_checker.check(call_info.get_call());
                let is_matching = call_args_check_results.iter().all(ArgCheckResult::is_ok);
                let ordered_call_check_result =
                    OrderedCallCheckResult::new(call_info.number, call_args_check_results);
                if is_matching {
                    call_info.mark_as_verified();
                    matching_calls_args_check_results.push(ordered_call_check_result);
                } else {
                    non_matching_calls_args_check_results.push(ordered_call_check_result);
                }
            }
            let matching_calls_check_result =
                OrderedCallsCheckResult::new(matching_calls_args_check_results);
            let non_matching_calls_check_result =
                OrderedCallsCheckResult::new(non_matching_calls_args_check_results);
            return (matching_calls_check_result, non_matching_calls_check_result);
        }

        pub(crate) fn get_optional_matching_config(
            &self,
            dyn_call: &DynCall<'rs>,
        ) -> MatchingConfigSearchResult<'rs, TMock> {
            let with_return_value = false;
            return self.try_get_matching_config(dyn_call, with_return_value);
        }

        pub(crate) fn get_required_matching_config(
            &self,
            dyn_call: &DynCall<'rs>,
        ) -> Rc<RefCell<FnConfig<'rs, TMock>>> {
            let with_return_value = true;
            let fn_config = match self.try_get_matching_config(dyn_call, with_return_value) {
                MatchingConfigSearchResult::Ok(matching_config) => matching_config,
                MatchingConfigSearchResult::Err(matching_config_search_err) => {
                    error_printing::panic_no_suitable_fn_configuration_found(
                        self.fn_name,
                        &self.formatted_fn_name,
                        dyn_call.get_arg_infos(),
                        dyn_call.get_generic_parameter_infos(),
                        matching_config_search_err,
                    )
                }
            };
            return fn_config;
        }

        pub(super) fn try_get_matching_config(
            &self,
            dyn_call: &DynCall<'rs>,
            with_return_value: bool,
        ) -> MatchingConfigSearchResult<'rs, TMock> {
            let generics_hash_key = dyn_call.get_generics_hash_key();
            let all_configs = self.configs.borrow();
            let Some(matching_configs) = all_configs.get(&generics_hash_key) else {
                return MatchingConfigSearchResult::Err(MatchingConfigSearchErr::empty());
            };
            let mut calls_args_check_results = Vec::with_capacity(matching_configs.len());
            for config in matching_configs.iter() {
                let config_ref = config.borrow();
                if with_return_value && !config_ref.has_return_value() {
                    continue;
                }
                let args_check_result = config_ref.check_call(dyn_call);
                drop(config_ref);
                if args_check_result.iter().all(|x| x.is_ok()) {
                    return MatchingConfigSearchResult::Ok(config.clone());
                }
                calls_args_check_results.push(args_check_result);
            }
            calls_args_check_results.sort_by(|a, b| {
                let a_matched_args_count = a.iter().filter(|x| x.is_ok()).count();
                let b_matched_args_count = b.iter().filter(|x| x.is_ok()).count();
                return b_matched_args_count.cmp(&a_matched_args_count);
            });
            let calls_check_result = CallsCheckResult::new(calls_args_check_results);
            return MatchingConfigSearchResult::Err(MatchingConfigSearchErr {
                args_check_results_sorted_by_number_of_correctly_matched_args_descending:
                    calls_check_result,
                needed_return_value: with_return_value,
            });
        }
    }
}
