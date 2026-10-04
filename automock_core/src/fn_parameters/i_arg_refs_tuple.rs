#[doc(hidden)]
pub trait IArgRefsTuple<'am> {}

impl<'am, T: 'am> IArgRefsTuple<'am> for T {}
