use automock::*;

mod a {
    use super::*;
    #[mock]
    pub trait Trait {
        #[allow(unused)]
        fn work() -> i32;
    }
}

mod b {
    use super::*;
    #[mock]
    pub trait Trait {
        fn work() -> i32;
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    #[test]
    fn DiffTraitSetup_work_DoesNotAffectAnotherModule() {
        // Arrange
        a::TraitMock::static_setup().work().returns(10);

        // Act
        let panic_message = record_panic(|| {
            use crate::b::Trait;
            b::TraitMock::work()
        });

        // Assert
        assert_eq!(
            panic_message,
            Some(
                "Mock wasn't configured to handle following call:
	Trait::work()"
                    .to_owned()
            )
        );
    }
}
