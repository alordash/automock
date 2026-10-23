use automock::*;

#[mock]
fn f() {}

#[mock]
trait Trait {
    fn f();
}

#[mock]
struct Struct;

#[mock]
impl Struct {
    fn f() {}
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    #[test]
    fn fn_Setup_ClearsPreviousSetups() {
        // Arrange
        f::setup().expect_once().and_does(|_| panic!("This callback should not have been called because it's configuration should've been overwritten by consecutive setup."));

        // Act
        f::setup().expect_once();
        f();

        // Assert
        f::verify_calls();
    }

    #[test]
    fn fn_Setup_ClearsPreviousExpectedCalls() {
        // Arrange
        f::setup().expect(2.times());

        // Act
        f();
        let actual_error_msg = record_panic(f::verify_calls);
        f::setup();

        // Assert
        let expected_error_msg = "Expected following calls:
	f() 1 time";
        assert_eq!(Some(expected_error_msg.to_owned()), actual_error_msg);
        f::verify_calls();
    }

    #[test]
    fn trait_Setup_ClearsPreviousSetups() {
        // Arrange
        TraitMock::static_setup().f().expect_once().and_does(|_| panic!("This callback should not have been called because it's configuration should've been overwritten by consecutive setup."));

        // Act
        TraitMock::static_setup().f().expect_once();
        TraitMock::f();

        // Assert
        TraitMock::verify_static_calls();
    }

    #[test]
    fn trait_Setup_ClearsPreviousExpectedCalls() {
        // Arrange
        TraitMock::static_setup().f().expect(2.times());

        // Act
        TraitMock::f();
        let actual_error_msg = record_panic(TraitMock::verify_static_calls);
        TraitMock::static_setup();

        // Assert
        let expected_error_msg = "Expected following calls:
	Trait::f() 1 time";
        assert_eq!(Some(expected_error_msg.to_owned()), actual_error_msg);
        TraitMock::verify_static_calls();
    }

    #[test]
    fn struct_Setup_ClearsPreviousSetups() {
        // Arrange
        Struct::static_setup().f().expect_once().and_does(|_| panic!("This callback should not have been called because it's configuration should've been overwritten by consecutive setup."));

        // Act
        Struct::static_setup().f().expect_once();
        Struct::f();

        // Assert
        Struct::verify_static_calls();
    }

    #[test]
    fn struct_Setup_ClearsPreviousExpectedCalls() {
        // Arrange
        Struct::static_setup().f().expect(2.times());

        // Act
        Struct::f();
        let actual_error_msg = record_panic(Struct::verify_static_calls);
        Struct::static_setup();

        // Assert
        let expected_error_msg = "Expected following calls:
	Struct::f() 1 time";
        assert_eq!(Some(expected_error_msg.to_owned()), actual_error_msg);
        Struct::verify_static_calls();
    }
}
