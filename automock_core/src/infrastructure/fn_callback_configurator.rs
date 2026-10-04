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

#[cfg(test)]
pub(crate) mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use crate::transmute_lifetime;
    use automock::Mockable;
    use fn_config::tests::utilities::*;
    use fn_configurator::tests::utilities::*;
    use utilities::*;

    const IRRELEVANT: bool = false;

    #[test]
    #[allow(clippy::extra_unused_lifetimes)]
    fn and_does_WithoutMock_Ok<'am>() {
        // Arrange
        let owner = Owner;
        let fn_callback_configurator = fn_callback_configurator::<false>(&owner);
        let arg_refs_tuple = ArgRefsTuple(5);

        // Act
        let result = fn_callback_configurator.and_does(mockless_callback);

        // Assert
        assert_eq!(result as *const Owner, &owner as *const Owner);

        fn_callback_configurator
            .fn_config
            .borrow_mut()
            .received()
            .set_callback(
                automock::Arg::is_mut(|callback: &mut Box<dyn FnMut(&'am Mock, ArgRefsTuple)>| {
                    callback.as_mut()(&Mock, arg_refs_tuple);
                    mockless_callback::received(arg_refs_tuple, automock::Times::Once)
                        .no_other_calls();
                    return true;
                }),
                automock::Times::Once,
            )
            .no_other_calls();
    }

    #[test]
    #[allow(clippy::extra_unused_lifetimes)]
    fn and_does_WithMock_Ok<'am>() {
        // Arrange
        let owner = Owner;
        let fn_callback_configurator = fn_callback_configurator::<true>(&owner);

        // Act
        let result = fn_callback_configurator.and_does(callback);

        // Assert
        assert_eq!(result as *const Owner, &owner as *const Owner);

        fn_callback_configurator
            .fn_config
            .borrow_mut()
            .received()
            .set_callback(
                automock::Arg::is_mut(
                    |callback: &mut Box<dyn FnMut(&'am MockArg, ArgRefsTuple)>| {
                        let mock_arg = MockArg(5);
                        let arg_refs_tuple = ArgRefsTuple(10);
                        callback.as_mut()(transmute_lifetime!(&mock_arg), arg_refs_tuple);
                        callback::received(
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
    fn Deref_deref_Ok() {
        // Arrange
        let owner = Owner;
        let fn_callback_configurator = fn_callback_configurator::<IRRELEVANT>(&owner);

        // Act
        let result = fn_callback_configurator.deref();

        // Assert
        assert_eq!(&owner as *const Owner, result as *const Owner);
    }

    pub mod utilities {
        use super::*;

        pub struct Owner;
        #[derive(Clone, Copy, PartialEq)]
        pub struct ArgRefsTuple(pub i32);
        #[derive(Clone, Copy, PartialEq)]
        pub struct MockArg(pub i32);

        pub fn fn_callback_configurator<const PASSES_MOCK_TO_CALLBACK: bool>(
            owner: &Owner,
        ) -> FnCallbackConfigurator<
            'static,
            Mock,
            Owner,
            ArgRefsTuple,
            MockArg,
            PASSES_MOCK_TO_CALLBACK,
        > {
            FnCallbackConfigurator {
                _phantom_args_tuple: PhantomData,
                _phantom_mock_arg: PhantomData,
                fn_config: Rc::new(RefCell::new(fn_config_mock())),
                owner: transmute_lifetime!(owner),
            }
        }

        #[automock::mock]
        pub(super) fn callback(_: &MockArg, _: ArgRefsTuple) {
            unreachable!()
        }

        #[automock::mock]
        pub(super) fn mockless_callback(_: ArgRefsTuple) {
            unreachable!()
        }
    }
}
