use crate::args::*;
use crate::fn_parameters::*;
use crate::infrastructure::*;
use crate::times::*;
use crate::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod handling;

#[cfg_attr(test, automock::mock)]
pub struct FnData<
    'am,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> {
    fn_name: &'static str,
    formatted_fn_name: String,
    pub call_checks: RefCell<HashMap<GenericsHashKey, Vec<CallCheck<'am>>>>,
    #[allow(clippy::type_complexity)]
    pub configs: RefCell<HashMap<GenericsHashKey, Vec<Rc<RefCell<FnConfig<'am, TMock>>>>>>,
}

#[cfg_attr(test, automock::mock)]
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
            call_checks: RefCell::new(HashMap::new()),
            configs: RefCell::new(HashMap::new()),
        }
    }

    #[doc(hidden)]
    pub fn reset(&self) {
        self.call_checks.borrow_mut().clear();
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
            internal::panic_received_verification_error(
                self.fn_name,
                &self.formatted_fn_name,
                &dyn_args_checker,
                matching_calls_check_result,
                non_matching_calls_check_result,
                times,
            );
            return;
        }
        self.handle_call_order_verification(matching_calls_check_result.calls_args_check_results);
    }

    pub fn get_unexpected_calls_error_msgs(&self) -> Vec<String> {
        let all_call_infos = self.call_checks.borrow();
        let mut unexpected_call_infos: Vec<_> = all_call_infos
            .values()
            .flatten()
            .filter(|x| x.is_not_verified())
            .collect();
        unexpected_call_infos.sort_by_key(|a| a.number);
        let unexpected_call_arg_infos = unexpected_call_infos
            .into_iter()
            .map(|x| {
                let call = x.get_dyn_call();
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
#[cfg_attr(test, automock::mock)]
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
        times: Times,
    ) {
        error_printing::panic_received_verification_error(
            fn_name,
            formatted_fn_name,
            args_checker,
            matching_calls_check_result,
            non_matching_calls_check_result,
            times,
        );
    }

    #[cfg_attr(test, automock::mock)]
    impl<
        'am,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    > FnData<'am, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        pub(crate) fn register_call(&self, dyn_call: Rc<DynCall<'am>>) {
            let generics_hash_key = dyn_call.get_generics_hash_key();
            self.call_checks
                .borrow_mut()
                .entry(generics_hash_key)
                .or_default()
                .push(CallCheck::new(dyn_call));
        }

        pub(crate) fn get_matching_and_non_matching_calls<'a>(
            &self,
            dyn_args_checker: &DynArgsChecker<'a>,
        ) -> (OrderedCallsCheckResult, OrderedCallsCheckResult) {
            let mut matching_calls_args_check_results = Vec::new();
            let mut non_matching_calls_args_check_results = Vec::new();
            let generics_hash_key = dyn_args_checker.get_generics_hash_key();
            let mut all_call_infos = self.call_checks.borrow_mut();
            let specific_call_infos = all_call_infos.entry(generics_hash_key).or_default();
            for call_info in specific_call_infos.iter_mut() {
                let call_args_check_results = dyn_args_checker.check(call_info.get_dyn_call());
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
            if !call_order_verification::should_perform() {
                return;
            }

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

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use args::i_args_checker::tests::utilities::*;
    use automock::Mockable;
    use fn_callback_configurator::tests::utilities::*;
    use fn_config::tests::utilities::*;
    use fn_configurator::tests::utilities::*;
    use i_call::tests::utilities::*;
    use utilities::*;

    const IRRELEVANT: bool = false;

    #[test]
    fn new_NoOwnerName_Ok() {
        // Arrange
        let fn_name = "quo vadis";
        FnData::<Mock, IRRELEVANT, IRRELEVANT, IRRELEVANT>::static_setup()
            .new(automock::Arg::Any, automock::Arg::Any)
            .call_base();

        // Act
        let result = FnData::<Mock, IRRELEVANT, IRRELEVANT, IRRELEVANT>::new(None, fn_name);

        // Assert
        assert_eq!(result.fn_name, fn_name);
        assert_eq!(result.formatted_fn_name, fn_name);
        assert!(result.call_checks.borrow().is_empty());
        assert!(result.configs.borrow().is_empty());
    }

    #[test]
    fn new_WithOwnerName_Ok() {
        // Arrange
        let fn_name = "quo vadis";
        let owner_name = "veridis quo";
        FnData::<Mock, IRRELEVANT, IRRELEVANT, IRRELEVANT>::static_setup()
            .new(automock::Arg::Any, automock::Arg::Any)
            .call_base();

        // Act
        let result =
            FnData::<Mock, IRRELEVANT, IRRELEVANT, IRRELEVANT>::new(Some(owner_name), fn_name);

        // Assert
        assert_eq!(result.fn_name, fn_name);
        let expected_formatted_fn_name = format!("{owner_name}::{fn_name}");
        assert_eq!(result.formatted_fn_name, expected_formatted_fn_name);
        assert!(result.call_checks.borrow().is_empty());
        assert!(result.configs.borrow().is_empty());
    }

    #[test]
    fn reset_Ok() {
        // Arrange
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data_mock
            .call_checks
            .borrow_mut()
            .insert(GenericsHashKey(5), Vec::new());
        fn_data_mock
            .configs
            .borrow_mut()
            .insert(GenericsHashKey(10), Vec::new());
        fn_data_mock.setup().reset().call_base();

        // Act
        fn_data_mock.reset();

        // Assert
        assert!(fn_data_mock.call_checks.borrow().is_empty());
        assert!(fn_data_mock.configs.borrow().is_empty());
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
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data_mock.configs.borrow_mut().clear();
        fn_data_mock
            .setup()
            .add_config::<ArgsCheckerMock, Owner, ArgRefsTuple, ReturnValue, MockArg>(
                automock::Arg::Any,
                automock::Arg::Any,
            )
            .call_base();

        let fn_configurator_mock =
            fn_configurator_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>(&fn_configurator_owner);
        let fn_configurator_mock_id = fn_configurator_mock.id();
        FnConfigurator::static_setup()
            .new(automock::Arg::Any, automock::Arg::Any)
            .returns(fn_configurator_mock);

        let fn_config_mock = fn_config_mock();
        let fn_config_mock_id = fn_config_mock.id();
        FnConfig::static_setup()
            .new(automock::Arg::Any)
            .returns(fn_config_mock);

        // Act
        let result = fn_data_mock
            .add_config::<ArgsCheckerMock, Owner, ArgRefsTuple, ReturnValue, MockArg>(
                args_checker_mock.clone(),
                &fn_configurator_owner,
            );

        // Assert
        let configs = fn_data_mock.configs.borrow();
        let inserted_fn_config = &configs[&generics_hash_key][0];
        assert_eq!(inserted_fn_config.borrow().id(), fn_config_mock_id);

        assert_eq!(result.id(), fn_configurator_mock_id);

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

        FnConfigurator::<
            Mock,
            Owner,
            ArgRefsTuple,
            ReturnValue,
            MockArg,
            IRRELEVANT,
            IRRELEVANT,
            IRRELEVANT,
        >::static_received()
        .new(
            automock::Arg::is(|arc_config: &Rc<RefCell<FnConfig<Mock>>>| {
                arc_config.borrow().id() == fn_config_mock_id
            }),
            automock::Arg::ref_eq(&fn_configurator_owner),
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
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        let existing_fn_config_index = {
            let mut configs = fn_data_mock.configs.borrow_mut();
            let new_entry = configs
                .entry(generics_hash_key)
                .insert_entry(vec![Rc::new(RefCell::new(fn_config_mock()))]);
            new_entry.get().len()
        };
        fn_data_mock
            .setup()
            .add_config::<ArgsCheckerMock, Owner, ArgRefsTuple, ReturnValue, MockArg>(
                automock::Arg::Any,
                automock::Arg::Any,
            )
            .call_base();

        let fn_configurator_mock =
            fn_configurator_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>(&fn_configurator_owner);
        let fn_configurator_mock_id = fn_configurator_mock.id();
        FnConfigurator::static_setup()
            .new(automock::Arg::Any, automock::Arg::Any)
            .returns(fn_configurator_mock);

        let fn_config_mock = fn_config_mock();
        let fn_config_mock_id = fn_config_mock.id();
        FnConfig::static_setup()
            .new(automock::Arg::Any)
            .returns(fn_config_mock);

        // Act
        let result = fn_data_mock.add_config::<_, _, ArgRefsTuple, ReturnValue, MockArg>(
            args_checker_mock.clone(),
            &fn_configurator_owner,
        );

        // Assert
        let configs = fn_data_mock.configs.borrow();
        let inserted_fn_config = &configs[&generics_hash_key][existing_fn_config_index];
        assert_eq!(inserted_fn_config.borrow().id(), fn_config_mock_id);

        assert_eq!(result.id(), fn_configurator_mock_id);

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

        FnConfigurator::<
            Mock,
            Owner,
            ArgRefsTuple,
            ReturnValue,
            MockArg,
            IRRELEVANT,
            IRRELEVANT,
            IRRELEVANT,
        >::static_received()
        .new(
            automock::Arg::is(|arc_config: &Rc<RefCell<FnConfig<Mock>>>| {
                arc_config.borrow().id() == fn_config_mock_id
            }),
            automock::Arg::ref_eq(&fn_configurator_owner),
            automock::Times::Once,
        )
        .no_other_calls();
    }

    #[test]
    fn verify_received_Valid_DoesNotPanicAndHandlesCallOrderVerification() {
        // Arrange
        let args_checker_mock = ArgsCheckerMock::new();

        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        let matching_calls_args_check_results = vec![OrderedCallCheckResult {
            call_order_number: 5,
            args_check_results: Vec::new(),
        }];
        let matching_calls_check_result = OrderedCallsCheckResult {
            calls_args_check_results: matching_calls_args_check_results.clone(),
        };
        let times = Times::Exactly(matching_calls_check_result.calls_args_check_results.len());
        let non_matching_calls_check_result = OrderedCallsCheckResult {
            calls_args_check_results: Vec::new(),
        };
        fn_data_mock
            .setup()
            .get_matching_and_non_matching_calls(automock::Arg::Any)
            .returns((matching_calls_check_result, non_matching_calls_check_result))
            .verify_received(automock::Arg::<ArgsCheckerMock>::Any, automock::Arg::Any)
            .call_base();

        // Act
        fn_data_mock.verify_received(args_checker_mock, times);

        // Assert
        internal::panic_received_verification_error::received_nothing();

        fn_data_mock
            .received()
            .get_matching_and_non_matching_calls(automock::Arg::Any, automock::Times::Once)
            .handle_call_order_verification(
                matching_calls_args_check_results,
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn verify_received_Invalid_PanicsWithoutHandlingCallOrderVerification() {
        // Arrange
        let args_checker_mock = ArgsCheckerMock::new();

        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        let matching_calls_check_result = OrderedCallsCheckResult {
            calls_args_check_results: vec![OrderedCallCheckResult {
                call_order_number: 5,
                args_check_results: Vec::new(),
            }],
        };
        let times = Times::Exactly(matching_calls_check_result.calls_args_check_results.len() + 10);
        let non_matching_calls_check_result = OrderedCallsCheckResult {
            calls_args_check_results: vec![OrderedCallCheckResult {
                call_order_number: 15,
                args_check_results: Vec::new(),
            }],
        };
        fn_data_mock
            .setup()
            .get_matching_and_non_matching_calls(automock::Arg::Any)
            .returns((
                matching_calls_check_result.clone(),
                non_matching_calls_check_result.clone(),
            ))
            .verify_received(automock::Arg::<ArgsCheckerMock>::Any, automock::Arg::Any)
            .call_base();

        // Act
        fn_data_mock.verify_received(args_checker_mock, times);

        // Assert
        internal::panic_received_verification_error::received(
            fn_data_mock.fn_name,
            automock::Arg::ref_eq(fn_data_mock.formatted_fn_name.as_str()),
            automock::Arg::Any,
            matching_calls_check_result,
            non_matching_calls_check_result,
            times,
            automock::Times::Once,
        )
        .no_other_calls();

        fn_data_mock
            .received()
            .get_matching_and_non_matching_calls(automock::Arg::Any, automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn get_unexpected_calls_error_msgs_Ok() {
        // Arrange
        let call_order_numbers = [2, 4, 3, 1];
        get_next_call_order_number::setup().returns_many(call_order_numbers);

        fn call_check<'am>(
            number: usize,
            verified: bool,
        ) -> (
            CallMock,
            Vec<ArgInfo>,
            Vec<GenericParameterInfo>,
            CallCheck<'am>,
        ) {
            let mut call_mock = CallMock::new();
            let arg_infos = vec![ArgInfo::new(
                format!("arg {number}").leak(),
                format!("arg {number} value").leak(),
                format!("arg {number} debug string"),
            )];
            let generic_parameter_infos = vec![GenericParameterInfo::Type(GenericTypeInfo {
                name: format!("arg {number} generic name").leak(),
                type_name: format!("arg {number} generic type name").leak(),
            })];
            call_mock
                .setup()
                .as_ICall()
                .get_arg_infos()
                .returns(arg_infos.clone());
            call_mock
                .setup()
                .as_IGenericsInfoProvider()
                .get_generic_parameter_infos()
                .returns(generic_parameter_infos.clone());
            let dyn_call = DynCall::new(call_mock.clone());
            let result = CallCheck::new(Rc::new(dyn_call));
            if verified {
                result.mark_as_verified()
            }
            return (call_mock, arg_infos, generic_parameter_infos, result);
        }
        let (mut call_mock_1, arg_infos_1, generic_parameter_infos_1, call_check_1) =
            call_check(1, false);
        let (mut call_mock_2, arg_infos_2, generic_parameter_infos_2, call_check_2) =
            call_check(2, false);
        let (mut call_mock_3, _, _, call_check_3) = call_check(3, true);
        let (mut call_mock_4, _, _, call_check_4) = call_check(4, true);

        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        {
            let mut call_checks_map = fn_data_mock.call_checks.borrow_mut();
            call_checks_map
                .entry(GenericsHashKey(5))
                .insert_entry(vec![call_check_2, call_check_4]);
            call_checks_map
                .entry(GenericsHashKey(15))
                .insert_entry(vec![call_check_3, call_check_1]);
        }
        fn_data_mock
            .setup()
            .get_unexpected_calls_error_msgs()
            .call_base();

        let received_unexpected_call_error_1 = "quo vadis".to_owned();
        let received_unexpected_call_error_2 = "veridis quo".to_owned();
        error_printing::format_received_unexpected_call_error::setup(
            automock::Arg::Any,
            automock::Arg::Any,
            automock::Arg::Any,
        )
        .returns_many([
            received_unexpected_call_error_1.clone(),
            received_unexpected_call_error_2.clone(),
        ]);

        // Act
        let result = fn_data_mock.get_unexpected_calls_error_msgs();

        // Assert
        let expected_result = [
            received_unexpected_call_error_1,
            received_unexpected_call_error_2,
        ];
        assert_eq!(result, expected_result);

        call_mock_1
            .received()
            .as_ICall()
            .get_arg_infos(automock::Times::Once);
        call_mock_1
            .received()
            .as_IGenericsInfoProvider()
            .get_generic_parameter_infos(automock::Times::Once)
            .no_other_calls();
        call_mock_2
            .received()
            .as_ICall()
            .get_arg_infos(automock::Times::Once);
        call_mock_2
            .received()
            .as_IGenericsInfoProvider()
            .get_generic_parameter_infos(automock::Times::Once)
            .no_other_calls();
        call_mock_3.received().no_other_calls();
        call_mock_4.received().no_other_calls();

        error_printing::format_received_unexpected_call_error::received(
            automock::Arg::ref_eq(fn_data_mock.formatted_fn_name.as_str()),
            arg_infos_1,
            generic_parameter_infos_1,
            automock::Times::Once,
        )
        .received(
            automock::Arg::ref_eq(fn_data_mock.formatted_fn_name.as_str()),
            arg_infos_2,
            generic_parameter_infos_2,
            automock::Times::Once,
        )
        .no_other_calls();
    }

    #[test]
    fn get_received_nothing_else_error_msgs_Ok() {
        // Arrange
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        let unexpected_calls_error_msgs = vec!["quo vadis".to_owned(), "veridis quo".to_owned()];
        fn_data_mock
            .setup()
            .get_unexpected_calls_error_msgs()
            .returns(unexpected_calls_error_msgs.clone())
            .as_IMockData()
            .get_received_nothing_else_error_msgs()
            .call_base();

        // Act
        let result = fn_data_mock.get_received_nothing_else_error_msgs();

        // Assert
        assert_eq!(result, vec![unexpected_calls_error_msgs]);
    }

    #[test]
    fn register_call_NoEntry_CreatesNewEntry() {
        // Arrange
        let generics_hash_key = GenericsHashKey(5);
        let mut call_mock = CallMock::new();
        call_mock
            .setup()
            .as_IGenericsInfoProvider()
            .get_generics_hash_key()
            .returns(generics_hash_key);
        let dyn_call = Rc::new(DynCall::new(call_mock));

        let call_check_number = 5;
        get_next_call_order_number::setup().returns(call_check_number);

        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data_mock
            .setup()
            .register_call(automock::Arg::Any)
            .call_base();
        fn_data_mock
            .call_checks
            .borrow_mut()
            .remove(&generics_hash_key);

        // Act
        fn_data_mock.register_call(dyn_call.clone());

        // Assert
        let all_call_checks = fn_data_mock.call_checks.borrow();
        let call_check = &all_call_checks[&generics_hash_key][0];
        let actual_dyn_call = call_check.get_dyn_call();
        assert!(Rc::ptr_eq(actual_dyn_call, &dyn_call));

        assert_eq!(call_check.number, call_check_number);
        get_next_call_order_number::received(automock::Times::Once).no_other_calls();
    }

    #[test]
    fn register_call_WithExistingEntry_CreatesNewEntry() {
        // Arrange
        let generics_hash_key = GenericsHashKey(5);
        let mut call_mock = CallMock::new();
        call_mock
            .setup()
            .as_IGenericsInfoProvider()
            .get_generics_hash_key()
            .returns(generics_hash_key);
        let dyn_call = Rc::new(DynCall::new(call_mock));

        let call_check_number = 5;
        get_next_call_order_number::setup().returns_many([2, call_check_number]);

        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data_mock
            .setup()
            .register_call(automock::Arg::Any)
            .call_base();
        let existing_call_check_index = {
            let mut call_checks = fn_data_mock.call_checks.borrow_mut();
            let new_entry = call_checks
                .entry(generics_hash_key)
                .insert_entry(vec![CallCheck::new(Rc::new(DynCall::new(CallMock::new())))]);
            new_entry.get().len()
        };

        // Act
        fn_data_mock.register_call(dyn_call.clone());

        // Assert
        let all_call_checks = fn_data_mock.call_checks.borrow();
        let call_check = &all_call_checks[&generics_hash_key][existing_call_check_index];
        let actual_dyn_call = call_check.get_dyn_call();
        assert!(Rc::ptr_eq(actual_dyn_call, &dyn_call));

        assert_eq!(call_check.number, call_check_number);
        get_next_call_order_number::received(automock::Times::Exactly(2)).no_other_calls();
    }

    mod utilities {
        use super::*;

        pub fn fn_data_mock<
            const HAS_RETURN_VALUE: bool,
            const SUPPORTS_BASE_CALLING: bool,
            const PASSES_MOCK_TO_CALLBACK: bool,
        >()
        -> FnData<'static, Mock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
        {
            FnData {
                fn_name: "quo vadis",
                formatted_fn_name: "veridis quo".to_owned(),
                call_checks: Default::default(),
                configs: Default::default(),
                __mock_data: Default::default(),
            }
        }
    }
}
