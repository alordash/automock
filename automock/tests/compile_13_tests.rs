#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(noop_method_call)]
#![allow(suspicious_double_ref_op)]

use automock::*;
use std::cell::{LazyCell, RefCell};
use std::fmt::Debug;
use std::sync::Arc;

pub trait IFoo: Debug {
    fn get_value(&self) -> i32;
}

#[derive(Debug)]
struct Foo(i32);

impl IFoo for Foo {
    fn get_value(&self) -> i32 {
        self.0
    }
}

const DEFAULT_MY_TRAIT_GET: i32 = 10;
#[mock(base)]
trait MyTrait {
    fn work(&self, value: i32);

    fn another_work(
        &self,
        string: &str,
        something: &&[u8],
        dyn_obj: &dyn IFoo,
        arc: Arc<dyn IFoo>,
    ) -> Vec<u8>;

    fn get(&self) -> i32 {
        let value = DEFAULT_MY_TRAIT_GET;
        self.work(value);
        return value;
    }
}

use not_enough_asserts::*;

#[test]
fn received_nothing_else_Ok() {
    // Arrange
    let mut mock = MyTraitMock::new();
    let returned_value = 11;
    mock.setup().get().returns(returned_value);

    // Act
    let actual_returned_value = mock.get();

    // Assert
    assert_eq!(returned_value, actual_returned_value);

    mock.received().get(Times::Once).no_other_calls();
}

#[test]
fn get_CallBase_Ok() {
    // Arrange
    let mut mock = MyTraitMock::new();
    mock.setup().get().call_base();

    // Act
    let actual_value = mock.get();

    // Assert
    assert_eq!(DEFAULT_MY_TRAIT_GET, actual_value);
    mock.received()
        .get(Times::Once)
        .work(actual_value, Times::Once)
        .no_other_calls();
}
