use super::*;

impl<'rs, TMock, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'rs, TMock, false, false, PASSES_MOCK_TO_CALLBACK>
{
    pub fn handle<'a, TMockArg, TCall: ICall + 'a>(&self, mock_arg: TMockArg, the_call: TCall) {
        let call = Rc::new(DynCall::new(the_call));
        let maybe_fn_config = self.get_optional_matching_config(&call);
        self.register_call(call.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call.clone());
            if let Some(callback) = fn_config.borrow().get_callback() {
                callback.borrow_mut()(&mock_arg as *const TMockArg as *const (), call.as_ref());
            }
        }
    }

    pub async fn handle_async<'a, TMockArg, TCall: ICall + 'a>(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
    ) {
        self.handle(mock_arg, the_call)
    }
}
