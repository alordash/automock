#![allow(unused)]

use automock::*;
use std::marker::PhantomData;

#[derive(Clone)]
struct Consumer<'a> {
    phantom: PhantomData<&'a ()>,
}

#[mock(base)]
fn consume(consumer: Consumer<'_>) {}

#[mock]
fn option_ref(v: Option<&i32>) {}

#[mock]
fn arg_impl(v: impl IntoIterator<Item = i32, IntoIter = Vec<i32>>) {}

#[mock]
struct S<T> {
    t: T,
}

#[mock]
impl<T> S<T> {
    pub(crate) fn new(t: T) -> Self {
        S { t }
    }
}

struct MyI32;
impl PartialEq<MyI32> for i32 {
    fn eq(&self, _: &MyI32) -> bool {
        unreachable!()
    }
}

#[mock(base)]
impl<T: PartialEq<U>, U> PartialEq<U> for S<T> {
    fn eq(&self, _: &U) -> bool {
        unreachable!()
    }
}

#[test]
fn compile() {}
