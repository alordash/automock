use crate::fn_parameters::*;
use crate::infrastructure::*;
use crate::times::*;
use crate::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

/// Controls behavior of mocked function.
#[cfg_attr(test, automock::mock)]
pub struct FnConfigurator<
    'am,
    TMock,
    TOwner,
    TArgRefsTuple,
    TReturnValue,
    TMockArg,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> {
    _phantom_return_value: PhantomData<TReturnValue>,
    fn_config: Rc<RefCell<FnConfig<'am, TMock>>>,
    fn_callback_configurator: FnCallbackConfigurator<
        'am,
        TMock,
        TOwner,
        TArgRefsTuple,
        TMockArg,
        PASSES_MOCK_TO_CALLBACK,
    >,
}

#[cfg_attr(test, automock::mock)]
impl<
    'am,
    TMock,
    TOwner,
    TArgRefsTuple,
    TReturnValue,
    TMockArg,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
>
    FnConfigurator<
        'am,
        TMock,
        TOwner,
        TArgRefsTuple,
        TReturnValue,
        TMockArg,
        HAS_RETURN_VALUE,
        SUPPORTS_BASE_CALLING,
        PASSES_MOCK_TO_CALLBACK,
    >
{
    pub(crate) fn new(fn_config: Rc<RefCell<FnConfig<'am, TMock>>>, owner: &'am TOwner) -> Self {
        Self {
            _phantom_return_value: PhantomData,
            fn_config: fn_config.clone(),
            fn_callback_configurator: FnCallbackConfigurator::new(fn_config, owner),
        }
    }
}

impl<
    'am,
    TMock,
    TOwner,
    TArgRefsTuple,
    TReturnValue,
    TMockArg,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
>
    FnConfigurator<
        'am,
        TMock,
        TOwner,
        TArgRefsTuple,
        TReturnValue,
        TMockArg,
        true,
        SUPPORTS_BASE_CALLING,
        PASSES_MOCK_TO_CALLBACK,
    >
{
    /// Sets return value of this function. This value will be returned multiple [`times`].
    pub fn returns<'a>(
        &self,
        return_value: TReturnValue,
        times: Times,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TReturnValue: Clone + IReturnValue<'a> + 'a,
    {
        let return_value_sources = core::iter::repeat_n(return_value, times.0)
            .map(|x| ReturnValueSource::SingleTime(transmute_lifetime!(DynReturnValue::new(x))));
        self.fn_config
            .borrow_mut()
            .add_return_value_sources(return_value_sources);
        return &self.fn_callback_configurator;
    }

    /// Sets return value of this function. This value will be returned only once.
    pub fn returns_once<'a>(
        &self,
        return_value: TReturnValue,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let dyn_return_value = transmute_lifetime!(DynReturnValue::new(return_value));
        let return_value_source = ReturnValueSource::SingleTime(dyn_return_value);
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_callback_configurator;
    }

    /// Sets multiple return values of this function. These values will be returned only once in the
    /// given order.
    pub fn returns_many<'a>(
        &self,
        return_values: impl IntoIterator<Item = TReturnValue>,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let return_value_sources: Vec<_> = return_values
            .into_iter()
            .map(|x| transmute_lifetime!(DynReturnValue::new(x)))
            .map(ReturnValueSource::SingleTime)
            .collect();
        self.fn_config
            .borrow_mut()
            .add_return_value_sources(return_value_sources);
        return &self.fn_callback_configurator;
    }

    /// Sets return value of this function using factory. Factory constructs new return value using
    /// references to source function argument values. Constructed value will be returned multiple
    /// [`times`].
    pub fn returns_with(
        &self,
        f: impl Clone + FnOnce(TArgRefsTuple) -> TReturnValue + 'am,
        times: Times,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    {
        let factory = Box::new(move |dyn_arg_refs_tuple: DynArgRefsTuple<'am>| {
            let arg_refs_tuple: TArgRefsTuple = dyn_arg_refs_tuple.downcast_into::<TArgRefsTuple>();
            let result = f(arg_refs_tuple);
            return transmute_lifetime!(DynReturnValue::new(result));
        });
        let return_value_sources =
            core::iter::repeat_n(factory, times.0).map(|x| ReturnValueSource::Factory(x));
        self.fn_config
            .borrow_mut()
            .add_return_value_sources(return_value_sources);
        return &self.fn_callback_configurator;
    }

    /// Sets return value of this function using factory. Factory constructs new return value using
    /// references to source function argument values. Constructed value will be returned only once.
    pub fn returns_once_with(
        &self,
        f: impl FnOnce(TArgRefsTuple) -> TReturnValue + 'am,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    {
        let return_value_source = ReturnValueSource::Factory(Box::new(
            move |dyn_arg_refs_tuple: DynArgRefsTuple<'am>| {
                let arg_refs_tuple: TArgRefsTuple =
                    dyn_arg_refs_tuple.downcast_into::<TArgRefsTuple>();
                let result = f(arg_refs_tuple);
                return transmute_lifetime!(DynReturnValue::new(result));
            },
        ));
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_callback_configurator;
    }
}

