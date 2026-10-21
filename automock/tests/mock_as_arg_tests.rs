use automock::*;

const DEFAULT_VALUE: i32 = 1;
#[mock]
fn f() -> i32 {
    DEFAULT_VALUE
}

mod fn_tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn MockedFnPassedAsArg_UsesMockedFn() {
        // Arrange
        fn g(r#fn: fn() -> i32) -> i32 {
            r#fn()
        }

        let mocked_value = DEFAULT_VALUE + 1;
        f::setup().returns_once(mocked_value);

        // Act
        let result = g(f);

        // Assert
        assert_eq!(mocked_value, result);
        f::verify_calls();
    }
}

#[mock]
trait Trait {
    fn f(&self) -> i32 {
        DEFAULT_VALUE
    }

    fn static_f() -> i32 {
        DEFAULT_VALUE
    }
}

mod trait_tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn MockedTraitPassedAsGeneric_AssociatedFn_UsesMockedTrait() {
        // Arrange
        fn g<T: Trait + Sized>(t: &T) -> i32 {
            t.f()
        }

        let mocked_value = DEFAULT_VALUE + 1;
        let mut mock = TraitMock::new();
        mock.setup().f().returns_once(mocked_value);

        // Act
        let result = g(&mock);

        // Assert
        assert_eq!(mocked_value, result);
        mock.verify_calls();
    }

    #[test]
    fn MockedTraitPassedAsGeneric_StaticFn_UsesMockedTrait() {
        // Arrange
        fn g_static<T: Trait>() -> i32 {
            T::static_f()
        }

        let mocked_value = DEFAULT_VALUE + 1;
        TraitMock::static_setup()
            .static_f()
            .returns_once(mocked_value);

        // Act
        let static_result = g_static::<TraitMock>();

        // Assert
        assert_eq!(mocked_value, static_result);
        TraitMock::verify_static_calls();
    }
}

#[mock]
struct Struct;

#[mock]
impl Struct {
    #[allow(unused)]
    pub fn new() -> Self {
        Self
    }
}

#[mock]
impl Struct {
    pub fn f(&self) -> i32 {
        DEFAULT_VALUE
    }
}

mod struct_tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn MockedStructPassedAsArgument_UsesMockedStruct() {
        // Arrange
        fn g(s: &Struct) -> i32 {
            s.f()
        }

        let mocked_value = DEFAULT_VALUE + 1;
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        mock.setup().f().returns_once(mocked_value);

        // Act
        let result = g(&mock);

        // Assert
        assert_eq!(mocked_value, result);
        mock.verify_calls();
    }
}
