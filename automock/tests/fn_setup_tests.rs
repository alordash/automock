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

    #[test]
    fn fn_Setup_ClearsPreviousSetups() {
        // Arrange
        f::setup().does(|_| panic!("This callback should not have been called because it's configuration should've been overwritten by consecutive setup."));

        // Act
        f::setup();
        f();

        // Assert
    }

    #[test]
    fn fn_Setup_ClearsPreviousReceivedCalls() {
        // Arrange
        // Act
        f();
        f::received(Times::Once).no_other_calls();
        f::setup();

        // Assert
        f::received_nothing();
    }

    #[test]
    fn trait_Setup_ClearsPreviousSetups() {
        // Arrange
        TraitMock::static_setup().f().does(|_| panic!("This callback should not have been called because it's configuration should've been overwritten by consecutive setup."));

        // Act
        TraitMock::static_setup().f();
        TraitMock::f();

        // Assert
    }

    #[test]
    fn trait_Setup_ClearsPreviousReceivedCalls() {
        // Arrange
        // Act
        TraitMock::f();
        TraitMock::static_received().f(Times::Once).no_other_calls();
        TraitMock::static_setup();

        // Assert
        TraitMock::static_received().no_other_calls();
    }

    #[test]
    fn struct_Setup_ClearsPreviousSetups() {
        // Arrange
        Struct::static_setup().f().does(|_| panic!("This callback should not have been called because it's configuration should've been overwritten by consecutive setup."));

        // Act
        Struct::static_setup().f();
        Struct::f();

        // Assert
    }

    #[test]
    fn struct_Setup_ClearsPreviousReceivedCalls() {
        // Arrange
        // Act
        Struct::f();
        Struct::static_received().f(Times::Once).no_other_calls();
        Struct::static_setup();

        // Assert
        Struct::static_received().no_other_calls();
    }
}
