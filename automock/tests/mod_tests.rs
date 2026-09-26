use automock::*;

mod a {
    pub trait Trait {
        fn work(&self) -> i32;
    }
}
mod b {
    pub trait Trait {
        fn work(&self) -> i32;
    }
}
mod c {
    use automock::*;
    #[mock]
    pub struct S;
}
#[mock(kavo)]
impl a::Trait for c::S {
    fn work(&self) -> i32 {
        unreachable!()
    }
}
#[mock]
impl b::Trait for c::S {
    fn work(&self) -> i32 {
        unreachable!()
    }
}
#[mock(base)]
impl c::S {
    pub fn new() -> Self {
        Self
    }
}

mod tests {
    use super::*;
    use automock_core::infrastructure::Mockable;

    #[test]
    fn ok() {
        // Arrange
        c::S::static_setup().new().call_base();
        let mut mock = c::S::new();

        let a_return_value = 10;
        let b_return_value = 20;

        mock.setup().as_a_Trait().work().returns(a_return_value);
        mock.setup().as_b_Trait().work().returns(b_return_value);

        // Act
        let a_result = a::Trait::work(&mock);
        let b_result = b::Trait::work(&mock);

        // Assert
        assert_eq!(a_return_value, a_result);
        assert_eq!(b_return_value, b_result);
        mock.received().as_a_Trait().work(Times::Once);
        mock.received().as_b_Trait().work(Times::Once);
        mock.received().no_other_calls();
    }
}
