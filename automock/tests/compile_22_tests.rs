#![cfg(nightly)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]
#![feature(associated_type_defaults)]

use automock::*;
use std::fmt::Debug;

#[mock(base)]
trait Trait {
    const CONST: usize = 43;

    type InputType<TAmogus: Clone>: Clone + Debug = i32;

    type OutputType<TT>: Clone + Sized + Default
        = u8
    where
        TT: Clone;

    fn get_const(&self) -> usize {
        Self::CONST
    }

    fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::OutputType<TT>
    where
        TT: ToString;
}

#[test]
fn compile() {}
