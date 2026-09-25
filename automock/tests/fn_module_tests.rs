use automock::*;

mod a {
    use super::*;
    #[allow(unused)]
    #[mock]
    pub fn work() -> i32 {
        unreachable!()
    }
}

mod b {
    use super::*;
    #[mock]
    pub fn work() -> i32 {
        unreachable!()
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    #[test]
    fn SameNameSetupInOneModule_work_DoesNotAffectAnotherModule() {
        // Arrange
        a::work::setup().returns(10);

        // Act
        let panic_message = record_panic(b::work);

        // Assert
        assert_eq!(
            panic_message,
            Some(
                "Mock wasn't configured to handle following call:
	work()"
                    .to_owned()
            )
        );
    }
}
