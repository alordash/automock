use automock::*;

mod a {
    use super::*;
    #[mock]
    pub struct Struct;
    #[mock]
    impl Struct {
        pub fn work() -> i32 {
            unreachable!()
        }
    }
}

mod ta {
    pub trait Trait {
        fn work() -> i32;
    }
}

#[mock]
impl ta::Trait for a::Struct {
    fn work() -> i32 {
        unreachable!()
    }
}

#[mock]
impl ta::Trait for b::Struct {
    fn work() -> i32 {
        unreachable!()
    }
}

mod b {
    use super::*;
    #[mock]
    pub struct Struct;
    #[mock]
    impl Struct {
        pub fn work() -> i32 {
            unreachable!()
        }
    }
}

mod tb {
    pub trait Trait {
        fn work() -> i32;
    }
}

#[mock]
impl tb::Trait for a::Struct {
    fn work() -> i32 {
        unreachable!()
    }
}

#[mock]
impl tb::Trait for b::Struct {
    fn work() -> i32 {
        unreachable!()
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    #[test]
    fn DiffStructSetup_work_DoesNotAffectAnotherModule() {
        // Arrange
        a::Struct::static_setup().work().returns(10);

        // Act
        let panic_message = record_panic(b::Struct::work);

        // Assert
        assert_eq!(
            panic_message,
            Some(
                "Mock wasn't configured to handle following call:
	Struct::work()"
                    .to_owned()
            )
        );
    }

    #[test]
    fn DiffTraitSameStructSetup_work_DoesNotAffectAnotherModule() {
        // Arrange
        a::Struct::static_setup().as_ta_Trait().work().returns(10);

        // Act
        let panic_message = record_panic(<a::Struct as tb::Trait>::work);

        // Assert
        assert_eq!(
            panic_message,
            Some(
                "Mock wasn't configured to handle following call:
	<Struct as tb_Trait>::work()"
                    .to_owned()
            )
        );
    }

    #[test]
    fn SameTraitDiffStructSetup_work_DoesNotAffectAnotherModule() {
        // Arrange
        a::Struct::static_setup().as_ta_Trait().work().returns(10);

        // Act
        let panic_message = record_panic(<b::Struct as ta::Trait>::work);

        // Assert
        assert_eq!(
            panic_message,
            Some(
                "Mock wasn't configured to handle following call:
	<Struct as ta_Trait>::work()"
                    .to_owned()
            )
        );
    }

    #[test]
    fn DiffTraitDiffStructSetup_work_DoesNotAffectAnotherModule() {
        // Arrange
        a::Struct::static_setup().as_ta_Trait().work().returns(10);

        // Act
        let panic_message = record_panic(<b::Struct as tb::Trait>::work);

        // Assert
        assert_eq!(
            panic_message,
            Some(
                "Mock wasn't configured to handle following call:
	<Struct as tb_Trait>::work()"
                    .to_owned()
            )
        );
    }
}
