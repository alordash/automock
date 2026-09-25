pub trait IReturnValue<'rs> {}

impl<'rs, T: 'rs> IReturnValue<'rs> for T {}
