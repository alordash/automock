use crate::fn_parameters::*;

pub enum ReturnValueSource<'rs> {
    SingleTime(DynReturnValue<'rs>),
    Perpetual(Box<dyn Fn() -> DynReturnValue<'rs> + 'rs>),
    Factory(Box<dyn Fn(DynArgRefsTuple<'rs>) -> DynReturnValue<'rs> + 'rs>),
}
