#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use automock::*;
use std::marker::PhantomData;
use std::ops::Deref;

#[mock]
struct Struct<'a, T1: Clone> {
    pub phantom_data: PhantomData<(&'a (), T1)>,
}

#[mock(base)]
impl<'a, T1: Clone> Struct<'a, T1> {
    pub fn f<'r>(&'r self, a: &i32, t1: T1) {
        unreachable!()
    }

    pub fn ret(&self) -> &i32 {
        unreachable!()
    }
}

#[mock(base)]
trait Trait {
    fn f(v: i32) -> i32;
}

#[mock(base)]
fn f(v: i32) -> i32 {
    v + 10
}

#[test]
fn compile() {}
