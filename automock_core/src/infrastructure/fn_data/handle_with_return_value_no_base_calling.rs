use super::*;

impl<'rs, TMock, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'rs, TMock, true, false, PASSES_MOCK_TO_CALLBACK>
{
    pub fn handle<'a, 'b, TMockArg, TCall: ICall + 'a, TReturnValue: IReturnValue<'b>>(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
    ) -> TReturnValue {
        let call = Rc::new(DynCall::new(the_call));
        let fn_config = self.get_required_matching_config(&call);
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        if let Some(callback) = fn_config.borrow().get_callback() {
            callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), call.as_ref());
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
        let call = Rc::new(DynCall::new(the_call));
        let fn_config = self.get_required_matching_config(&call);
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        if let Some(callback) = fn_config.borrow().get_callback() {
            callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), call.as_ref());
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
