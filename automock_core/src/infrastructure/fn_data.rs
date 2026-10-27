use crate::args::*;
use crate::fn_parameters::*;
use crate::infrastructure::sequence::*;
use crate::infrastructure::error_printing::*;
use crate::infrastructure::*;
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
    pub(crate) fn new(maybe_owner_name: Option<String>, fn_name: &'static str) -> Self {
        let formatted_fn_name = match maybe_owner_name {
            None => fn_name.to_owned(),
            Some(owner_name) => format!("{owner_name}::{fn_name}"),
        };
        Self {
            fn_name,
            formatted_fn_name,
            configs: RefCell::new(HashMap::new()),
        }
    }

    #[doc(hidden)]
    pub fn reset(&self) {
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

    pub fn verify_calls(&self) -> CallsVerificationResult<'am, TMock> {
        let all_configs = self.configs.borrow();
        let unused_configs: Vec<_> = all_configs
            .values()
            .flatten()
            .filter(|x| !x.borrow().return_value_sources.is_empty())
            .cloned()
            .collect();
        return CallsVerificationResult {
            formatted_fn_name: self.formatted_fn_name.clone(),
            unused_configs,
        };
    }

    // For static fns
    pub fn validate_calls(&self) {
        let calls_vertification_result = self.verify_calls();
        if calls_vertification_result.unused_configs.is_empty() {
            return;
        }
        panic_unused_configs_error(vec![calls_vertification_result]);
    }
}

mod internal {
    use super::*;

    #[cfg_attr(test, automock::mock)]
    impl<
        'am,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    > FnData<'am, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        pub(super) fn try_get_matching_config(
            &self,
            dyn_call: &DynCall<'am>,
        ) -> MatchingConfigSearchResult<'am, TMock> {
            let generics_hash_key = dyn_call.get_generics_hash_key();
            let all_configs = self.configs.borrow();
            let Some(matching_configs) = all_configs.get(&generics_hash_key) else {
                return MatchingConfigSearchResult::Err(MatchingConfigSearchErr::empty());
            };
            let mut calls_args_check_results = Vec::with_capacity(matching_configs.len());
            for config in matching_configs.iter() {
                let config_ref = config.borrow();
                if !config_ref.has_return_value() {
                    continue;
                }
                let args_check_result = config_ref.check_call(dyn_call);
                if args_check_result.iter().all(|x| x.is_ok()) {
                    if let Some(sequence) = &config_ref.maybe_sequence {
                        let formatted_string = fmt_call(&self.formatted_fn_name, args_check_result);
                        sequence.add_call_number(config_ref.number, formatted_string);
                    }
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
            });
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use args::i_args_checker::tests::utilities::*;
    use automock::{AsTimes, Mockable};
    use core::assert_matches;
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
        let result = FnData::<Mock, IRRELEVANT, IRRELEVANT, IRRELEVANT>::new(
            Some(owner_name.to_owned()),
            fn_name,
        );

