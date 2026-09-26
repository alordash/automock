use automock::*;

#[derive(Debug, PartialEq)]
struct Foo;
#[derive(Debug, PartialEq)]
struct Bar;

trait IPayload {
    fn name(&self) -> &'static str;
}
impl IPayload for Foo {
    fn name(&self) -> &'static str {
        "Foo"
    }
}
impl IPayload for Bar {
    fn name(&self) -> &'static str {
        "Bar"
    }
}
impl IPayload for Box<dyn IPayload> {
    fn name(&self) -> &'static str {
        self.as_ref().name()
    }
}

#[mock(base)]
trait Trait {
    fn work(&self) -> impl IPayload {
        Foo
    }

    fn static_work() -> impl IPayload {
        Foo
    }
}

#[mock]
struct Struct;

#[mock(base)]
impl Struct {
    fn new() -> Self {
        Self
    }

    fn work(&self) -> impl IPayload {
        Foo
    }

    fn static_work() -> impl IPayload {
        Foo
    }
}

#[mock(base)]
impl Trait for Struct {
    fn work(&self) -> impl IPayload {
        Foo
    }

    fn static_work() -> impl IPayload {
        Foo
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    mod r#trait {
        use super::*;

        #[test]
        fn Trait_work_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            mock.setup()
                .work()
                .returns(Box::new(Bar))
                .work()
                .returns(Box::new(Foo));

            // Act
            let result = mock.work();

            // Assert
            assert_eq!(result.name(), "Bar");
            mock.received().work(1.time()).no_other_calls();
        }

        #[test]
        fn Trait_static_work_Ok() {
            // Arrange
            TraitMock::static_setup()
                .static_work()
                .returns(Box::new(Bar));

            // Act
            let result = TraitMock::static_work();

            // Assert
            assert_eq!(result.name(), "Bar");
            TraitMock::static_received()
                .static_work(1.time())
                .no_other_calls();
        }

        #[test]
        fn TraitBase_work_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            mock.setup().work().call_base();

            // Act
            let result = mock.work();

            // Assert
            assert_eq!(result.name(), "Foo");
            mock.received().no_other_calls();
        }

        #[test]
        fn TraitBase_static_work_Ok() {
            // Arrange
            TraitMock::static_setup().static_work().call_base();

            // Act
            let result = TraitMock::static_work();

            // Assert
            assert_eq!(result.name(), "Foo");
            TraitMock::static_received().no_other_calls();
        }
    }

    mod r#struct {
        use super::*;

        #[test]
        fn Struct_work_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup().work().returns(Box::new(Bar));

            // Act
            let result = mock.work();

            // Assert
            assert_eq!(result.name(), "Bar");
            mock.received().work(1.time()).no_other_calls();
        }

        #[test]
        fn Struct_static_work_Ok() {
            // Arrange
            Struct::static_setup().static_work().returns(Box::new(Bar));

            // Act
            let result = Struct::static_work();

            // Assert
            assert_eq!(result.name(), "Bar");
            Struct::static_received()
                .static_work(1.time())
                .no_other_calls();
        }

        #[test]
        fn StructBase_work_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup().work().call_base();

            // Act
            let result = mock.work();

            // Assert
            assert_eq!(result.name(), "Foo");
            mock.received().no_other_calls();
        }

        #[test]
        fn StructBase_static_work_Ok() {
            // Arrange
            Struct::static_setup().static_work().call_base();

            // Act
            let result = Struct::static_work();

            // Assert
            assert_eq!(result.name(), "Foo");
            Struct::static_received().no_other_calls();
        }
    }

    mod struct_as_trait {
        use super::*;

        #[test]
        fn StructAsTrait_work_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup().as_Trait().work().returns(Box::new(Bar));

            // Act
            let result = Trait::work(&mock);

            // Assert
            assert_eq!(result.name(), "Bar");
            mock.received().as_Trait().work(1.time()).no_other_calls();
        }

        #[test]
        fn StructAsTrait_static_work_Ok() {
            // Arrange
            Struct::static_setup()
                .as_Trait()
                .static_work()
                .returns(Box::new(Bar));

            // Act
            let result = <Struct as Trait>::static_work();

            // Assert
            assert_eq!(result.name(), "Bar");
            Struct::static_received()
                .as_Trait()
                .static_work(1.time())
                .no_other_calls();
        }

        #[test]
        fn StructAsTraitBase_work_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            mock.setup().as_Trait().work().call_base();

            // Act
            let result = Trait::work(&mock);

            // Assert
            assert_eq!(result.name(), "Foo");
            mock.received().as_Trait().no_other_calls();
        }

        #[test]
        fn StructAsTraitBase_static_work_Ok() {
            // Arrange
            Struct::static_setup().as_Trait().static_work().call_base();

            // Act
            let result = <Struct as Trait>::static_work();

            // Assert
            assert_eq!(result.name(), "Foo");
            Struct::static_received().as_Trait().no_other_calls();
        }
    }
}
