#![cfg(nightly)]
#![feature(associated_type_defaults)]
use automock::*;

#[mock]
trait Trait<T, const N: usize> {
    type Item = T;
    #[allow(unused)]
    const M: usize = N;
    #[allow(unused)]
    const CHAR: char = 'm';
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn MockCreation_RequiresOnlyFirstTwoGenericArgument() {
        // Arrange
        // Act
        // Assert
        let _ = TraitMock::<i32, 3>::new();
    }
}