impl<
    'am,
    TMock,
    TOwner,
    TArgRefsTuple,
    TReturnValue: Default,
    TMockArg,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
>
    FnConfigurator<
        'am,
        TMock,
        TOwner,
        TArgRefsTuple,
        TReturnValue,
        TMockArg,
        false,
        SUPPORTS_BASE_CALLING,
        PASSES_MOCK_TO_CALLBACK,
    >
{
    /// Specifies how many times this function must be called.
    pub fn expect(
        &self,
        times: Times,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TMock: 'am,
        TArgRefsTuple: 'am,
    {
        let return_values = core::iter::repeat_with(TReturnValue::default)
            .take(times.0)
            .map(|x| transmute_lifetime!(DynReturnValue::new(x)))
            .map(ReturnValueSource::SingleTime);
        self.fn_config
            .borrow_mut()
            .add_return_value_sources(return_values);
        return &self.fn_callback_configurator;
    }

    /// Specifies that this function must be called exactly once.
    pub fn expect_once(
        &self,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TMock: 'am,
        TArgRefsTuple: 'am,
    {
        self.fn_config
            .borrow_mut()
            .add_return_value_source(ReturnValueSource::SingleTime(transmute_lifetime!(
                DynReturnValue::new(TReturnValue::default())
            )));
        return &self.fn_callback_configurator;
    }
}

impl<
    'am,
    TMock,
    TOwner,
    TArgRefsTuple,
    TReturnValue,
    TMockArg,
    const HAS_RETURN_VALUE: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
>
    FnConfigurator<
        'am,
        TMock,
        TOwner,
        TArgRefsTuple,
        TReturnValue,
        TMockArg,
        HAS_RETURN_VALUE,
        true,
        PASSES_MOCK_TO_CALLBACK,
    >
{
    /// Instructs this function to call it's base implementation. If the function has return value,
    /// then it will return value returned by base implementation.
    pub fn call_base(
        &self,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    {
        self.fn_config.borrow_mut().set_call_base();
        return &self.fn_callback_configurator;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use automock::Mockable;
    use fn_callback_configurator::tests::utilities::*;
    use fn_config::tests::utilities::*;
    use fn_parameters::dyn_arg_refs_tuple::tests::utilities::*;
    use utilities::*;

    const IRRELEVANT: bool = false;

    #[test]
    fn returns_once_Ok() {
        // Arrange
        let owner = Owner;
        let fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let return_value = ReturnValue(5);
        fn_configurator_mock
            .fn_config
            .borrow_mut()
            .setup()
            .add_return_value_source(automock::Arg::is(
                |source: &ReturnValueSource| match source {
                    ReturnValueSource::SingleTime(value) => {
                        *value.downcast_to::<ReturnValue>() == return_value
                    }
                    _ => false,
                },
            ))
            .expect_once();

        // Act
        fn_configurator_mock.returns_once(return_value);

        // Assert
        fn_configurator_mock.fn_config.borrow_mut().verify_calls();
    }

    #[test]
    fn returns_many_Ok() {
        // Arrange
        let owner = Owner;
        let fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let return_values = [ReturnValue(5), ReturnValue(10)];
        fn_configurator_mock
            .fn_config
            .borrow_mut()
            .setup()
            .add_return_value_sources(automock::Arg::is(|sources: &Vec<ReturnValueSource>| {
                let values: Vec<_> = sources
                    .iter()
                    .map(|source| match source {
                        ReturnValueSource::SingleTime(value) => *value.downcast_to::<ReturnValue>(),
                        _ => panic!("Return value source must be SingleTime."),
                    })
                    .collect();
                values == return_values
            }))
            .expect_once();

        // Act
        fn_configurator_mock.returns_many(return_values);

        // Assert
        fn_configurator_mock.fn_config.borrow_mut().verify_calls();
    }

    #[test]
    fn returns_once_with_Ok() {
        // Arrange
        let owner = Owner;
        let fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let return_value = ReturnValue(5);
        let arg_refs_tuple = ArgRefsTuple(10);
        returns_with_factory::setup(arg_refs_tuple).returns_once(return_value);
        let mut dyn_arg_refs_tuple = dyn_arg_refs_tuple_mock();
        dyn_arg_refs_tuple
            .setup()
            .downcast_into::<ArgRefsTuple>()
            .returns_once(arg_refs_tuple);
        fn_configurator_mock
            .fn_config
            .borrow_mut()
            .setup()
            .add_return_value_source(automock::Arg::Any)
            .call_base();

        // Act
        fn_configurator_mock.returns_once_with(returns_with_factory);

        let source = fn_configurator_mock
            .fn_config
            .borrow_mut()
            .return_value_sources
            .pop_front()
            .expect("Factory must be stored.");
        let factory = match source {
            ReturnValueSource::Factory(factory) => factory,
            _ => panic!("Return value source must be Factory."),
        };
        let actual_return_value: ReturnValue = factory(dyn_arg_refs_tuple).downcast_into();

        // Assert
        assert_eq!(actual_return_value, return_value);
        returns_with_factory::verify_calls();
    }

    #[test]
    fn call_base_Ok() {
        // Arrange
        let owner = Owner;
        let fn_configurator_mock = fn_configurator_mock::<IRRELEVANT, true, IRRELEVANT>(&owner);
        fn_configurator_mock
            .fn_config
            .borrow_mut()
            .setup()
            .set_call_base()
            .expect_once();

        // Act
        let result = fn_configurator_mock.call_base();

        // Assert
        assert!(core::ptr::eq(
            result,
            &fn_configurator_mock.fn_callback_configurator
        ));

        fn_configurator_mock.fn_config.borrow_mut().verify_calls();
    }

    pub mod utilities {
        use super::*;

        pub struct Mock;
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub struct ReturnValue(pub i32);

        pub fn fn_configurator_mock<
            'a,
            const HAS_RETURN_VALUE: bool,
            const SUPPORTS_BASE_CALLING: bool,
            const PASSES_MOCK_TO_CALLBACK: bool,
        >(
            owner: &'a Owner,
        ) -> FnConfigurator<
            'a,
            Mock,
            Owner,
            ArgRefsTuple,
            ReturnValue,
            MockArg,
            HAS_RETURN_VALUE,
            SUPPORTS_BASE_CALLING,
            PASSES_MOCK_TO_CALLBACK,
        > {
            FnConfigurator {
                _phantom_return_value: PhantomData,
                fn_config: Rc::new(RefCell::new(fn_config_mock())),
                fn_callback_configurator: fn_callback_configurator(owner),
                __mock_data: Default::default(),
            }
        }

        #[automock::mock]
        pub(super) fn returns_with_factory(_: ArgRefsTuple) -> ReturnValue {
            unreachable!()
        }
    }
}
