use automock::*;

#[mock]
fn base_dep() {}
#[mock]
fn callback_dep() {}

#[mock(base)]
fn f() {
    base_dep();
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn CallbackIsCalledBeforeBaseImpl() {
        // Arrange
        f::setup().call_base().and_does(|_| callback_dep());

        // Act
        f();

        // Assert
        verify_call_order(|| {
            callback_dep::received(Times::Once).no_other_calls();
            base_dep::received(Times::Once).no_other_calls();
        });
    }
}
