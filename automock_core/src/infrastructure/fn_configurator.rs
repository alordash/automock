use crate::fn_parameters::*;
use crate::infrastructure::*;
use crate::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

/// Controls behavior of mocked function.
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
    owner: &'am TOwner,
    fn_callback_configurator: FnCallbackConfigurator<
        'am,
        TMock,
        TOwner,
        TArgRefsTuple,
        TMockArg,
        PASSES_MOCK_TO_CALLBACK,
    >,
}

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
            owner,
            fn_callback_configurator: FnCallbackConfigurator::new(fn_config.clone(), owner),
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
    /// Sets return value of this function. This value will be returned only once.
    pub fn returns<'a>(
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

    /// Sets return value of this function. Clones of this value will be returned indefinitely. The
    /// provided values itself will never be returned.
    pub fn always_returns<'a>(
        &self,
        return_value: TReturnValue,
    ) -> &FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TReturnValue: 'am + 'a + IReturnValue<'a> + Clone,
    {
        let return_value_source = ReturnValueSource::Perpetual(Box::new(move || {
            transmute_lifetime!(DynReturnValue::new(return_value.clone()))
        }));
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_callback_configurator;
    }

    /// Sets return value of this function using factory. Factory constructs new return value using
    /// references to source function argument values. Never ends.
    pub fn returns_with<'a>(
        &self,
        f: impl Fn(TArgRefsTuple) -> TReturnValue + 'am,
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