        // Assert
        assert_eq!(result.fn_name, fn_name);
        assert_eq!(result.formatted_fn_name, format!("{owner_name}::{fn_name}"));
        assert!(result.configs.borrow().is_empty());
    }

    #[test]
    fn reset_Ok() {
        // Arrange
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data_mock
            .configs
            .borrow_mut()
            .insert(GenericsHashKey(10), Vec::new());
        fn_data_mock.setup().reset().call_base();

        // Act
        fn_data_mock.reset();

        // Assert
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
            .returns(generics_hash_key, 2.times());
        let owner = Owner;
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data_mock
            .setup()
            .add_config::<ArgsCheckerMock, Owner, ArgRefsTuple, ReturnValue, MockArg>(
                automock::Arg::Any,
                automock::Arg::Any,
            )
            .call_base();
        let configurator = fn_configurator_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>(&owner);
        let configurator_id = configurator.id();
        let config = fn_config_mock();
        let config_id = config.id();
        FnConfig::<Mock>::static_setup()
            .new(automock::Arg::is(|checker: &DynArgsChecker| {
                checker.get_generics_hash_key() == generics_hash_key
            }))
            .returns_once(config);
        FnConfigurator::static_setup()
            .new(
                automock::Arg::is(|config: &Rc<RefCell<FnConfig<Mock>>>| {
                    config.borrow().id() == config_id
                }),
                automock::Arg::ref_eq(&owner),
            )
            .returns_once(configurator);

        // Act
        let result = fn_data_mock.add_config::<_, _, ArgRefsTuple, ReturnValue, MockArg>(
            args_checker_mock.clone(),
            &owner,
        );

        // Assert
        assert_eq!(
            fn_data_mock.configs.borrow()[&generics_hash_key][0]
                .borrow()
                .id(),
            config_id
        );
        assert_eq!(result.id(), configurator_id);
        args_checker_mock.verify_calls();
        FnConfig::<Mock>::verify_static_calls();
        FnConfigurator::<
            Mock,
            Owner,
            ArgRefsTuple,
            ReturnValue,
            MockArg,
            IRRELEVANT,
            IRRELEVANT,
            IRRELEVANT,
        >::verify_static_calls();
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
            .returns(generics_hash_key, 2.times());
        let owner = Owner;
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        let existing_config = Rc::new(RefCell::new(fn_config_mock()));
        fn_data_mock
            .configs
            .borrow_mut()
            .insert(generics_hash_key, vec![existing_config.clone()]);
        fn_data_mock
            .setup()
            .add_config::<ArgsCheckerMock, Owner, ArgRefsTuple, ReturnValue, MockArg>(
                automock::Arg::Any,
                automock::Arg::Any,
            )
            .call_base();
        let configurator = fn_configurator_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>(&owner);
        let configurator_id = configurator.id();
        let config = fn_config_mock();
        let config_id = config.id();
        FnConfig::<Mock>::static_setup()
            .new(automock::Arg::is(|checker: &DynArgsChecker| {
                checker.get_generics_hash_key() == generics_hash_key
            }))
            .returns_once(config);
        FnConfigurator::static_setup()
            .new(
                automock::Arg::is(|config: &Rc<RefCell<FnConfig<Mock>>>| {
                    config.borrow().id() == config_id
                }),
                automock::Arg::ref_eq(&owner),
            )
            .returns_once(configurator);

        // Act
        let result = fn_data_mock.add_config::<_, _, ArgRefsTuple, ReturnValue, MockArg>(
            args_checker_mock.clone(),
            &owner,
        );

        // Assert
        let configs = fn_data_mock.configs.borrow();
        assert_eq!(configs[&generics_hash_key].len(), 2);
        assert!(Rc::ptr_eq(
            &configs[&generics_hash_key][0],
            &existing_config
        ));
        assert_eq!(configs[&generics_hash_key][1].borrow().id(), config_id);
        assert_eq!(result.id(), configurator_id);
        args_checker_mock.verify_calls();
        FnConfig::<Mock>::verify_static_calls();
        FnConfigurator::<
            Mock,
            Owner,
            ArgRefsTuple,
            ReturnValue,
            MockArg,
            IRRELEVANT,
            IRRELEVANT,
            IRRELEVANT,
        >::verify_static_calls();
    }

    #[test]
    fn verify_calls_AllConfigsUsed_ReturnsNoUnusedConfigs() {
        // Arrange
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data_mock.configs.borrow_mut().insert(
            GenericsHashKey(5),
            vec![Rc::new(RefCell::new(fn_config_mock()))],
        );
        fn_data_mock.setup().verify_calls().call_base();

        // Act
        let result = fn_data_mock.verify_calls();

        // Assert
        assert_eq!(result.formatted_fn_name, fn_data_mock.formatted_fn_name);
        assert!(result.unused_configs.is_empty());
    }

    #[test]
    fn verify_calls_UnusedConfig_ReturnsUnusedConfig() {
        // Arrange
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        let used_config = Rc::new(RefCell::new(fn_config_mock()));
        let unused_config = Rc::new(RefCell::new(fn_config_mock()));
        unused_config
            .borrow_mut()
            .return_value_sources
            .push_back(ReturnValueSource::SingleTime(DynReturnValue::new(5)));
        fn_data_mock
            .configs
            .borrow_mut()
            .insert(GenericsHashKey(5), vec![used_config]);
        fn_data_mock
            .configs
            .borrow_mut()
            .insert(GenericsHashKey(10), vec![unused_config.clone()]);
        fn_data_mock.setup().verify_calls().call_base();

        // Act
        let result = fn_data_mock.verify_calls();

        // Assert
        assert_eq!(result.formatted_fn_name, fn_data_mock.formatted_fn_name);
        assert_eq!(result.unused_configs.len(), 1);
        assert!(Rc::ptr_eq(&result.unused_configs[0], &unused_config));
    }

    #[test]
    fn try_get_matching_config_NoConfigs_ReturnsError() {
        // Arrange
        let mut call_mock = CallMock::new();
        call_mock
            .setup()
            .as_IGenericsInfoProvider()
            .get_generics_hash_key()
            .returns_once(GenericsHashKey(5));
        let dyn_call = DynCall::new(call_mock.clone());
        let mut fn_data_mock = fn_data_mock::<IRRELEVANT, IRRELEVANT, IRRELEVANT>();
        fn_data_mock
            .setup()
            .try_get_matching_config(automock::Arg::Any)
            .call_base();

        // Act
        let result = fn_data_mock.try_get_matching_config(&dyn_call);

        // Assert
        assert_matches!(result, MatchingConfigSearchResult::Err(_));
        let MatchingConfigSearchResult::Err(error) = result else {
            unreachable!()
        };
        assert!(
            error
                .args_check_results_sorted_by_number_of_correctly_matched_args_descending
                .calls_args_check_results
                .is_empty()
        );
        call_mock.verify_calls();
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
                configs: Default::default(),
                __mock_data: Default::default(),
            }
        }
    }
}
