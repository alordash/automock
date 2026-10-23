use crate::fn_parameters::*;
use crate::infrastructure::*;
use std::cell::RefCell;
use std::rc::Rc;

type SharedFnConfig<'am, TMock> = Rc<RefCell<FnConfig<'am, TMock>>>;

impl<
    'am,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
    const PASSES_MOCK_TO_CALLBACK: bool,
> FnData<'am, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
{
    fn handle_core_prepare<'a, TMockArg, TCall: ICall, TReturnValue: IReturnValue<'a>>(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
    ) -> DataForHandling<'am, TMock, TMockArg> {
        let call = DynCall::new(the_call);
        let fn_config = match self.try_get_matching_config::<TReturnValue>(&call) {
            MatchingConfigSearchResult::Ok(x) => x,
            MatchingConfigSearchResult::Err(matching_config_search_err) => {
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
        return DataForHandling {
            mock_arg,
            call,
            fn_config,
            should_call_base,
        };
    }

    fn handle_core_continue<TReturnValue>(
        &self,
        call: DynCall<'am>,
        fn_config: SharedFnConfig<'am, TMock>,
    ) -> TReturnValue {
        let rc_call = Rc::new(call);
        self.register_call(rc_call.clone());
        fn_config.borrow_mut().register_call(rc_call.clone());
        let return_value = match fn_config.borrow_mut().select_next_return_value(&rc_call) {
            Some(v) => v,
            None => error_printing::panic_return_values_exhausted(
                &self.formatted_fn_name,
                rc_call.get_arg_infos(),
                rc_call.get_generic_parameter_infos(),
            ),
        };
        return return_value.downcast_into();
    }

    fn handle_core<
        'a,
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
        let data_for_handling = self.handle_core_prepare::<_, _, TReturnValue>(mock_arg, the_call);
        if data_for_handling.should_call_base
            && let MaybeBaseCall::Some(mut base_call) = base_call_policy
        {
            let call_for_base_call = data_for_handling.call.downcast_into();
            let base_return_value = base_call(data_for_handling.mock_arg, call_for_base_call);
            return base_return_value;
        }
        return self.handle_core_continue(data_for_handling.call, data_for_handling.fn_config);
    }

    async fn handle_core_async<
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
        base_call_policy: MaybeBaseCall<TBaseCall>,
    ) -> TReturnValue {
        let data_for_handling = self.handle_core_prepare::<_, _, TReturnValue>(mock_arg, the_call);
        if data_for_handling.should_call_base
            && let MaybeBaseCall::Some(mut base_call) = base_call_policy
        {
            let call_for_base_call = data_for_handling.call.downcast_into();
            let base_return_value = base_call(data_for_handling.mock_arg, call_for_base_call).await;
            return base_return_value;
        }
        return self.handle_core_continue(data_for_handling.call, data_for_handling.fn_config);
    }
}

impl<'am, TMock, const HAS_RETURN_VALUE: bool, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'am, TMock, HAS_RETURN_VALUE, true, PASSES_MOCK_TO_CALLBACK>
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
        self.handle_core(mock_arg, the_call, MaybeBaseCall::Some(base_call))
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
        self.handle_core_async(mock_arg, the_call, MaybeBaseCall::Some(base_call))
            .await
    }
}

impl<'am, TMock, const HAS_RETURN_VALUE: bool, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'am, TMock, HAS_RETURN_VALUE, false, PASSES_MOCK_TO_CALLBACK>
{
    pub fn handle<'a, 'b, TMockArg, TCall: ICall + 'a, TReturnValue: IReturnValue<'b>>(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
    ) -> TReturnValue {
        self.handle_core(
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
        self.handle_core_async(
            mock_arg,
            the_call,
            MaybeBaseCall::None(async |_, _| unreachable!()),
        )
        .await
    }
}

struct DataForHandling<'am, TMock, TMockArg> {
    mock_arg: TMockArg,
    call: DynCall<'am>,
    fn_config: SharedFnConfig<'am, TMock>,
    should_call_base: bool,
}

enum MaybeBaseCall<T> {
    Some(T),
    None(T), // Need to store because type inference does not work
}
