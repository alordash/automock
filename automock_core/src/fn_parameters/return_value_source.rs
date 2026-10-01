use crate::fn_parameters::*;
use std::fmt::{Debug, Formatter};

pub enum ReturnValueSource<'am> {
    SingleTime(DynReturnValue<'am>),
    Perpetual(Box<dyn Fn() -> DynReturnValue<'am> + 'am>),
    Factory(Box<dyn Fn(DynArgRefsTuple<'am>) -> DynReturnValue<'am> + 'am>),
}

impl<'am> Debug for ReturnValueSource<'am> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ReturnValueSource::SingleTime(_) => "SingleTime",
            ReturnValueSource::Perpetual(_) => "Perpetual",
            ReturnValueSource::Factory(_) => "Factory",
        })
    }
}
