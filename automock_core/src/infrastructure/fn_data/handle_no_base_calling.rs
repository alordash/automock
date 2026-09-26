use super::*;
use crate::infrastructure::fn_data::handle_with_base_calling::MaybeBaseCall;

impl<'rs, TMock, const HAS_RETURN_VALUE: bool, const PASSES_MOCK_TO_CALLBACK: bool>
    FnData<'rs, TMock, HAS_RETURN_VALUE, false, PASSES_MOCK_TO_CALLBACK>
{
    pub fn handle<'a, 'b, TMockArg, TCall: ICall + 'a, TReturnValue: IReturnValue<'b>>(
        &self,
        mock_arg: TMockArg,
        the_call: TCall,
    ) -> TReturnValue {
        self.handle_base(
            mock_arg,
            the_call,
            MaybeBaseCall::DoNotUse(|_, _| unreachable!()),
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
        self.handle_base_async(
            mock_arg,
            the_call,
            MaybeBaseCall::DoNotUse(async |_, _| unreachable!()),
        )
        .await
    }
}
