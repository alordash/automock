use crate::infrastructure::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

/// Controls callback of mocked function that has return value.
pub struct FnCallbackConfigurator<
    'am,
    TMock,
    TOwner,
    TArgRefsTuple,
    TMockArg,
    const PASSES_MOCK_TO_CALLBACK: bool,
> {
    _phantom_args_tuple: PhantomData<TArgRefsTuple>,
    _phantom_mock_arg: PhantomData<TMockArg>,
    fn_config: Rc<RefCell<FnConfig<'am, TMock>>>,
    owner: &'am TOwner,
}

impl<'am, TMock, TOwner, TArgRefsTuple, TMockArg, const PASSES_MOCK_TO_CALLBACK: bool>
    FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
{
    pub(crate) fn new(fn_config: Rc<RefCell<FnConfig<'am, TMock>>>, owner: &'am TOwner) -> Self {
        Self {
            _phantom_args_tuple: PhantomData,
            _phantom_mock_arg: PhantomData,
            fn_config,
            owner,
        }
    }
}

impl<'am, TMock, TOwner, TArgRefsTuple, TMockArg>
    FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, false>
{
    pub fn and_does(&self, mut callback: impl FnMut(TArgRefsTuple) + 'static) -> &'am TOwner
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

impl<'am, TMock, TOwner, TArgRefsTuple, TMockArg>
    FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, true>
{
    pub fn and_does(&self, callback: impl FnMut(&TMockArg, TArgRefsTuple) + 'am) -> &'am TOwner
    where
        TMockArg: 'am,
        TArgRefsTuple: 'am,
    {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'am, TMock, TOwner, TArgRefsTuple, TMockArg, const PASSES_MOCK_TO_CALLBACK: bool> Deref
    for FnCallbackConfigurator<'am, TMock, TOwner, TArgRefsTuple, TMockArg, PASSES_MOCK_TO_CALLBACK>
{
    type Target = TOwner;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}
