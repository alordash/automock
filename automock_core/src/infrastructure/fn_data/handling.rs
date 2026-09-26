use crate::args::*;
use crate::fn_parameters::*;
use crate::infrastructure::*;
use std::rc::Rc;

impl<
    'rs,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> FnData<'rs, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
{
    fn handle_core<
        'a,
        TActualReturnValue: IReturnValue<'a>,
        TMockArg,
        TCall: ICall,
        TReturnValue: IReturnValue<'a>,
        TBaseCall: FnMut(TMockArg, TCall) -> TReturnValue,
    >(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
        base_call_policy: MaybeBaseCall<TBaseCall>,
    ) -> TReturnValue {
        let call = DynCall::new(the_call);
        let with_return_value = true;
        let fn_config =
            match self.try_get_matching_config::<TActualReturnValue>(&call, with_return_value) {
                MatchingConfigSearchResult::Ok(x) => x,
                MatchingConfigSearchResult::Err(matching_config_search_err) => {
                    if size_of::<TActualReturnValue>() == 0 {
                        let rc_call = Rc::new(call);
                        self.register_call(rc_call.clone());
                        return unsafe { core::mem::zeroed() };
                    }
                    error_printing::panic_no_suitable_fn_configuration_found(
                        self.fn_name,
                        &self.formatted_fn_name,
                        call.get_arg_infos(),
                        call.get_generic_parameter_infos(),
                        matching_config_search_err,
                    )
                }
            };
        let should_call_base = {
            let fn_config_ref = fn_config.borrow();
            if let Some(callback) = fn_config_ref.get_callback() {
                callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), &call);
            }
            fn_config_ref.should_call_base()
        };
        if should_call_base && let MaybeBaseCall::Some(mut base_call) = base_call_policy {
            let call_for_base_call = call.downcast_into();
            let base_return_value = base_call(mock_arg, call_for_base_call);
            return base_return_value;
        }
        let rc_call = Rc::new(call);
        self.register_call(rc_call.clone());
        fn_config.borrow_mut().register_call(rc_call.clone());
        let return_value = match fn_config.borrow_mut().select_next_return_value(&rc_call) {
            Some(v) => v,
            None if size_of::<TReturnValue>() == 0 => return unsafe { core::mem::zeroed() },
            None => error_printing::panic_no_return_value_was_configured(
                &self.formatted_fn_name,
                rc_call.get_arg_infos(),
                rc_call.get_generic_parameter_infos(),
            ),
        };
        return return_value.downcast_into();
    }

    async fn handle_core_async<
        'a,
        TActualReturnValue: IReturnValue<'a>,
        TMockArg,
        TCall: ICall,
        TReturnValue: IReturnValue<'a>,
        TBaseCall: FnMut(TMockArg, TCall) -> Fut,
        Fut: Future<Output = TReturnValue>,
    >(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
        base_call_policy: MaybeBaseCall<TBaseCall>,
    ) -> TReturnValue {
        // return self.handle_core::<TActualReturnValue, _, _, _, _>(mock_arg, the_call, base_call_policy).await;
        let call = DynCall::new(the_call);
        let with_return_value = true;
        let fn_config =
            match self.try_get_matching_config::<TActualReturnValue>(&call, with_return_value) {
                MatchingConfigSearchResult::Ok(x) => x,
                MatchingConfigSearchResult::Err(matching_config_search_err) => {
                    if size_of::<TActualReturnValue>() == 0 {
                        let rc_call = Rc::new(call);
                        self.register_call(rc_call.clone());
                        return unsafe { core::mem::zeroed() };
                    }
                    error_printing::panic_no_suitable_fn_configuration_found(
                        self.fn_name,
                        &self.formatted_fn_name,
                        call.get_arg_infos(),
                        call.get_generic_parameter_infos(),
                        matching_config_search_err,
                    )
                }
            };
        let should_call_base = {
            let fn_config_ref = fn_config.borrow();
            if let Some(callback) = fn_config_ref.get_callback() {
                callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), &call);
            }
            fn_config_ref.should_call_base()
        };
        if should_call_base && let MaybeBaseCall::Some(mut base_call) = base_call_policy {
            let call_for_base_call = call.downcast_into();
            let base_return_value = base_call(mock_arg, call_for_base_call).await;
            return base_return_value;
        }
        let rc_call = Rc::new(call);
        self.register_call(rc_call.clone());
        fn_config.borrow_mut().register_call(rc_call.clone());
        let return_value = match fn_config.borrow_mut().select_next_return_value(&rc_call) {
            Some(v) => v,
            None if size_of::<TReturnValue>() == 0 => return unsafe { core::mem::zeroed() },
            None => error_printing::panic_no_return_value_was_configured(
                &self.formatted_fn_name,
                rc_call.get_arg_infos(),
                rc_call.get_generic_parameter_infos(),
            ),
        };
        return return_value.downcast_into();
    }
}

impl<'rs, TMock, const HAS_RETURN_VALUE: bool, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'rs, TMock, HAS_RETURN_VALUE, true, PASSES_MOCK_TO_CALLBACK>
{
    pub fn handle_base<
        'a,
        TMockArg,
        TCall: ICall,
        TReturnValue: IReturnValue<'a>,
        TBaseCall: FnMut(TMockArg, TCall) -> TReturnValue,
    >(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
        base_call: TBaseCall,
    ) -> TReturnValue {
        self.handle_core::<TReturnValue, _, _, _, _>(
            mock_arg,
            the_call,
            MaybeBaseCall::Some(base_call),
        )
    }

    pub async fn handle_base_async<
        'a,
        TMockArg,
        TCall: ICall,
        TReturnValue: IReturnValue<'a>,
        TBaseCall: FnMut(TMockArg, TCall) -> Fut,
        Fut: Future<Output = TReturnValue>,
    >(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
        base_call: TBaseCall,
    ) -> TReturnValue {
        self.handle_core_async::<TReturnValue, _, _, _, _, _>(
            mock_arg,
            the_call,
            MaybeBaseCall::Some(base_call),
        )
        .await
    }
}

impl<'rs, TMock, const HAS_RETURN_VALUE: bool, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'rs, TMock, HAS_RETURN_VALUE, false, PASSES_MOCK_TO_CALLBACK>
{
    pub fn handle<'a, 'b, TMockArg, TCall: ICall + 'a, TReturnValue: IReturnValue<'b>>(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
    ) -> TReturnValue {
        self.handle_core::<TReturnValue, _, _, _, _>(
            mock_arg,
            the_call,
            MaybeBaseCall::None(|_, _| unreachable!()),
        )
    }

    pub async fn handle_async<
        'a,
        'b,
        TMockArg,
        TCall: ICall + 'a,
        TReturnValue: IReturnValue<'b>,
    >(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
    ) -> TReturnValue {
        self.handle_core_async::<TReturnValue, _, _, _, _, _>(
            mock_arg,
            the_call,
            MaybeBaseCall::None(async |_, _| unreachable!()),
        )
        .await
    }
}

enum MaybeBaseCall<T> {
    Some(T),
    None(T), // Need to store because type inference does not work
}
