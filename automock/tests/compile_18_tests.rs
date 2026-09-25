#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use automock::*;
use std::ops::Deref;

#[mock]
#[derive(Clone)]
pub struct Struct<S1: Clone> {
    pub s1: S1,
}

#[mock]
impl<S1: Clone> Struct<S1> {
    pub fn new(s1: S1) -> Self {
        Self { s1 }
    }
}

#[mock(base)]
impl<S1: Clone> Struct<S1> {
    pub fn f(&self, v: i32) -> f32
    where
        S1: Clone + Into<i32>,
    {
        (v + self.s1.clone().into()) as f32
    }

    pub fn f_static(v: i32) -> f32 {
        23f32
    }

    pub fn transform(s: Self) -> Self {
        s
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
