use std::hash::Hash;

#[doc(hidden)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct GenericsHashKey(pub(crate) u64);
