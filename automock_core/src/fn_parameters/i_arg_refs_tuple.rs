pub(crate) trait IArgRefsTuple<'rs> {}

impl<'rs, T: 'rs> IArgRefsTuple<'rs> for T {}
