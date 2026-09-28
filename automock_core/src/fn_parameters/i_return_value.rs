pub trait IReturnValue<'am> {}

impl<'am, T: 'am> IReturnValue<'am> for T {}
