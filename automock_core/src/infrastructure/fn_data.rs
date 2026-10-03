use crate::args::*;
use crate::fn_parameters::*;
use crate::infrastructure::*;
use crate::times::*;
use crate::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod handling;

pub struct FnData<
    'am,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> {
    fn_name: &'static str,
    formatted_fn_name: String,
    pub call_infos: RefCell<HashMap<GenericsHashKey, Vec<CallCheck<'am>>>>,
    #[allow(clippy::type_complexity)]
    pub configs: RefCell<HashMap<GenericsHashKey, Vec<Rc<RefCell<FnConfig<'am, TMock>>>>>>,
}

impl<
    'am,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> FnData<'am, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
{
    pub(crate) fn new(maybe_owner_name: Option<&'static str>, fn_name: &'static str) -> Self {
        let formatted_fn_name = match maybe_owner_name {
            None => fn_name.to_owned(),
            Some(owner_name) => format!("{owner_name}::{fn_name}"),
        };
        Self {
            fn_name,
            formatted_fn_name,
            call_infos: RefCell::new(HashMap::new()),
            configs: RefCell::new(HashMap::new()),
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
        self.handle_call_order_verification(matching_calls_check_result.calls_args_check_results);
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
    'am,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> IMockData
    for FnData<'am, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
{
    fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>> {
        vec![self.get_unexpected_calls_error_msgs()]
    }
}

mod internal {
    use super::*;

    #[cfg_attr(test, automock::mock)]
    pub(super) fn panic_received_verification_error<'am>(
        fn_name: &str,
        formatted_fn_name: &str,
        args_checker: &DynArgsChecker<'am>,
        matching_calls_check_result: OrderedCallsCheckResult,
        non_matching_calls_check_result: OrderedCallsCheckResult,
        _times: Times,
    ) {
        error_printing::panic_received_verification_error(
            fn_name,
            formatted_fn_name,
            args_checker,
            matching_calls_check_result,
            non_matching_calls_check_result,
            _times,
        );
    }

    impl<
        'am,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    > FnData<'am, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        pub(crate) fn register_call(&self, call: Rc<DynCall<'am>>) -> &Self {
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
                let ordered_call_check_result = OrderedCallCheckResult {
                    call_order_number: call_info.number,
                    args_check_results: call_args_check_results,
                };
                if is_matching {
                    call_info.mark_as_verified();
                    matching_calls_args_check_results.push(ordered_call_check_result);
                } else {
                    non_matching_calls_args_check_results.push(ordered_call_check_result);
                }
            }
            let matching_calls_check_result = OrderedCallsCheckResult {
                calls_args_check_results: matching_calls_args_check_results,
            };
            let non_matching_calls_check_result = OrderedCallsCheckResult {
                calls_args_check_results: non_matching_calls_args_check_results,
            };
            return (matching_calls_check_result, non_matching_calls_check_result);
        }

        // todo - remove?
        // pub(crate) fn get_optional_matching_config(
        //     &self,
        //     dyn_call: &DynCall<'am>,
        // ) -> MatchingConfigSearchResult<'am, TMock> {
        //     let with_return_value = false;
        //     return self.try_get_matching_config(dyn_call, with_return_value);
        // }
        //
        // pub(crate) fn get_required_matching_config(
        //     &self,
        //     dyn_call: &DynCall<'am>,
        // ) -> Rc<RefCell<FnConfig<'am, TMock>>> {
        //     let with_return_value = true;
        //     let fn_config = match self.try_get_matching_config(dyn_call, with_return_value) {
        //         MatchingConfigSearchResult::Ok(matching_config) => matching_config,
        //         MatchingConfigSearchResult::Err(matching_config_search_err) => {
        //             error_printing::panic_no_suitable_fn_configuration_found(
        //                 self.fn_name,
        //                 &self.formatted_fn_name,
        //                 dyn_call.get_arg_infos(),
        //                 dyn_call.get_generic_parameter_infos(),
        //                 matching_config_search_err,
        //             )
        //         }
        //     };
        //     return fn_config;
        // }

        pub(super) fn try_get_matching_config<TReturnValue>(
            &self,
            dyn_call: &DynCall<'am>,
            with_return_value: bool,
        ) -> MatchingConfigSearchResult<'am, TMock> {
            let generics_hash_key = dyn_call.get_generics_hash_key();
            let all_configs = self.configs.borrow();
            let Some(matching_configs) = all_configs.get(&generics_hash_key) else {
                return MatchingConfigSearchResult::Err(MatchingConfigSearchErr::empty());
            };
            let mut calls_args_check_results = Vec::with_capacity(matching_configs.len());
            for config in matching_configs.iter() {
                let config_ref = config.borrow();
                if size_of::<TReturnValue>() != 0
                    && with_return_value
                    && !config_ref.has_return_value()
                {
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

        pub(super) fn handle_call_order_verification(
            &self,
            ordered_call_check_results: Vec<OrderedCallCheckResult>,
        ) {
            if call_order_verification::should_perform() {
                for matching_call in ordered_call_check_results {
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
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use args::i_args_checker::tests::utilities::*;
    use automock::Mockable;
    use fn_callback_configurator::tests::utilities::*;
    use fn_config::tests::utilities::*;
    use fn_configurator::tests::utilities::*;
    use utilities::*;

    const IRRELEVANT: bool = false;

    #[test]
    fn new_NoOwnerName_Ok() {
        // Arrange
        let fn_name = "quo vadis";

        // Act
        let result = FnData::<Mock, IRRELEVANT, IRRELEVANT, IRRELEVANT>::new(None, fn_name);

        // Assert
        assert_eq!(result.fn_name, fn_name);
        assert_eq!(result.formatted_fn_name, fn_name);
        assert!(result.call_infos.borrow().is_empty());
        assert!(result.configs.borrow().is_empty());
    }

    #[test]
    fn new_WithOwnerName_Ok() {
        // Arrange
        let fn_name = "quo vadis";
        let owner_name = "veridis quo";

        // Act
        let result =
            FnData::<Mock, IRRELEVANT, IRRELEVANT, IRRELEVANT>::new(Some(owner_name), fn_name);

        // Assert
        assert_eq!(result.fn_name, fn_name);
        let expected_formatted_fn_name = format!("{owner_name}::{fn_name}");
        assert_eq!(result.formatted_fn_name, expected_formatted_fn_name);
        assert!(result.call_infos.borrow().is_empty());
        assert!(result.configs.borrow().is_empty());
    }

    #[test]
    fn reset_Ok() {
        // Arrange
        let fn_data = fn_data::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data
            .call_infos
            .borrow_mut()
            .insert(GenericsHashKey(5), Vec::new());
        fn_data
            .configs
            .borrow_mut()
            .insert(GenericsHashKey(10), Vec::new());

        // Act
        fn_data.reset();

        // Assert
        assert!(fn_data.call_infos.borrow().is_empty());
        assert!(fn_data.configs.borrow().is_empty());
    }

    #[test]
    fn add_config_NoEntryForGenericsHashKey_CreatesNewEntry() {
        // Arrange
        let generics_hash_key = GenericsHashKey(5);
        let mut args_checker_mock = ArgsCheckerMock::new();
        args_checker_mock
            .setup()
            .as_IGenericsInfoProvider()
            .get_generics_hash_key()
            .always_returns(generics_hash_key);

        let fn_configurator_owner = Owner;
        let fn_data = fn_data::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data.configs.borrow_mut().clear();

        let fn_config_mock = fn_config_mock();
        let fn_config_mock_id = fn_config_mock.id();
        FnConfig::static_setup()
            .new(automock::Arg::Any)
            .returns(fn_config_mock);

        // Act
        let result = fn_data.add_config::<_, _, ArgRefsTuple, ReturnValue, MockArg>(
            args_checker_mock.clone(),
            &fn_configurator_owner,
        );

        // Assert
        let configs = fn_data.configs.borrow();
        let inserted_fn_config = &configs
            .get(&generics_hash_key)
            .expect("Config should contain new `FnConfig`s map")[0];
        assert_eq!(inserted_fn_config.borrow().id(), fn_config_mock_id);

        assert_eq!(result.fn_config().borrow().id(), fn_config_mock_id);

        args_checker_mock
            .received()
            .as_IGenericsInfoProvider()
            .get_generics_hash_key(automock::Times::Once)
            .no_other_calls();

        FnConfig::<Mock>::static_received()
            .new(
                automock::Arg::is(|dyn_args_checker: &DynArgsChecker| {
                    dyn_args_checker.get_generics_hash_key() == generics_hash_key
                }),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn add_config_WithEntryForGenericsHashKey_ModifiesExistingEntry() {
        // Arrange
        let generics_hash_key = GenericsHashKey(5);
        let mut args_checker_mock = ArgsCheckerMock::new();
        args_checker_mock
            .setup()
            .as_IGenericsInfoProvider()
            .get_generics_hash_key()
            .always_returns(generics_hash_key);

        let fn_configurator_owner = Owner;
        let fn_data = fn_data::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        let existing_fn_config_index = {
            let mut configs = fn_data.configs.borrow_mut();
            let new_entry = configs
                .entry(generics_hash_key)
                .insert_entry(vec![Rc::new(RefCell::new(fn_config_mock()))]);
            new_entry.get().len()
        };

        let fn_config_mock = fn_config_mock();
        let fn_config_mock_id = fn_config_mock.id();
        FnConfig::static_setup()
            .new(automock::Arg::Any)
            .returns(fn_config_mock);

        // Act
        let result = fn_data.add_config::<_, _, ArgRefsTuple, ReturnValue, MockArg>(
            args_checker_mock.clone(),
            &fn_configurator_owner,
        );

        // Assert
        let configs = fn_data.configs.borrow();
        let inserted_fn_config = &configs
            .get(&generics_hash_key)
            .expect("Config should contain new `FnConfig`s map")[existing_fn_config_index];
        assert_eq!(inserted_fn_config.borrow().id(), fn_config_mock_id);

        assert_eq!(result.fn_config().borrow().id(), fn_config_mock_id);

        args_checker_mock
            .received()
            .as_IGenericsInfoProvider()
            .get_generics_hash_key(automock::Times::Once)
            .no_other_calls();

        FnConfig::<Mock>::static_received()
            .new(
                automock::Arg::is(|dyn_args_checker: &DynArgsChecker| {
                    dyn_args_checker.get_generics_hash_key() == generics_hash_key
                }),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    mod utilities {
        use super::*;

        pub fn fn_data<
            const HAS_RETURN_VALUE: bool,
            const SUPPORTS_BASE_CALLING: bool,
            const PASSES_MOCK_TO_CALLBACK: bool,
        >()
        -> FnData<'static, Mock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
        {
            FnData {
                fn_name: "quo vadis",
                formatted_fn_name: "veridis quo".to_owned(),
                call_infos: Default::default(),
                configs: Default::default(),
            }
        }
    }
}
