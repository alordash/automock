use std::marker::PhantomData;
use std::ops::Deref;

pub struct ArgRefsBinder<TOwner, TArgRefsTuple> {
    arg_refs_tuple: PhantomData<TArgRefsTuple>,
    owner: TOwner,
}

impl<TOwner, TArgRefsTuple> ArgRefsBinder<TOwner, TArgRefsTuple> {
    pub fn new(owner: TOwner) -> Self {
        Self {
            arg_refs_tuple: PhantomData,
            owner,
        }
    }
}

impl<TOwner, TArgRefsTuple> Deref for ArgRefsBinder<TOwner, TArgRefsTuple> {
    type Target = TOwner;

    fn deref(&self) -> &Self::Target {
        &self.owner
    }
}
