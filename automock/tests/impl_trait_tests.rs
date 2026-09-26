#![allow(clippy::borrowed_box)]
use automock::*;

const DUMMY_VALUE: i32 = 124;
#[mock(base)]
trait Dummy {
    fn work(&self) -> i32 {
        DUMMY_VALUE
    }
}

impl Dummy for Box<dyn Dummy> {
    fn work(&self) -> i32 {
        self.as_ref().work()
    }
}

struct DummyImpl;
impl Dummy for DummyImpl {}

#[mock]
fn input(dummy: impl Dummy) -> i32 {
    dummy.work()
}

#[mock(base)]
fn output() -> impl Dummy {
    return DummyImpl;
}

#[mock]
trait Trait {
    fn input(&self, _: impl Dummy) -> i32 {
        unreachable!()
    }

    fn static_input(_: impl Dummy) -> i32 {
        unreachable!()
    }
}

#[mock(base)]
trait ReturnTrait {
    fn output(&self) -> impl Dummy {
        DummyImpl
    }

    fn static_output() -> impl Dummy {
        DummyImpl
    }
}

#[mock]
struct Struct;
#[mock(base)]
impl Struct {
    pub fn new() -> Self {
        Self
    }
}

#[mock]
impl Struct {
    pub fn input_self(&self, dummy: impl Dummy) -> i32 {
        unreachable!()
    }

    pub fn static_input_self(dummy: impl Dummy) -> i32 {
        unreachable!()
    }
}

#[mock(base)]
impl Struct {
    pub fn output_self(&self) -> impl Dummy {
        DummyImpl
    }

    pub fn static_output_self() -> impl Dummy {
        DummyImpl
    }
}

#[mock]
impl Trait for Struct {
    fn input(&self, dummy: impl Dummy) -> i32 {
        unreachable!()
    }

    fn static_input(dummy: impl Dummy) -> i32 {
        unreachable!()
    }
}

#[mock(base)]
impl ReturnTrait for Struct {
    fn output(&self) -> impl Dummy {
        DummyImpl
    }

