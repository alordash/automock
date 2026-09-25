use crate::fn_parameters::*;
use crate::infrastructure::*;
use crate::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

/// Controls behavior of mocked function.
pub struct FnConfigurator<
    'rs,
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
    fn_config: Rc<RefCell<FnConfig<'rs, TMock>>>,
    owner: &'rs TOwner,
    fn_callback_configurator: FnCallbackConfigurator<
        'rs,
        TMock,
        TOwner,
        TArgRefsTuple,
        TMockArg,
        PASSES_MOCK_TO_CALLBACK,
    >,
}

impl<
    'rs,
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
        'rs,
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
    pub(crate) fn new(fn_config: Rc<RefCell<FnConfig<'rs, TMock>>>, owner: &'rs TOwner) -> Self {
        Self {
            _phantom_return_value: PhantomData,
            fn_config: fn_config.clone(),
            owner,
            fn_callback_configurator: FnCallbackConfigurator::new(fn_config.clone(), owner),
        }
    }
}

impl<
    'rs,
    TMock,
    TOwner,
    TArgRefsTuple,
    TReturnValue,
    TMockArg,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
>
    FnConfigurator<
        'rs,
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
    ) -> &FnCallbackConfigurator<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
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
    ) -> &FnCallbackConfigurator<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let return_value_sources = return_values
            .into_iter()
            .map(|x| transmute_lifetime!(DynReturnValue::new(x)))
            .map(ReturnValueSource::SingleTime);
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
    ) -> &FnCallbackConfigurator<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    where
        TReturnValue: 'rs + 'a + IReturnValue<'a> + Clone,
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
        f: impl Fn(TArgRefsTuple) -> TReturnValue + 'rs,
    ) -> &FnCallbackConfigurator<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    {
        let return_value_source = ReturnValueSource::Factory(Box::new(
            move |dyn_arg_refs_tuple: DynArgRefsTuple<'rs>| {
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

impl<'rs, TMock, TOwner, TArgRefsTuple, TReturnValue, TMockArg, const SUPPORTS_BASE_CALLING: bool>
    FnConfigurator<
        'rs,
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
    pub fn does(&self, mut callback: impl FnMut(TArgRefsTuple) + 'static) -> &'rs TOwner {
        let callback_with_mock =
            move |_mock: &TMock, arg_refs_tuple: TArgRefsTuple| callback(arg_refs_tuple);
        self.fn_config.borrow_mut().set_callback(callback_with_mock);
        return self.owner;
    }
}

impl<
    'rs,
    TMock,
    TOwner,
    TArgRefsTuple,
    TReturnValue,
    TMockArg,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
>
    FnConfigurator<
        'rs,
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
    pub fn does(&self, callback: impl FnMut(&TMockArg, TArgRefsTuple) + 'static) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<
    'rs,
    TMock,
    TOwner,
    TArgRefsTuple,
    TReturnValue,
    TMockArg,
    const HAS_RETURN_VALUE: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
>
    FnConfigurator<
        'rs,
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
    ) -> &FnCallbackConfigurator<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
    {
        self.fn_config.borrow_mut().set_call_base();
        return &self.fn_callback_configurator;
    }
}
