use crate::fn_parameters::*;

pub enum ReturnValueSource<'am> {
    SingleTime(DynReturnValue<'am>),
    Perpetual(Box<dyn Fn() -> DynReturnValue<'am> + 'am>),
    Factory(Box<dyn Fn(DynArgRefsTuple<'am>) -> DynReturnValue<'am> + 'am>),
}
