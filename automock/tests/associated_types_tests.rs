#![cfg(nightly)]
#![feature(associated_type_defaults)]

use automock::*;
use std::fmt::Debug;

#[mock(base)]
trait Trait {
    const CONST: usize = 43;

    type InputType<TAmogus: Clone>: Clone + Debug
        = i32
    where
        TAmogus: Debug;

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

#[mock]
#[derive(Clone)]
struct Struct;

#[mock(base)]
impl Struct {
    #[allow(unused)]
    pub fn new() -> Self {
        Self
    }
}

#[mock(base)]
impl Trait for Struct {
    const CONST: usize = 4;
    type InputType<TAmogus: Clone>
        = [TAmogus; Self::CONST]
    where
        TAmogus: Debug;

    fn get_const(&self) -> usize {
        Self::CONST
    }

    fn get_my_type<TT: Clone>(
        &self,
        #[allow(unused)] input: <Self as Trait>::InputType<i32>,
    ) -> <Self as Trait>::OutputType<TT>
    where
        Self: Clone + Sized,
        TT: ToString,
    {
        <Self as Trait>::OutputType::<TT>::default()
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    const TEST_CONST: usize = 111;
    type TestInputType = i32;
    type TestOutputType = &'static str;

    #[test]
    fn get_const_Trait_Ok() {
        // Arrange
        let mut mock = TraitMock::<TEST_CONST, TestInputType, TestOutputType>::new();
        let const_value = TEST_CONST * 2;
        mock.setup().get_const().returns(const_value);

        // Act
        let actual_const_value = mock.get_const();

        // Assert
        assert_eq!(const_value, actual_const_value);
        mock.received().get_const(Times::Once).no_other_calls();
    }

    #[test]
    fn get_const_TraitBase_Ok() {
        // Arrange
        let mut mock = TraitMock::<TEST_CONST, TestInputType, TestOutputType>::new();
        mock.setup().get_const().call_base();

        // Act
        let actual_const_value = mock.get_const();

        // Assert
        assert_eq!(TEST_CONST, actual_const_value);
        mock.received().get_const(Times::Once).no_other_calls();
    }

    #[test]
    fn get_my_type_Trait_Ok() {
        // Arrange
        let mut mock = TraitMock::<TEST_CONST, TestInputType, TestOutputType>::new();

        type FirstTT = u128;
        let first_input: TestInputType = 10;
        let first_output: TestOutputType = "quo vadis";
        type SecondTT = f64;
        let second_input: TestInputType = 20;
        let second_output: TestOutputType = "veridis quo";
        type UnknownTT = i16;

        mock.setup()
            .get_my_type::<FirstTT>(first_input)
            .returns(first_output)
            .get_my_type::<SecondTT>(second_input)
            .returns(second_output);

        // Act
        let actual_first_output = mock.get_my_type::<FirstTT>(first_input);
        let actual_second_output = mock.get_my_type::<SecondTT>(second_input);

        // Assert
        assert_eq!(first_output, actual_first_output);
        assert_eq!(second_output, actual_second_output);

        mock.received()
            .get_my_type::<FirstTT>(first_input, Times::Once)
            .get_my_type::<UnknownTT>(first_input, Times::Never)
            .get_my_type::<SecondTT>(second_input, Times::Once)
            .get_my_type::<UnknownTT>(second_input, Times::Never)
            .no_other_calls();
    }

    #[test]
    fn get_my_type_Struct_Ok() {
        // Arrange
        let mut mock = Struct::new();

        type FirstTT = u128;
        let first_input: [i32; 4] = [10, 11, 111, 12];
        let first_output: u8 = 3;
        type SecondTT = f64;
        let second_input: [i32; 4] = [20, 2, 3, 33];
        let second_output: u8 = 67;
        type UnknownTT = i16;

        mock.setup()
            .as_Trait()
            .get_my_type::<FirstTT>(first_input)
            .returns(first_output)
            .get_my_type::<SecondTT>(second_input)
            .returns(second_output);

        // Act
        let actual_first_output = mock.get_my_type::<FirstTT>(first_input);
        let actual_second_output = mock.get_my_type::<SecondTT>(second_input);

        // Assert
        assert_eq!(first_output, actual_first_output);
        assert_eq!(second_output, actual_second_output);

        mock.received()
            .as_Trait()
            .get_my_type::<FirstTT>(first_input, Times::Once)
            .get_my_type::<UnknownTT>(first_input, Times::Never)
            .get_my_type::<SecondTT>(second_input, Times::Once)
            .get_my_type::<UnknownTT>(second_input, Times::Never);
        mock.received().no_other_calls();
    }

    #[test]
    fn get_my_type_StructBase_Ok() {
        // Arrange
        let mut mock = Struct::new();

        type FirstTT = u128;
        let first_input: [i32; 4] = [10, 11, 111, 12];
        type SecondTT = f64;
        let second_input: [i32; 4] = [20, 2, 3, 33];
        type UnknownTT = i16;

        mock.setup()
            .as_Trait()
            .get_my_type::<FirstTT>(first_input)
            .call_base()
            .get_my_type::<SecondTT>(second_input)
            .call_base();

        // Act
        let actual_first_output = mock.get_my_type::<FirstTT>(first_input);
        let actual_second_output = mock.get_my_type::<SecondTT>(second_input);

        // Assert
        let expected_output = u8::default();
        assert_eq!(expected_output, actual_first_output);
        assert_eq!(expected_output, actual_second_output);

        mock.received()
            .as_Trait()
            .get_my_type::<FirstTT>(first_input, Times::Once)
            .get_my_type::<UnknownTT>(first_input, Times::Never)
            .get_my_type::<SecondTT>(second_input, Times::Once)
            .get_my_type::<UnknownTT>(second_input, Times::Never);
        mock.received().no_other_calls();
    }
}
