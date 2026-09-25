use super::*;

impl<'rs, TMock, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'rs, TMock, true, true, PASSES_MOCK_TO_CALLBACK>
{
    pub fn handle<
        'a,
        TMockArg,
        TCall: ICall + Clone,
        TReturnValue: IReturnValue<'a>,
        TBaseCall: FnMut(TMockArg, TCall) -> TReturnValue,
    >(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
        mut base_call: TBaseCall,
    ) -> TReturnValue {
        let call_for_base_call = the_call.clone();
        let call = Rc::new(DynCall::new(the_call));
        let with_return_value = true;
        let fn_config = match self.try_get_matching_config(&call, with_return_value) {
            MatchingConfigSearchResult::Ok(x) => x,
            MatchingConfigSearchResult::Err(matching_config_search_err) => {
                if self.force_call_base {
                    let base_return_value = base_call(mock_arg, call_for_base_call);
                    return base_return_value;
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
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        let fn_config_ref = fn_config.borrow();
        if let Some(callback) = fn_config_ref.get_callback() {
            callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), call.as_ref());
        }
        if fn_config_ref.should_call_base() {
            let base_return_value = base_call(mock_arg, call_for_base_call);
            return base_return_value;
        }
        drop(fn_config_ref);
        let Some(return_value) = fn_config.borrow_mut().select_next_return_value(&call) else {
            error_printing::panic_no_return_value_was_configured(
                &self.formatted_fn_name,
                call.get_arg_infos(),
                call.get_generic_parameter_infos(),
            );
        };
        return return_value.downcast_into();
    }
    pub async fn handle_async<
        'a,
        TMockArg,
        TCall: ICall + Clone,
        TReturnValue: IReturnValue<'a>,
        TBaseCall: FnMut(TMockArg, TCall) -> Fut,
        Fut: Future<Output = TReturnValue>,
    >(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
        mut base_call: TBaseCall,
    ) -> TReturnValue {
        let call_for_base_call = the_call.clone();
        let dyn_call = DynCall::new(the_call);
        let call = Rc::new(dyn_call);
        let with_return_value = true;
        let fn_config = match self.try_get_matching_config(&call, with_return_value) {
            MatchingConfigSearchResult::Ok(x) => x,
            MatchingConfigSearchResult::Err(matching_config_search_err) => {
                if self.force_call_base {
                    let base_return_value = base_call(mock_arg, call_for_base_call);
                    return base_return_value.await;
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
        self.register_call(call.clone());
        let should_call_base = {
            fn_config.borrow_mut().register_call(call.clone());
            let fn_config_ref = fn_config.borrow();
            if let Some(callback) = fn_config_ref.get_callback() {
                callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), call.as_ref());
            }
            fn_config_ref.should_call_base()
        };
        if should_call_base {
            let base_return_value = base_call(mock_arg, call_for_base_call);
            return base_return_value.await;
        }
        let Some(return_value) = fn_config.borrow_mut().select_next_return_value(&call) else {
            error_printing::panic_no_return_value_was_configured(
                &self.formatted_fn_name,
                call.get_arg_infos(),
                call.get_generic_parameter_infos(),
            );
        };
        return return_value.downcast_into();
    }
}
