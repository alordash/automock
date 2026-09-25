#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use automock::*;

#[mock(base)]
fn f<'a, T: Clone>(v: &'a T) -> i32 {
    121
}

#[mock(base)]
fn flex<'a, 'b>(v: &'a &'b &()) {}

struct Q;
impl Q {
    fn kavo(&self) {}
    fn chevo(&self) {
        self.kavo();
    }
}

struct QMock;
impl QMock {
    fn __base_chevo(__rsa_self: &Q) {
        __rsa_self.kavo()
    }
}

trait Trait<T> {
    type MyType;

    fn flex(&self) -> Self::MyType;
}

impl<T> Trait<T> for Q {
    type MyType = i32;

    fn flex(&self) -> Self::MyType {
        unreachable!()
    }
}

impl Q {
    fn trait_usage<T>(&self) -> <Self as Trait<T>>::MyType {
        <Q as Trait<T>>::flex(self)
    }
}

trait F {
    fn ff(&self) {}
}
impl F for Q {}
impl Q {
    fn use_f(&self) {
        Q::ff(self);
    }
}

#[test]
fn compile() {}