impl<'am, TMock, TOwner, TArgRefsTuple, TReturnValue, TMockArg, const SUPPORTS_BASE_CALLING: bool>
    FnConfigurator<
        'am,
        TMock,
        TOwner,
        TArgRefsTuple,
        TReturnValue,
        TMockArg,
        false,
        SUPPORTS_BASE_CALLING,
        false,
    >
{
    /// Adds callback that is called after source function was called. Callback receives references
    /// to source function argument values. If function has enabled base implementation, this
    /// callback is called BEFORE the base implementation.
    pub fn does(&self, mut callback: impl FnMut(TArgRefsTuple) + 'static) -> &'am TOwner
    where
        TMock: 'am,
        TArgRefsTuple: 'am,
    {
        let callback_with_mock =
            move |_mock: &TMock, arg_refs_tuple: TArgRefsTuple| callback(arg_refs_tuple);
        self.fn_config.borrow_mut().set_callback(callback_with_mock);
        return self.owner;
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
    const SUPPORTS_BASE_CALLING: bool,
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
        true,
    >
{
    /// Adds callback that is called after source function was called. Callback receives reference
    /// to mock object and references to source function argument values. If function has enabled
    /// base implementation, this callback is called BEFORE the base implementation.
    pub fn does(&self, callback: impl FnMut(&TMockArg, TArgRefsTuple) + 'static) -> &'am TOwner
    where
        TMockArg: 'am,
        TArgRefsTuple: 'am,
    {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
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
    fn returns_Ok() {
        // Arrange
        let owner = Owner;
        let fn_configurator = fn_configurator::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let return_value = ReturnValue(5);

        // Act
        fn_configurator.returns(return_value);

        // Assert
        fn_configurator
            .fn_config
            .borrow_mut()
            .received()
            .add_return_value_source(automock::Arg::is(|return_value_source| {
                let dyn_return_value = match return_value_source {
                    ReturnValueSource::SingleTime(x) => x,
                    _ => panic!("Return value source must be `SingleTime`, was instead: {return_value_source:?}")
                };
                let actual_return_value: &ReturnValue = dyn_return_value.downcast_to();
                assert_eq!(*actual_return_value, return_value);
                return true;
            }), automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn returns_many_Ok() {
        // Arrange
        let owner = Owner;
        let fn_configurator = fn_configurator::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let return_values = [ReturnValue(5), ReturnValue(10)];

        // Act
        fn_configurator.returns_many(return_values);

        // Assert
        fn_configurator
            .fn_config
            .borrow_mut()
            .received()
            .add_return_value_sources(automock::Arg::is(|return_value_sources: &Vec<ReturnValueSource>| {
                let actual_return_values: Vec<ReturnValue> = return_value_sources.iter().map(|return_value_source|
                    match return_value_source {
                        ReturnValueSource::SingleTime(x) => *x.downcast_to(),
                        _ => panic!("Return value source must be `SingleTime`, was instead: {return_value_source:?}")
                    }).collect();
                assert_eq!(actual_return_values, return_values);
                return true;
            }), automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn always_returns_Ok() {
        // Arrange
        let owner = Owner;
        let fn_configurator = fn_configurator::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let return_value = ReturnValue(5);

        // Act
        fn_configurator.always_returns(return_value);

        // Assert
        fn_configurator
            .fn_config
            .borrow_mut()
            .received()
            .add_return_value_source(automock::Arg::is(|return_value_source| {
                let perpetual_factory = match return_value_source {
                    ReturnValueSource::Perpetual(x) => x,
                    _ => panic!("Return value source must be `Perpetual`, was instead: {return_value_source:?}")
                };
                let actual_return_value: ReturnValue = perpetual_factory().downcast_into();
                assert_eq!(actual_return_value, return_value);
                return true;
            }), automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn returns_with_Ok() {
        // Arrange
        let owner = Owner;
        let fn_configurator = fn_configurator::<true, IRRELEVANT, IRRELEVANT>(&owner);
        let return_value = ReturnValue(5);
        returns_with_factory::setup(automock::Arg::Any).returns(return_value);

        // Act
        fn_configurator.returns_with(returns_with_factory);

        // Assert
        fn_configurator
            .fn_config
            .borrow_mut()
            .received()
            .add_return_value_source(automock::Arg::is(|return_value_source| {
                let factory = match return_value_source {
                    ReturnValueSource::Factory(x) => x,
                    _ => panic!("Return value source must be `Factory`, was instead: {return_value_source:?}")
                };

                let mut dyn_arg_refs_tuple_mock = dyn_arg_refs_tuple_mock();
                let arg_refs_tuple = ArgRefsTuple(10);
                dyn_arg_refs_tuple_mock.setup().downcast_into::<ArgRefsTuple>().returns(arg_refs_tuple.clone());

                let actual_return_value: ReturnValue = factory(dyn_arg_refs_tuple_mock).downcast_into();
                assert_eq!(actual_return_value, return_value);

                returns_with_factory::received(arg_refs_tuple, automock::Times::Once).no_other_calls();

                return true;
            }), automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn does_WithoutMockObject_Ok<'am>() {
        // Arrange
        let owner = Owner;
        let fn_configurator = fn_configurator::<false, IRRELEVANT, false>(&owner);

        // Act
        fn_configurator.does(callback_without_mock_object);

        // Assert
        fn_configurator
            .fn_config
            .borrow_mut()
            .received()
            .set_callback(
                automock::Arg::is_mut(|callback: &mut Box<dyn FnMut(&'am Mock, ArgRefsTuple)>| {
                    let arg_refs_tuple = ArgRefsTuple(10);
                    callback.as_mut()(&Mock, arg_refs_tuple);
                    callback_without_mock_object::received(arg_refs_tuple, automock::Times::Once)
                        .no_other_calls();
                    return true;
                }),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn does_WithMockObject_Ok<'am>() {
        // Arrange
        let owner = Owner;
        let fn_configurator = fn_configurator::<IRRELEVANT, IRRELEVANT, true>(&owner);

        // Act
        fn_configurator.does(callback_with_mock_object);

        // Assert
        fn_configurator
            .fn_config
            .borrow_mut()
            .received()
            .set_callback(
                automock::Arg::is_mut(
                    |callback: &mut Box<dyn FnMut(&'am MockArg, ArgRefsTuple)>| {
                        let mock_arg = MockArg(5);
                        let arg_refs_tuple = ArgRefsTuple(10);
                        callback.as_mut()(transmute_lifetime!(&mock_arg), arg_refs_tuple);
                        callback_with_mock_object::received(
                            automock::Arg::ref_eq(&mock_arg),
                            arg_refs_tuple,
                            automock::Times::Once,
                        )
                        .no_other_calls();
                        return true;
                    },
                ),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    fn call_base_Ok<'am>() {
        // Arrange
        let owner = Owner;
        let fn_configurator = fn_configurator::<IRRELEVANT, true, IRRELEVANT>(&owner);

        // Act
        let result = fn_configurator.call_base();

        // Assert
        assert!(core::ptr::eq(
            result,
            &fn_configurator.fn_callback_configurator
        ));

        fn_configurator
            .fn_config
            .borrow_mut()
            .received()
            .set_call_base(automock::Times::Once)
            .no_other_calls();
    }

    pub mod utilities {
        use super::*;

        pub struct Mock;
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub struct ReturnValue(pub i32);

        pub fn fn_configurator<
            const HAS_RETURN_VALUE: bool,
            const SUPPORTS_BASE_CALLING: bool,
            const PASSES_MOCK_TO_CALLBACK: bool,
        >(
            owner: &Owner,
        ) -> FnConfigurator<
            'static,
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
                owner: transmute_lifetime!(owner),
                fn_callback_configurator: fn_callback_configurator(owner),
            }
        }

        #[automock::mock]
        pub(super) fn returns_with_factory(_: ArgRefsTuple) -> ReturnValue {
            unreachable!()
        }

        #[automock::mock]
        pub(super) fn callback_without_mock_object(_: ArgRefsTuple) {
            unreachable!()
        }

        #[automock::mock]
        pub(super) fn callback_with_mock_object(_: &MockArg, _: ArgRefsTuple) {
            unreachable!()
        }
    }
}
