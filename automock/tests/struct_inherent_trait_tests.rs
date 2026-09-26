use automock::*;

pub trait A {
    fn foo(&self) -> i32;
}

pub trait B: A {
    fn bar(&self) -> i32;
}

#[mock]
struct C;

#[mock(base)]
impl C {
    fn new() -> Self {
        Self
    }
}

const A_DEFAULT: i32 = 11;
#[mock(base)]
impl A for C {
    fn foo(&self) -> i32 {
        A_DEFAULT
    }
}

const B_DEFAULT: i32 = 11;
#[mock(base)]
impl B for C {
    fn bar(&self) -> i32 {
        B_DEFAULT
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn A_Mocked_Ok() {
        // Arrange
        C::static_setup().new().call_base();
        let mut mock = C::new();
        mock.setup().as_A().foo().returns(42);

        // Act
        let result = mock.foo();

        // Assert
        assert_eq!(42, result);
        mock.received().as_A().foo(1.time());
    }

    #[test]
    fn A_Base_Ok() {
        // Arrange
        C::static_setup().new().call_base();
        let mut mock = C::new();
        mock.setup().as_A().foo().call_base();

        // Act
        let result = mock.foo();

        // Assert
        assert_eq!(A_DEFAULT, result);
        mock.received().no_other_calls();
    }

    #[test]
    fn B_Mocked_Ok() {
        // Arrange
        C::static_setup().new().call_base();
        let mut mock = C::new();
        mock.setup().as_B().bar().returns(42);

        // Act
        let result = mock.bar();

        // Assert
        assert_eq!(42, result);
        mock.received().as_B().bar(1.time());
    }

    #[test]
    fn B_Base_Ok() {
        // Arrange
        C::static_setup().new().call_base();
        let mut mock = C::new();
        mock.setup().as_B().bar().call_base();

        // Act
        let result = mock.bar();

        // Assert
        assert_eq!(B_DEFAULT, result);
        mock.received().no_other_calls();
    }
}
