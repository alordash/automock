use super::*;

impl<
    'rs,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> FnData<'rs, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
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
        maybe_base_call: MaybeBaseCall<TBaseCall>,
    ) -> TReturnValue {
        let call = DynCall::new(the_call);
        let with_return_value = true;
        let fn_config = match self.try_get_matching_config::<TReturnValue>(&call, with_return_value)
        {
            MatchingConfigSearchResult::Ok(x) => x,
            MatchingConfigSearchResult::Err(matching_config_search_err) => {
                if size_of::<TReturnValue>() == 0 {
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
        if should_call_base && let MaybeBaseCall::Use(mut base_call) = maybe_base_call {
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
        maybe_base_call: MaybeBaseCall<TBaseCall>,
    ) -> TReturnValue {
        let call = DynCall::new(the_call);
        let with_return_value = true;
        let fn_config = match self.try_get_matching_config::<TReturnValue>(&call, with_return_value)
        {
            MatchingConfigSearchResult::Ok(x) => x,
            MatchingConfigSearchResult::Err(matching_config_search_err) => {
                if size_of::<TReturnValue>() == 0 {
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
        if should_call_base && let MaybeBaseCall::Use(mut base_call) = maybe_base_call {
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

pub enum MaybeBaseCall<T> {
    Use(T),
    DoNotUse(T),
}
