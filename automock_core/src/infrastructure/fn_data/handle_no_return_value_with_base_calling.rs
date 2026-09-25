use super::*;

impl<'rs, TMock, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'rs, TMock, false, true, PASSES_MOCK_TO_CALLBACK>
{
    pub fn handle<'a, TMockArg, TCall: ICall + Clone + 'a>(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
        mut base_call: impl FnMut(TMockArg, TCall),
    ) {
        let call_for_base_call = the_call.clone();
        let call = Rc::new(DynCall::new(the_call));
        let maybe_fn_config = self.get_optional_matching_config(&call);
        self.register_call(call.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call.clone());
            let fn_config_ref = fn_config.borrow();
            if let Some(callback) = fn_config_ref.get_callback() {
                callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), call.as_ref());
            }
            if fn_config_ref.should_call_base() {
                base_call(mock_arg, call_for_base_call);
            }
        } else if self.force_call_base {
            base_call(mock_arg, call_for_base_call)
        }
    }

    pub async fn handle_async<'a, TMockArg, TCall: ICall + Clone + 'a, Fut: Future>(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
        mut base_call: impl FnMut(TMockArg, TCall) -> Fut,
    ) {
        let call_for_base_call = the_call.clone();
        let call = Rc::new(DynCall::new(the_call));
        let maybe_fn_config = self.get_optional_matching_config(&call);
        self.register_call(call.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call.clone());
            let should_call_base = {
                let fn_config_ref = fn_config.borrow();
                if let Some(callback) = fn_config_ref.get_callback() {
                    callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), call.as_ref());
                }
                fn_config_ref.should_call_base()
            };
            if should_call_base {
                base_call(mock_arg, call_for_base_call).await;
            }
        } else if self.force_call_base {
            base_call(mock_arg, call_for_base_call).await;
        }
    }
}
