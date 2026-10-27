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

#[cfg_attr(test, automock::mock)]
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
        let return_values = core::iter::repeat_n(return_value, times.0);
        return self.returns_many(return_values);
    }

    /// Sets return value of this function. This value will be returned only once.
    pub fn returns_once<'a>(
        &self,
        return_value: TReturnValue,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        return self.returns_many([return_value]);
    }

    /// Sets multiple return values of this function. These values will be returned only once in the
    /// given order.
    pub fn returns_many<'a, RV: IntoIterator<Item = TReturnValue>>(
        &self,
        return_values: RV,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let return_value_sources: Vec<_> = return_values
            .into_iter()
            .map(|x| transmute_lifetime!(DynReturnValue::new(x)))
            .map(ReturnValueSource::SingleTime)
            .collect();
        return self.returns_many_core(return_value_sources);
    }

    /// Sets return value of this function using factory. Factory constructs new return value using
    /// references to source function argument values. Constructed value will be returned multiple
    /// [`times`].
    pub fn returns_with<F: Clone + FnOnce(TArgRefsTuple) -> TReturnValue + 'am>(
        &self,
        f: F,
        times: Times,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TArgRefsTuple: 'am,
        TReturnValue: 'am,
    {
        let fs: Vec<_> = core::iter::repeat_n(f, times.0)
            .map(|x| Box::new(x) as Box<dyn FnOnce(TArgRefsTuple) -> TReturnValue>)
            .collect();
        return self.returns_with_many(fs);
    }

    /// Sets return value of this function using factory. Factory constructs new return value using
    /// references to source function argument values. Constructed value will be returned only once.
    pub fn returns_once_with(
        &self,
        f: impl FnOnce(TArgRefsTuple) -> TReturnValue + 'am,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TArgRefsTuple: 'am,
        TReturnValue: 'am,
    {
        return self
            .returns_with_many([Box::new(f) as Box<dyn FnOnce(TArgRefsTuple) -> TReturnValue>]);
    }

    fn returns_with_many<
        FS: IntoIterator<Item = Box<dyn FnOnce(TArgRefsTuple) -> TReturnValue + 'am>>,
    >(
        &self,
        fs: FS,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TArgRefsTuple: 'am,
        TReturnValue: 'am,
    {
        let return_value_sources: Vec<_> = fs
            .into_iter()
            .map(|f| {
                Box::new(move |dyn_arg_refs_tuple: DynArgRefsTuple<'am>| {
                    let arg_refs_tuple: TArgRefsTuple =
                        dyn_arg_refs_tuple.downcast_into::<TArgRefsTuple>();
                    let result = f(arg_refs_tuple);
                    return DynReturnValue::new(result);
                })
                    as Box<dyn FnOnce(DynArgRefsTuple<'am>) -> DynReturnValue<'am> + 'am>
            })
            .map(ReturnValueSource::Factory)
            .collect();
        return self.returns_many_core(return_value_sources);
    }

    fn returns_many_core<RVS: IntoIterator<Item = ReturnValueSource<'am>>>(
        &self,
        return_value_sources: RVS,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    {
        self.fn_config
            .borrow_mut()
            .add_return_value_sources(return_value_sources);
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
    use core::assert_matches;
    use fn_callback_configurator::tests::utilities::*;
    use fn_config::tests::utilities::*;
    use utilities::*;

    const IRRELEVANT: bool = false;

    #[test]
    fn returns_Ok() {
        // Arrange
        let owner = Owner;
        let mut fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let expected_fn_callback_configurator = fn_callback_configurator::<IRRELEVANT>(&owner);
        let return_value = ReturnValue(5);
        let return_values = [return_value; 3];
        fn_configurator_mock
            .setup()
            .returns(return_value, 3.times())
            .call_base()
            .returns_many(automock::Arg::is(
                |values: &std::iter::RepeatN<ReturnValue>| {
                    let actual_return_values: Vec<_> = values.clone().collect();
                    let result = actual_return_values == return_values;
                    return result;
                },
            ))
            .returns_once(&expected_fn_callback_configurator);

        // Act
        let result = fn_configurator_mock.returns(return_value, 3.times());

        // Assert
        assert!(core::ptr::eq(result, &expected_fn_callback_configurator));
        fn_configurator_mock.verify_calls();
    }

    #[test]
    fn returns_once_Ok() {
        // Arrange
        let owner = Owner;
        let mut fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let expected_fn_callback_configurator = fn_callback_configurator::<IRRELEVANT>(&owner);
        let return_value = ReturnValue(5);
        fn_configurator_mock
            .setup()
            .returns_once(return_value)
            .call_base()
            .returns_many([return_value])
            .returns_once(&expected_fn_callback_configurator);

        // Act
        let result = fn_configurator_mock.returns_once(return_value);

        // Assert
        assert!(core::ptr::eq(result, &expected_fn_callback_configurator));
        fn_configurator_mock.verify_calls();
    }

    #[test]
    fn returns_many_Ok() {
        // Arrange
        let owner = Owner;
        let mut fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let expected_fn_callback_configurator = fn_callback_configurator::<IRRELEVANT>(&owner);
        let return_values = [ReturnValue(5), ReturnValue(10)];
        fn_configurator_mock
            .setup()
            .returns_many(return_values)
            .call_base()
            .returns_many_core(automock::Arg::is(|sources: &Vec<ReturnValueSource>| {
                let values: Vec<_> = sources
                    .iter()
                    .map(|source| {
                        assert_matches!(source, ReturnValueSource::SingleTime(_));
                        let ReturnValueSource::SingleTime(value) = source else {
                            unreachable!()
                        };
                        *value.downcast_to::<ReturnValue>()
                    })
                    .collect();
                let result = values == return_values;
                return result;
            }))
            .returns_once(&expected_fn_callback_configurator);

        // Act
        let result = fn_configurator_mock.returns_many(return_values);

        // Assert
        assert!(core::ptr::eq(result, &expected_fn_callback_configurator));
        fn_configurator_mock.verify_calls();
    }

    #[test]
    fn returns_with_Ok() {
        // Arrange
        let owner = Owner;
        let mut fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let expected_fn_callback_configurator = fn_callback_configurator::<IRRELEVANT>(&owner);
        let arg_refs_tuple = ArgRefsTuple(10);
        let return_values = [ReturnValue(11); 2];
        let factory: fn(ArgRefsTuple) -> ReturnValue = |arg_refs_tuple| {
            let result = ReturnValue(arg_refs_tuple.0 + 1);
            return result;
        };
        fn_configurator_mock
            .setup()
            .returns_with(
                automock::Arg::<fn(ArgRefsTuple) -> ReturnValue>::Any,
                2.times(),
            )
            .call_base()
            .returns_with_many(automock::Arg::is_mut(
                |fs: &mut Vec<Box<dyn FnOnce(ArgRefsTuple) -> ReturnValue>>| {
                    let actual_return_values: Vec<_> = fs
                        .drain(..)
                        .map(|f| {
                            let result = f(arg_refs_tuple);
                            return result;
                        })
                        .collect();
                    let result = actual_return_values == return_values;
                    return result;
                },
            ))
            .returns_once(&expected_fn_callback_configurator);

        // Act
        let result = fn_configurator_mock.returns_with(factory, 2.times());

        // Assert
        assert!(core::ptr::eq(result, &expected_fn_callback_configurator));
        fn_configurator_mock.verify_calls();
    }

    #[test]
    fn returns_once_with_Ok() {
        // Arrange
        let owner = Owner;
        let mut fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let expected_fn_callback_configurator = fn_callback_configurator::<IRRELEVANT>(&owner);
        let return_value = ReturnValue(5);
        let arg_refs_tuple = ArgRefsTuple(10);
        fn_configurator_mock
            .setup()
            .returns_once_with(automock::Arg::Any)
            .call_base()
            .returns_with_many(automock::Arg::is_mut(
                |fs: &mut [Box<dyn FnOnce(ArgRefsTuple) -> ReturnValue>; 1]| {
                    let factory = core::mem::replace(&mut fs[0], Box::new(|_| unreachable!()));
                    let actual_return_value = factory(arg_refs_tuple);
                    let result = actual_return_value == return_value;
                    return result;
                },
            ))
            .returns_once(&expected_fn_callback_configurator);

        // Act
        let result = fn_configurator_mock.returns_once_with(move |_| return_value);

        // Assert
        assert!(core::ptr::eq(result, &expected_fn_callback_configurator));
        fn_configurator_mock.verify_calls();
    }

    #[test]
    fn returns_with_many_Ok() {
        // Arrange
        let owner = Owner;
        let mut fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let expected_fn_callback_configurator = fn_callback_configurator::<IRRELEVANT>(&owner);
        let fs: [Box<dyn FnOnce(ArgRefsTuple) -> ReturnValue>; 2] =
            [Box::new(|_| unreachable!()), Box::new(|_| unreachable!())];
        fn_configurator_mock
            .setup()
            .returns_with_many::<[Box<dyn FnOnce(ArgRefsTuple) -> ReturnValue>; 2]>(
                automock::Arg::Any,
            )
            .call_base()
            .returns_many_core(automock::Arg::is(|sources: &Vec<ReturnValueSource>| {
                let result = sources.len() == 2
                    && sources
                        .iter()
                        .all(|source| matches!(source, ReturnValueSource::Factory(_)));
                return result;
            }))
            .returns_once(&expected_fn_callback_configurator);

        // Act
        let result = fn_configurator_mock.returns_with_many(fs);

        // Assert
        assert!(core::ptr::eq(result, &expected_fn_callback_configurator));
        fn_configurator_mock.verify_calls();
    }

    #[test]
    fn returns_many_core_Ok() {
        // Arrange
        let owner = Owner;
        let mut fn_configurator_mock = fn_configurator_mock::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let return_values = [ReturnValue(5), ReturnValue(10)];
        let return_value_sources: Vec<_> = return_values
            .into_iter()
            .map(DynReturnValue::new)
            .map(ReturnValueSource::SingleTime)
            .collect();
        fn_configurator_mock
            .setup()
            .returns_many_core::<Vec<ReturnValueSource>>(automock::Arg::Any)
            .call_base();
        fn_configurator_mock
            .fn_config
            .borrow_mut()
            .setup()
            .add_return_value_sources(automock::Arg::is(|sources: &Vec<ReturnValueSource>| {
                let actual_return_values: Vec<_> = sources
                    .iter()
                    .map(|source| {
                        assert_matches!(source, ReturnValueSource::SingleTime(_));
                        let ReturnValueSource::SingleTime(value) = source else {
                            unreachable!()
                        };
                        *value.downcast_to::<ReturnValue>()
                    })
                    .collect();
                let result = actual_return_values == return_values;
                return result;
            }))
            .expect_once();

        // Act
        let result = fn_configurator_mock.returns_many_core(return_value_sources);

        // Assert
        assert!(core::ptr::eq(
            result,
            &fn_configurator_mock.fn_callback_configurator
        ));
        fn_configurator_mock.verify_calls();
        fn_configurator_mock.fn_config.borrow_mut().verify_calls();
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
            const HAS_RETURN_VALUE: bool,
            const SUPPORTS_BASE_CALLING: bool,
            const PASSES_MOCK_TO_CALLBACK: bool,
        >(
            owner: &Owner,
        ) -> FnConfigurator<
            '_,
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
    }
}
