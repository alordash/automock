use super::*;

impl<'rs, TMock, const HAS_RETURN_VALUE: bool, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'rs, TMock, HAS_RETURN_VALUE, false, PASSES_MOCK_TO_CALLBACK>
{
    pub fn handle<'a, 'b, TMockArg, TCall: ICall + 'a, TReturnValue: IReturnValue<'b>>(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
    ) -> TReturnValue {
        let call = Rc::new(DynCall::new(the_call));
        let with_return_value = true;
        let fn_config = match self.try_get_matching_config::<TReturnValue>(&call, with_return_value) {
            MatchingConfigSearchResult::Ok(x) => x,
            MatchingConfigSearchResult::Err(matching_config_search_err) => {
                if size_of::<TReturnValue>() == 0 {
                    self.register_call(call.clone());
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
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        if let Some(callback) = fn_config.borrow().get_callback() {
            callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), call.as_ref());
        }
        let return_value = match fn_config.borrow_mut().select_next_return_value(&call) {
            Some(v) => v,
            None if size_of::<TReturnValue>() == 0 => return unsafe { core::mem::zeroed() },
            None => error_printing::panic_no_return_value_was_configured(
                &self.formatted_fn_name,
                call.get_arg_infos(),
                call.get_generic_parameter_infos(),
            ),
        };
        return return_value.downcast_into();
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
        self.handle(mock_arg, the_call)
    }
}
