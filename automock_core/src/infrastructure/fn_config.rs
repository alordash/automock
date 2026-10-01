use crate::args::*;
use crate::fn_parameters::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;

#[cfg_attr(test, automock::mock)]
pub struct FnConfig<'am, TMock> {
    _phantom_mock: PhantomData<TMock>,
    pub args_checker: DynArgsChecker<'am>,
    pub return_value_sources: VecDeque<ReturnValueSource<'am>>,
    pub calls: Vec<Rc<DynCall<'am>>>,
    #[allow(clippy::type_complexity)]
    pub maybe_callback: Option<Rc<RefCell<dyn FnMut(*const (), &DynCall<'am>) + 'am>>>,
    pub call_base: bool,
}

#[cfg_attr(test, automock::mock)]
impl<'am, TMock> FnConfig<'am, TMock> {
    pub(crate) fn new(args_checker: DynArgsChecker<'am>) -> Self {
        FnConfig {
            _phantom_mock: PhantomData,
            args_checker,
            return_value_sources: VecDeque::new(),
            calls: Vec::new(),
            maybe_callback: None,
            call_base: false,
        }
    }

    pub(crate) fn add_return_value_source(&mut self, return_value: ReturnValueSource<'am>) {
        self.return_value_sources.push_back(return_value);
    }

