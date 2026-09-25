use std::hash::Hash;

#[doc(hidden)]
#[derive(Eq, PartialEq, Hash)]
pub struct GenericsHashKey(pub(crate) u64);
