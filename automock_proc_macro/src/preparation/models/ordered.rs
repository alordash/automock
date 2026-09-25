use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

pub(crate) struct Ordered<T> {
    pub order_number: usize,
    pub value: T,
}

impl<T> Ordered<T> {
    pub fn new(order_number: usize, value: T) -> Self {
        Self {
            order_number,
            value,
        }
    }

    pub fn map<U>(self, mapping: impl FnOnce(T) -> U) -> Ordered<U> {
        Ordered {
            order_number: self.order_number,
            value: mapping(self.value),
        }
    }

    pub fn ref_map<U>(&self, mapping: impl FnOnce(&T) -> U) -> Ordered<U> {
        Ordered {
            order_number: self.order_number,
            value: mapping(&self.value),
        }
    }
}

impl<T> Deref for Ordered<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Borrow<T> for Ordered<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}

impl<T> BorrowMut<T> for Ordered<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
