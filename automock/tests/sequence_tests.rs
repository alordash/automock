use automock::*;

mod common;

#[mock]
fn foo<T>(_: T) {}

#[mock]
fn bar<T>(_: T) {}

#[mock]
fn get(_: i32) -> i32 {
    0
}

#[mock]
trait Trait {
    fn traiting(&self, _: i32);
}

#[mock]
struct Struct;

#[mock]
impl Struct {
    fn new() -> Self {
        Self
    }

    fn structing(&self, _: i32) {}
}

#[mock]
impl Trait for Struct {
    fn traiting(&self, _: i32) {}
}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::assert_panics;

    #[test]
    fn OnlyFn_WithVerificationRepeatedExpect_Ok() {
        // Arrange
        let sequence = in_sequence(|| {
            foo::setup(1).expect(2.times()).setup(2).expect_once();
        });

        // Act
        foo(1);
        foo(1);
        foo(2);

        // Assert
        foo::verify_calls::<i32>();
        sequence.verify();
    }

    #[test]
    fn OnlyFn_WithVerificationRepeatedReturns_Ok() {
        // Arrange
        let sequence = in_sequence(|| {
            get::setup(1)
                .returns(10, 2.times())
                .setup(2)
                .returns_once(20);
        });

        // Act
        let first = get(1);
        let second = get(1);
        let third = get(2);

        // Assert
        assert_eq!(first, 10);
        assert_eq!(second, 10);
        assert_eq!(third, 20);
        get::verify_calls();
        sequence.verify();
    }

    #[test]
    fn OnlyFn_WithVerificationReturnsMany_Ok() {
        // Arrange
        let sequence = in_sequence(|| {
            get::setup(1)
                .returns_many([10, 20])
                .setup(2)
                .returns_once(30);
        });

        // Act
        let first = get(1);
        let second = get(1);
        let third = get(2);

        // Assert
        assert_eq!(first, 10);
        assert_eq!(second, 20);
        assert_eq!(third, 30);
        get::verify_calls();
        sequence.verify();
    }

    #[test]
    fn OnlyFn_WithVerificationRepeatedReturnsWith_Ok() {
        // Arrange
        let sequence = in_sequence(|| {
            get::setup(1)
                .returns_with(|(value,)| *value + 10, 2.times())
                .setup(2)
                .returns_once(20);
        });

        // Act
        let first = get(1);
        let second = get(1);
        let third = get(2);

        // Assert
        assert_eq!(first, 11);
        assert_eq!(second, 11);
        assert_eq!(third, 20);
        get::verify_calls();
        sequence.verify();
    }

    #[test]
    fn OnlyFn_WithVerificationInvalidOrderTwice_Panics() {
        // Arrange
        let sequence = in_sequence(|| {
            get::setup(1).returns_once(10).setup(2).returns_once(20);
        });
        let expected_error = r#"Expected to receive these calls in order:

	get(1)
	get(2)

Actually received matching calls in this order:

	get(2)
	get(1)
"#;

        // Act
        let first = get(2);
        let second = get(1);

        // Assert
        assert_eq!(first, 20);
        assert_eq!(second, 10);
        get::verify_calls();
        assert_panics(|| sequence.verify(), expected_error);
        sequence.verify();
    }

    #[test]
    fn OnlyFn_NoVerificationValidOrder_Ok() {
        // Arrange
        foo::setup(1).expect_once().setup(2).expect_once();
        foo::setup("amogus").expect_once();
        bar::setup(false).expect_once();

        // Act
        foo(1);
        foo(2);
        foo("amogus");
        bar(false);

        // Assert
        foo::verify_calls::<i32>();
        foo::verify_calls::<&str>();
        bar::verify_calls::<bool>();
    }

    #[test]
    fn OnlyFn_NoVerificationInvalidOrder_Ok() {
        // Arrange
        foo::setup(1).expect_once().setup(2).expect_once();
        foo::setup("amogus").expect_once();
        bar::setup(false).expect_once();

        // Act
        bar(false);
        foo("amogus");
        foo(2);
        foo(1);

        // Assert
        foo::verify_calls::<i32>();
        foo::verify_calls::<&str>();
        bar::verify_calls::<bool>();
    }

    #[test]
    fn OnlyFn_WithVerificationValidOrder_Ok() {
        // Arrange
        let sequence = in_sequence(|| {
            foo::setup(1).expect_once().setup(2).expect_once();
            foo::setup("amogus").expect_once();
            bar::setup(false).expect_once();
        });

        // Act
        foo(1);
        foo(2);
        foo("amogus");
        bar(false);

        // Assert
        sequence.verify();
    }

    #[test]
    fn OnlyFn_WithVerification_Panics() {
        // Arrange
        let sequence = in_sequence(|| {
            foo::setup(1).expect_once().setup(2).expect_once();
            foo::setup("amogus").expect_once();
            bar::setup(false).expect_once();
        });

        // Act
        bar(false);
        foo("amogus");
        foo(2);
        foo(1);

        // Assert
        let first_foo_debug_string = common::debug_string("1");
        let second_foo_debug_string = common::debug_string("2");
        let third_foo_debug_string = common::debug_string(r#""amogus""#);
        let bar_debug_string = common::debug_string("false");
        assert_panics(
            || sequence.verify(),
            format!(
                r#"Expected to receive these calls in order:

	foo({first_foo_debug_string})
	foo({second_foo_debug_string})
	foo({third_foo_debug_string})
	bar({bar_debug_string})

Actually received matching calls in this order:

	bar({bar_debug_string})
	foo({third_foo_debug_string})
	foo({second_foo_debug_string})
	foo({first_foo_debug_string})
"#
            ),
        );
    }

    #[test]
    fn AllTogether_NoVerificationValidOrder_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        Struct::static_setup().new().call_base();
        let mut struct_mock = Struct::new();
        foo::setup(1).expect_once();
        trait_mock.setup().traiting(2).expect_once();
        struct_mock.setup().structing(3).expect_once();
        bar::setup(4).expect_once();
        struct_mock.setup().as_Trait().traiting(5).expect_once();

        // Act
        foo(1);
        trait_mock.traiting(2);
        struct_mock.structing(3);
        bar(4);
        struct_mock.traiting(5);

        // Assert
        foo::verify_calls::<i32>();
        trait_mock.verify_calls();
        struct_mock.verify_calls();
        bar::verify_calls::<i32>();
    }

    #[test]
    fn AllTogether_NoVerificationInvalidOrder_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        Struct::static_setup().new().call_base();
        let mut struct_mock = Struct::new();
        foo::setup(1).expect_once();
        trait_mock.setup().traiting(2).expect_once();
        struct_mock.setup().structing(3).expect_once();
        bar::setup(4).expect_once();
        struct_mock.setup().as_Trait().traiting(5).expect_once();

        // Act
        struct_mock.traiting(5);
        bar(4);
        struct_mock.structing(3);
        trait_mock.traiting(2);
        foo(1);

        // Assert
        foo::verify_calls::<i32>();
        trait_mock.verify_calls();
        struct_mock.verify_calls();
        bar::verify_calls::<i32>();
    }

    #[test]
    fn AllTogether_WithVerificationValidOrder_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        Struct::static_setup().new().call_base();
        let mut struct_mock = Struct::new();
        let sequence = in_sequence(|| {
            foo::setup(1).expect_once();
            trait_mock.setup().traiting(2).expect_once();
            struct_mock.setup().structing(3).expect_once();
            bar::setup(4).expect_once();
            struct_mock.setup().as_Trait().traiting(5).expect_once();
        });

        // Act
        foo(1);
        trait_mock.traiting(2);
        struct_mock.structing(3);
        bar(4);
        struct_mock.traiting(5);

        // Assert
        sequence.verify();
    }

    #[test]
    fn AllTogether_WithVerificationInvalidOrder_Panics() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        Struct::static_setup().new().call_base();
        let mut struct_mock = Struct::new();
        let sequence = in_sequence(|| {
            foo::setup(1).expect_once();
            trait_mock.setup().traiting(2).expect_once();
            struct_mock.setup().structing(3).expect_once();
            bar::setup(4).expect_once();
            struct_mock.setup().as_Trait().traiting(5).expect_once();
        });

        // Act
        struct_mock.traiting(5);
        bar(4);
        struct_mock.structing(3);
        trait_mock.traiting(2);
        foo(1);

        // Assert
        let foo_debug_string = common::debug_string("1");
        let bar_debug_string = common::debug_string("4");
        assert_panics(
            || sequence.verify(),
            format!(
                r#"Expected to receive these calls in order:

	foo({foo_debug_string})
	Trait::traiting(2)
	Struct::structing(3)
	bar({bar_debug_string})
	<Struct as Trait>::traiting(5)

Actually received matching calls in this order:

	<Struct as Trait>::traiting(5)
	bar({bar_debug_string})
	Struct::structing(3)
	Trait::traiting(2)
	foo({foo_debug_string})
"#
            ),
        );
    }
}