    fn static_output() -> impl Dummy {
        DummyImpl
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    const ACTUAL_VALUE: i32 = 455;

    mod fn_tests {
        use super::*;

        #[test]
        fn input_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            input::setup(Arg::Any).returns(ACTUAL_VALUE);

            // Act
            let result = input(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            input::received(Arg::Any, Times::Once).no_other_calls();
        }

        #[test]
        fn input_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().always_returns(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().always_returns(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            input::setup(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .setup(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = input(dummy1);
            let result2 = input(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            input::received(
                Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                Times::Once,
            )
            .received(
                Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                Times::Once,
            )
            .no_other_calls();
        }

        #[test]
        fn output_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().returns(ACTUAL_VALUE);

            output::setup().returns(Box::new(dummy));

            // Act
            let result = output();
            let result_value = result.work();

            // Assert
            assert_eq!(ACTUAL_VALUE, result_value);
            output::received(Times::Once).no_other_calls();
        }

        #[test]
        fn output_Base_Ok() {
            // Arrange
            output::setup().call_base();

            // Act
            let result = output();
            let result_value = result.work();

            // Assert
            assert_eq!(DUMMY_VALUE, result_value);
            output::received_nothing();
        }
    }

    mod Trait_tests {
        use super::*;

        #[test]
        fn Trait_input_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            let mut mock = TraitMock::new();
            mock.setup().input(Arg::Any).returns(ACTUAL_VALUE);

            // Act
            let result = mock.input(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            mock.received()
                .input(Arg::Any, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn Trait_input_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().always_returns(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().always_returns(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            let mut mock = TraitMock::new();
            mock.setup()
                .input(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .input(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = mock.input(dummy1);
            let result2 = mock.input(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            mock.received()
                .input(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .input(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn Trait_static_input_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            TraitMock::static_setup()
                .static_input(Arg::Any)
                .returns(ACTUAL_VALUE);

            // Act
            let result = TraitMock::static_input(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            TraitMock::static_received()
                .static_input(Arg::Any, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn Trait_static_input_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().always_returns(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().always_returns(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            TraitMock::static_setup()
                .static_input(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .static_input(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = TraitMock::static_input(dummy1);
            let result2 = TraitMock::static_input(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            TraitMock::static_received()
                .static_input(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .static_input(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn ReturnTrait_output_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().returns(ACTUAL_VALUE);

            let mut mock = ReturnTraitMock::new();
            mock.setup().output().returns(Box::new(dummy));

            // Act
            let result = mock.output();
            let result_value = result.work();

            // Assert
            assert_eq!(ACTUAL_VALUE, result_value);
            mock.received().output(Times::Once).no_other_calls();
        }

        #[test]
        fn ReturnTrait_output_Base_Ok() {
            // Arrange
            let mut mock = ReturnTraitMock::new();
            mock.setup().output().call_base();

            // Act
            let result = mock.output();
            let result_value = result.work();

            // Assert
            assert_eq!(DUMMY_VALUE, result_value);
            mock.received().no_other_calls();
        }

        #[test]
        fn ReturnTrait_static_output_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().returns(ACTUAL_VALUE);

            ReturnTraitMock::static_setup()
                .static_output()
                .returns(Box::new(dummy));

            // Act
            let result = ReturnTraitMock::static_output();
            let result_value = result.work();

            // Assert
            assert_eq!(ACTUAL_VALUE, result_value);
            ReturnTraitMock::static_received()
                .static_output(Times::Once)
                .no_other_calls();
        }

        #[test]
        fn ReturnTrait_static_output_Base_Ok() {
            // Arrange
            ReturnTraitMock::static_setup().static_output().call_base();

            // Act
            let result = ReturnTraitMock::static_output();
            let result_value = result.work();

            // Assert
            assert_eq!(DUMMY_VALUE, result_value);
            ReturnTraitMock::static_received().no_other_calls();
        }
    }

    mod Struct_tests {
        use super::*;

        #[test]
        fn Struct_input_self_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup().input_self(Arg::Any).returns(ACTUAL_VALUE);

            // Act
            let result = mock.input_self(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            mock.received()
                .input_self(Arg::Any, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn Struct_input_self_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().always_returns(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().always_returns(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup()
                .input_self(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .input_self(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = mock.input_self(dummy1);
            let result2 = mock.input_self(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            mock.received()
                .input_self(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .input_self(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn Struct_static_input_self_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            Struct::static_setup()
                .static_input_self(Arg::Any)
                .returns(ACTUAL_VALUE);

            // Act
            let result = Struct::static_input_self(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            Struct::static_received()
                .static_input_self(Arg::Any, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn Struct_static_input_self_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().always_returns(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().always_returns(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            Struct::static_setup()
                .static_input_self(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .static_input_self(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = Struct::static_input_self(dummy1);
            let result2 = Struct::static_input_self(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            Struct::static_received()
                .static_input_self(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .static_input_self(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn Struct_output_self_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().returns(ACTUAL_VALUE);

            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup().output_self().returns(Box::new(dummy));

            // Act
            let result = mock.output_self();
            let result_value = result.work();

            // Assert
            assert_eq!(ACTUAL_VALUE, result_value);
            mock.received().output_self(Times::Once).no_other_calls();
        }

        #[test]
        fn Struct_output_self_Base_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup().output_self().call_base();

            // Act
            let result = mock.output_self();
            let result_value = result.work();

            // Assert
            assert_eq!(DUMMY_VALUE, result_value);
            mock.received().no_other_calls();
        }

        #[test]
        fn Struct_static_output_self_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().returns(ACTUAL_VALUE);

            Struct::static_setup()
                .static_output_self()
                .returns(Box::new(dummy));

            // Act
            let result = Struct::static_output_self();
            let result_value = result.work();

            // Assert
            assert_eq!(ACTUAL_VALUE, result_value);
            Struct::static_received()
                .static_output_self(Times::Once)
                .no_other_calls();
        }

        #[test]
        fn Struct_static_output_self_Base_Ok() {
            // Arrange
            Struct::static_setup().static_output_self().call_base();

            // Act
            let result = Struct::static_output_self();
            let result_value = result.work();

            // Assert
            assert_eq!(DUMMY_VALUE, result_value);
            Struct::static_received().no_other_calls();
        }

        #[test]
        fn Struct_as_trait_input_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup()
                .as_Trait()
                .input(Arg::Any)
                .returns(ACTUAL_VALUE);

            // Act
            let result = mock.input(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            mock.received()
                .as_Trait()
                .input(Arg::Any, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn Struct_as_trait_input_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().always_returns(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().always_returns(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup()
                .as_Trait()
                .input(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .input(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = mock.input(dummy1);
            let result2 = mock.input(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            mock.received()
                .as_Trait()
                .input(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .input(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn Struct_static_as_trait_input_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            Struct::static_setup()
                .as_Trait()
                .static_input(Arg::Any)
                .returns(ACTUAL_VALUE);

            // Act
            let result = Struct::static_input(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            Struct::static_received()
                .as_Trait()
                .static_input(Arg::Any, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn Struct_static_as_trait_input_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().always_returns(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().always_returns(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            Struct::static_setup()
                .as_Trait()
                .static_input(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .static_input(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = Struct::static_input(dummy1);
            let result2 = Struct::static_input(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            Struct::static_received()
                .as_Trait()
                .static_input(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .static_input(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn Struct_output_as_trait_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().returns(ACTUAL_VALUE);

            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup()
                .as_ReturnTrait()
                .output()
                .returns(Box::new(dummy));

            // Act
            let result = mock.output();
            let result_value = result.work();

            // Assert
            assert_eq!(ACTUAL_VALUE, result_value);
            mock.received()
                .as_ReturnTrait()
                .output(Times::Once)
                .no_other_calls();
        }

        #[test]
        fn Struct_output_as_trait_Base_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup().as_ReturnTrait().output().call_base();

            // Act
            let result = mock.output();
            let result_value = result.work();

            // Assert
            assert_eq!(DUMMY_VALUE, result_value);
            mock.received().as_ReturnTrait().no_other_calls();
        }

        #[test]
        fn Struct_static_output_as_trait_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().returns(ACTUAL_VALUE);

            Struct::static_setup()
                .as_ReturnTrait()
                .static_output()
                .returns(Box::new(dummy));

            // Act
            let result = Struct::static_output();
            let result_value = result.work();

            // Assert
            assert_eq!(ACTUAL_VALUE, result_value);
            Struct::static_received()
                .as_ReturnTrait()
                .static_output(Times::Once)
                .no_other_calls();
        }

        #[test]
        fn Struct_static_output_as_trait_Base_Ok() {
            // Arrange
            Struct::static_setup()
                .as_ReturnTrait()
                .static_output()
                .call_base();

            // Act
            let result = Struct::static_output();
            let result_value = result.work();

            // Assert
            assert_eq!(DUMMY_VALUE, result_value);
            Struct::static_received().as_ReturnTrait().no_other_calls();
        }
    }
}
