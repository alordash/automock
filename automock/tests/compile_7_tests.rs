#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use automock::*;
use std::cell::{LazyCell, RefCell};
use std::fmt::Debug;
use std::sync::Arc;

#[mock(base)]
fn global(number: i32) -> String {
    return format!("actual number: {number}");
}

mod tests {
    use super::*;
    use crate::global;

    #[test]
    pub fn global_test() {
        // Arrange
        global::setup(Arg::eq(2))
            .call_base()
            .setup(Arg::eq(143))
            .returns("MOCK: 143".to_string());

        // Act
        let result1 = global(2);
        let result2_1 = global(143);

        // Assert
        let expected_v = 2;
        global::received(Arg::is(|v| *v == expected_v), Times::Once);
        global::received(Arg::eq(2), Times::Once).received(Arg::eq(143), Times::Exactly(1));
        assert_eq!("actual number: 2", result1);
        assert_eq!("MOCK: 143", result2_1);
    }

    #[test]
    pub fn global_test2() {
        // Arrange
        global::setup(Arg::eq(11))
            .call_base()
            .setup(Arg::eq(33))
            .returns("MOCK: 33".to_string());

        // Act
        let result1 = global(11);
        let result2_1 = global(33);

        // Assert
        global::received(Arg::eq(11), Times::Once).received(Arg::eq(33), Times::Exactly(1));
        assert_eq!("actual number: 11", result1);
        assert_eq!("MOCK: 33", result2_1);
    }
}
