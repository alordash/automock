use automock::*;

#[mock]
fn base_dep() {}
#[mock]
fn callback_dep() {}

#[mock]
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
        let sequence = in_sequence(|| {
            callback_dep::setup().expect_once();
            base_dep::setup().expect_once();
        });

        // Act
        f();

        // Assert
        sequence.verify();
    }
}