    pub(crate) fn add_return_value_sources<T: IntoIterator<Item = ReturnValueSource<'am>>>(
        &mut self,
        return_values: T,
    ) {
        self.return_value_sources.extend(return_values);
    }

    pub(crate) fn set_callback<TArgRefsTuple: 'am, TMockArg: 'am>(
        &mut self,
        mut callback: impl FnMut(&TMockArg, TArgRefsTuple) + 'am,
    ) {
        let dyn_callback = move |raw_mock_ptr: *const (), dyn_call: &DynCall<'am>| {
            let arg_refs_tuple = if size_of::<TArgRefsTuple>() == 0 {
                // SAFETY: target type is ZST, it is safe to initialize it using zeroed memory
                unsafe { core::mem::zeroed() }
            } else {
                let raw_arg_refs_tuple_ptr = dyn_call.get_ptr_to_boxed_tuple_of_refs();
                let arg_refs_tuple_ptr = raw_arg_refs_tuple_ptr as *mut TArgRefsTuple;

                // SAFETY: both `get_ptr_to_boxed_tuple_of_refs` implementation and `TArgRefsTuple` type
                // are controlled by procedure macro. This guarantees that downcasting from `Box` is safe
                // and won't lead to transmutation between different types.
                let boxed_arg_refs_tuple = unsafe { Box::from_raw(arg_refs_tuple_ptr) };
                *boxed_arg_refs_tuple
            };
            // SAFETY: using pointer instead of reference to untie `TMock` lifetime from `callback`
            // in `FnConfig`. Pointer is passed from `FnData` which casts valid reference to pointer.
            let mock_ref = unsafe {
                let mock_ptr = raw_mock_ptr as *const TMockArg;
                mock_ptr
                    .as_ref()
                    .expect("Pointer to mock in user callback must be not null.")
            };
            callback(mock_ref, arg_refs_tuple)
        };
        self.maybe_callback = Some(Rc::new(RefCell::new(dyn_callback)));
    }

    pub(crate) fn register_call(&mut self, call: Rc<DynCall<'am>>) {
        self.calls.push(call);
    }

    pub(crate) fn check_call(&self, call: &DynCall<'am>) -> Vec<ArgCheckResult> {
        self.args_checker.check(call)
    }

    pub(crate) fn has_return_value(&self) -> bool {
        self.call_base || self.return_value_sources.front().is_some()
    }

    pub(crate) fn select_next_return_value(
        &mut self,
        call: &DynCall<'am>,
    ) -> Option<DynReturnValue<'am>> {
        let return_value_source = self.return_value_sources.front()?;

        return match return_value_source {
            ReturnValueSource::SingleTime(_) => {
                let Some(ReturnValueSource::SingleTime(return_value)) =
                    self.return_value_sources.pop_front()
                else {
                    panic!(
                        "Front return value source must be not empty and single time because it was just checked."
                    )
                };
                Some(return_value)
            }
            ReturnValueSource::Perpetual(perpetual_factory) => {
                let return_value = perpetual_factory();
                Some(return_value)
            }
            ReturnValueSource::Factory(factory) => {
                let dyn_arg_refs_tuple = call.get_dyn_tuple_of_refs();
                let return_value = factory(dyn_arg_refs_tuple);
                return Some(return_value);
            }
        };
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn get_callback(
        &self,
    ) -> Option<Rc<RefCell<dyn FnMut(*const (), &DynCall<'am>) + 'am>>> {
        self.maybe_callback.clone()
    }

    pub(crate) fn set_call_base(&mut self) {
        self.call_base = true;
    }

    pub(crate) fn should_call_base(&self) -> bool {
        self.call_base
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use crate::args::i_args_checker::tests::utilities::*;
    use crate::fn_parameters::dyn_arg_refs_tuple::tests::utilities::*;
    use crate::fn_parameters::i_call::tests::utilities::*;
    use automock::Mockable;
    use utilities::*;

    struct Mock;
    type MockArg = i32;
    struct ZeroSizeArgRefsTuple;
    #[derive(Clone, PartialEq)]
    struct ArgRefsTuple(i32);

    #[test]
    fn new_Ok() {
        // Arrange
        FnConfig::<Mock>::static_setup()
            .new(automock::Arg::Any)
            .call_base();
        let dyn_args_checker = DynArgsChecker::new(ArgsCheckerMock::new());

        // Act
        let result = FnConfig::<Mock>::new(dyn_args_checker);

        // Assert
        assert!(result.return_value_sources.is_empty());
        assert!(result.calls.is_empty());
        assert!(result.maybe_callback.is_none());
        assert!(!result.call_base);
    }

    #[test]
    fn add_return_value_source_Ok() {
        // Arrange
        let mut fn_config = fn_config_mock();
        fn_config
            .setup()
            .add_return_value_source(automock::Arg::Any)
            .call_base();
        let initial_return_value_source_count = fn_config.return_value_sources.len();
        let return_value_source = ReturnValueSource::SingleTime(DynReturnValue::new(1));

        // Act
        fn_config.add_return_value_source(return_value_source);

        // Assert
        let expected_return_value_source_count = initial_return_value_source_count + 1;
        assert_eq!(
            fn_config.return_value_sources.len(),
            expected_return_value_source_count
        );
    }

    #[test]
    fn add_return_value_sources_Ok() {
        // Arrange
        let mut fn_config = fn_config_mock();
        fn_config
            .setup()
            .add_return_value_sources::<[ReturnValueSource; 2]>(automock::Arg::Any)
            .call_base();
        let initial_return_value_source_count = fn_config.return_value_sources.len();
        let return_value_sources = [
            ReturnValueSource::SingleTime(DynReturnValue::new(1)),
            ReturnValueSource::SingleTime(DynReturnValue::new(2)),
        ];
        let return_value_sources_len = return_value_sources.len();

        // Act
        fn_config.add_return_value_sources(return_value_sources);

        // Assert
        let expected_return_value_source_count =
            initial_return_value_source_count + return_value_sources_len;
        assert_eq!(
            fn_config.return_value_sources.len(),
            expected_return_value_source_count
        );
    }

    #[test]
    fn set_callback_ZeroSizeArgRefsTuple_Ok() {
        // Arrange
        let mut fn_config = fn_config_mock();
        fn_config
            .setup()
            .set_callback::<ZeroSizeArgRefsTuple, MockArg>(automock::Arg::Any)
            .call_base();
        let mock_arg: MockArg = 5;
        let raw_mock_arg_ptr = &mock_arg as *const _ as *const ();
        let dyn_call = DynCall::new(CallMock::new());

        // Act
        fn_config.set_callback(callback::<ZeroSizeArgRefsTuple>);

        // Assert
        let actual_callback = fn_config.maybe_callback.expect("Callback must be set.");

        actual_callback.borrow_mut()(raw_mock_arg_ptr, &dyn_call);
        callback::received::<ZeroSizeArgRefsTuple>(
            automock::Arg::ref_eq(&mock_arg),
            automock::Arg::Any,
            automock::Times::Once,
        )
        .no_other_calls();
    }

    #[test]
    fn set_callback_NotZeroSizeArgRefsTuple_Ok() {
        // Arrange
        let mut fn_config = fn_config_mock();
        fn_config
            .setup()
            .set_callback::<ArgRefsTuple, MockArg>(automock::Arg::Any)
            .call_base();
        let mock_arg: MockArg = 5;
        let raw_mock_arg_ptr = &mock_arg as *const _ as *const ();
        let mut call_mock = CallMock::new();
        let arg_refs_tuple = ArgRefsTuple(10);
        let raw_arg_refs_tuple_ptr =
            Box::leak(Box::new(arg_refs_tuple.clone())) as *mut _ as *mut ();
        call_mock
            .setup()
            .as_ICall()
            .get_ptr_to_boxed_tuple_of_refs()
            .returns(raw_arg_refs_tuple_ptr);
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        fn_config.set_callback(callback::<ArgRefsTuple>);

        // Assert
        let actual_callback = fn_config.maybe_callback.expect("Callback must be set.");

        actual_callback.borrow_mut()(raw_mock_arg_ptr, &dyn_call);
        call_mock
            .received()
            .as_ICall()
            .get_ptr_to_boxed_tuple_of_refs(automock::Times::Once);
        callback::received::<ArgRefsTuple>(
            automock::Arg::ref_eq(&mock_arg),
            arg_refs_tuple,
            automock::Times::Once,
        )
        .no_other_calls();
    }

    #[test]
    fn register_call_Ok() {
        // Arrange
        let mut fn_config = fn_config_mock();
        fn_config
            .setup()
            .register_call(automock::Arg::Any)
            .call_base();
        let dyn_call = Rc::new(DynCall::new(CallMock::new()));
        let initial_calls_count = fn_config.calls.len();

        // Act
        fn_config.register_call(dyn_call);

        // Assert
        let expected_calls_count = initial_calls_count + 1;
        assert_eq!(fn_config.calls.len(), expected_calls_count);
    }

    #[test]
    fn check_call_Ok() {
        // Arrange
        let mut args_checker_mock = ArgsCheckerMock::new();
        let mut fn_config = fn_config_mock();
        fn_config.args_checker = DynArgsChecker::new(args_checker_mock.clone());
        fn_config.setup().check_call(automock::Arg::Any).call_base();
        let dyn_call = DynCall::new(CallMock::new());
        let arg_check_results = vec![
            ArgCheckResult::Ok(ArgCheckResultOk {
                arg_info: ArgInfo::new("quo", &1, "vadis".to_owned()),
            }),
            ArgCheckResult::Ok(ArgCheckResultOk {
                arg_info: ArgInfo::new("veridis", &2, "quo".to_owned()),
            }),
        ];
        args_checker_mock
            .setup()
            .as_IArgsChecker()
            .check(automock::Arg::Any)
            .returns(arg_check_results.clone());

        // Act
        let result = fn_config.check_call(&dyn_call);

        // Assert
        assert_eq!(result, arg_check_results);

        args_checker_mock
            .received()
            .as_IArgsChecker()
            .check(automock::Arg::ref_eq(&dyn_call), automock::Times::Once)
            .no_other_calls();
    }

    mod has_return_value_tests {
        use super::*;

        #[allow(non_camel_case_types)]
        pub struct Params<'a> {
            pub call_base: bool,
            pub return_value_sources: VecDeque<ReturnValueSource<'a>>,
            pub expected_result: bool,
        }
        pub fn test(
            Params {
                call_base,
                return_value_sources,
                expected_result,
            }: Params,
        ) {
            // Arrange
            let mut fn_config = fn_config_mock();
            fn_config.return_value_sources = return_value_sources;
            fn_config.call_base = call_base;
            fn_config.setup().has_return_value().call_base();

            // Act
            let result = fn_config.has_return_value();

            // Assert
            assert_eq!(result, expected_result)
        }
    }

    #[test]
    fn has_return_value_DoNotCallBaseAndNoReturnValueSources_ReturnsFalse() {
        has_return_value_tests::test(has_return_value_tests::Params {
            call_base: false,
            return_value_sources: VecDeque::new(),
            expected_result: false,
        });
    }

    #[test]
    fn has_return_value_CallBaseAndNoReturnValueSources_ReturnsTrue() {
        has_return_value_tests::test(has_return_value_tests::Params {
            call_base: true,
            return_value_sources: VecDeque::new(),
            expected_result: true,
        });
    }

    #[test]
    fn has_return_value_DoNotCallBaseAndSomeReturnValueSources_ReturnsTrue() {
        has_return_value_tests::test(has_return_value_tests::Params {
            call_base: false,
            return_value_sources: VecDeque::from(vec![ReturnValueSource::SingleTime(
                DynReturnValue::new(5),
            )]),
            expected_result: true,
        });
    }

    #[test]
    fn has_return_value_CallBaseAndSomeReturnValueSources_ReturnsTrue() {
        has_return_value_tests::test(has_return_value_tests::Params {
            call_base: true,
            return_value_sources: VecDeque::from(vec![ReturnValueSource::SingleTime(
                DynReturnValue::new(5),
            )]),
            expected_result: true,
        });
    }

    #[test]
    fn select_next_return_value_EmptyReturnValueSources_ReturnsNone() {
        // Arrange
        let mut fn_config = fn_config_mock();
        fn_config
            .setup()
            .select_next_return_value(automock::Arg::Any)
            .call_base();
        let dyn_call = DynCall::new(CallMock::new());

        // Act
        let result = fn_config.select_next_return_value(&dyn_call);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn select_next_return_value_FrontSingleTime_ReturnsSomeAndPopsFront() {
        // Arrange
        let mut fn_config = fn_config_mock();
        type TExistedReturnValue = &'static str;
        let existed_return_value: TExistedReturnValue = "whatever";
        let existed_return_value_source =
            ReturnValueSource::SingleTime(DynReturnValue::new(existed_return_value));
        type TReturnValue = i32;
        let return_value: TReturnValue = 5;
        let return_value_source = ReturnValueSource::SingleTime(DynReturnValue::new(return_value));
        fn_config
            .return_value_sources
            .push_front(existed_return_value_source);
        fn_config
            .return_value_sources
            .push_front(return_value_source);
        let initial_return_value_sources_length = fn_config.return_value_sources.len();
        fn_config
            .setup()
            .select_next_return_value(automock::Arg::Any)
            .call_base();
        let dyn_call = DynCall::new(CallMock::new());

        // Act
        let result = fn_config.select_next_return_value(&dyn_call);

        // Assert
        let dyn_return_value = result.expect("Result must contain return value.");
        let actual_return_value: TReturnValue = dyn_return_value.downcast_into();
        assert_eq!(actual_return_value, return_value);

        let expected_return_value_sources_length = initial_return_value_sources_length - 1;
        assert_eq!(
            fn_config.return_value_sources.len(),
            expected_return_value_sources_length
        );

        let actual_existed_dyn_return_value = match fn_config
            .return_value_sources
            .pop_front()
            .expect("Existed return value should exist.")
        {
            ReturnValueSource::SingleTime(x) => x,
            y => panic!(
                "Existed return value must be `ReturnValueSource::SingleTime`, was instead: {y:?}"
            ),
        };
        let actual_existed_return_value: TExistedReturnValue =
            actual_existed_dyn_return_value.downcast_into();
        assert_eq!(actual_existed_return_value, existed_return_value);
    }

    #[test]
    fn select_next_return_value_FrontPerpetual_ReturnsSomeButDoesNotPopFront() {
        // Arrange
        let mut fn_config = fn_config_mock();
        type TReturnValue = i32;
        let return_value: TReturnValue = 5;
        let return_value_source =
            ReturnValueSource::Perpetual(Box::new(move || DynReturnValue::new(return_value)));
        fn_config
            .return_value_sources
            .push_front(return_value_source);
        let initial_return_value_sources_length = fn_config.return_value_sources.len();
        fn_config
            .setup()
            .select_next_return_value(automock::Arg::Any)
            .call_base();
        let dyn_call = DynCall::new(CallMock::new());

        // Act
        let result = fn_config.select_next_return_value(&dyn_call);

        // Assert
        let dyn_return_value = result.expect("Result must contain return value.");
        let actual_return_value: TReturnValue = dyn_return_value.downcast_into();
        assert_eq!(actual_return_value, return_value);

        assert_eq!(
            fn_config.return_value_sources.len(),
            initial_return_value_sources_length
        );
    }

    #[test]
    fn select_next_return_value_FrontFactory_ReturnsSomeButDoesNotPopFront() {
        // Arrange
        let mut fn_config = fn_config_mock();
        type TReturnValue = i32;
        let return_value: TReturnValue = 5;
        return_value_source_factory::setup(automock::Arg::Any)
            .returns(DynReturnValue::new(return_value));
        let return_value_source = ReturnValueSource::Factory(Box::new(return_value_source_factory));
        fn_config
            .return_value_sources
            .push_front(return_value_source);
        let initial_return_value_sources_length = fn_config.return_value_sources.len();
        fn_config
            .setup()
            .select_next_return_value(automock::Arg::Any)
            .call_base();
        let dyn_arg_refs_tuple = dyn_arg_refs_tuple_mock();
        let dyn_arg_refs_tuple_id = dyn_arg_refs_tuple.id();
        let mut call_mock = CallMock::new();
        call_mock
            .setup()
            .as_ICall()
            .get_dyn_tuple_of_refs()
            .returns(dyn_arg_refs_tuple);
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        let result = fn_config.select_next_return_value(&dyn_call);

        // Assert
        let dyn_return_value = result.expect("Result must contain return value.");
        let actual_return_value: TReturnValue = dyn_return_value.downcast_into();
        assert_eq!(actual_return_value, return_value);

        assert_eq!(
            fn_config.return_value_sources.len(),
            initial_return_value_sources_length
        );

        call_mock
            .received()
            .as_ICall()
            .get_dyn_tuple_of_refs(automock::Times::Once)
            .no_other_calls();

        return_value_source_factory::received(
            automock::Arg::is(|x: &DynArgRefsTuple| x.id() == dyn_arg_refs_tuple_id),
            automock::Times::Once,
        )
        .no_other_calls();
    }

    #[test]
    fn get_callback_NoCallback_ReturnsNone() {
        // Arrange
        let maybe_callback = None;
        let mut fn_config = fn_config_mock();
        fn_config.maybe_callback = maybe_callback;
        fn_config.setup().get_callback().call_base();

        // Act
        let result = fn_config.get_callback();

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_callback_WithCallback_ReturnsSome<'am>() {
        // Arrange
        let maybe_callback = Some(Rc::new(RefCell::new(inner_callback))
            as Rc<RefCell<dyn FnMut(*const (), &DynCall<'am>)>>);
        let mut fn_config = fn_config_mock();
        fn_config.maybe_callback = maybe_callback;
        fn_config.setup().get_callback().call_base();
        let ptr = 1234 as *const ();
        let dyn_call = DynCall::new(CallMock::new());

        // Act
        let result = fn_config.get_callback();

        // Assert
        let callback = result.expect("Callback must be set.");
        inner_callback::received_nothing();
        callback.borrow_mut()(ptr, &dyn_call);
        inner_callback::received(ptr, automock::Arg::ref_eq(&dyn_call), automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn set_call_base_Ok() {
        // Arrange
        let mut fn_config = fn_config_mock();
        fn_config.call_base = false;
        fn_config.setup().set_call_base().call_base();

        // Act
        fn_config.set_call_base();

        // Assert
        assert!(fn_config.call_base);
    }

    #[test]
    fn should_call_base_Ok() {
        // Arrange
        let call_base = true;
        let mut fn_config = fn_config_mock();
        fn_config.call_base = call_base;
        fn_config.setup().should_call_base().call_base();

        // Act
        let result = fn_config.should_call_base();

        // Assert
        assert_eq!(result, call_base);
    }

    mod utilities {
        use super::*;

        pub fn fn_config_mock<'am>() -> FnConfig<'am, Mock> {
            FnConfig {
                _phantom_mock: PhantomData,
                args_checker: DynArgsChecker::new(ArgsCheckerMock::new()),
                return_value_sources: VecDeque::new(),
                calls: Vec::new(),
                maybe_callback: None,
                call_base: false,
                __mock_data: Default::default(),
            }
        }

        #[automock::mock]
        pub fn callback<TArgRefsTuple>(_: &MockArg, _: TArgRefsTuple) {
            unreachable!()
        }

        #[automock::mock]
        pub fn inner_callback<'am>(_: *const (), _: &DynCall<'am>) {
            unreachable!();
        }

        #[automock::mock]
        pub fn return_value_source_factory<'am>(_: DynArgRefsTuple<'am>) -> DynReturnValue<'am> {
            unreachable!()
        }
    }
}
