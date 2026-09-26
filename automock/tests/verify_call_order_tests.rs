use automock::*;

mod common;

#[mock]
fn foo<T>(_: T) {}

#[mock]
fn bar<T>(_: T) {}

#[mock]
trait Trait {
    fn traiting(&self, _: i32);
}

#[mock]
struct Struct;

#[mock(base)]
impl Struct {
    fn new() -> Self {
        Self
    }

    fn structing(&self, _: i32) {}
}

#[mock(base)]
impl Trait for Struct {
    fn traiting(&self, _: i32) {}
}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::assert_panics;

    #[test]
    fn OnlyFn_NoVerificationValidOrder_Ok() {
        // Arrange
        // Act
        foo(1);
        foo(2);
        foo("amogus");
        bar(false);

        // Assert
        foo::received(1, Times::Once);
        foo::received(2, Times::Once);
        foo::received("amogus", Times::Once).no_other_calls();
        bar::received(false, Times::Once).no_other_calls();
    }

    #[test]
    fn OnlyFn_NoVerificationInvalidOrder_Ok() {
        // Arrange
        // Act
        bar(false);
        foo("amogus");
        foo(2);
        foo(1);

        // Assert
        foo::received(1, Times::Once);
        foo::received(2, Times::Once);
        foo::received("amogus", Times::Once).no_other_calls();
        bar::received(false, Times::Once).no_other_calls();
    }

    #[test]
    fn OnlyFn_WithVerificationValidOrder_Ok() {
        // Arrange
        // Act
        foo(1);
        foo(2);
        foo("amogus");
        bar(false);

        // Assert
        verify_call_order(|| {
            foo::received(1, Times::Once);
            foo::received(2, Times::Once);
            foo::received("amogus", Times::Once).no_other_calls();
            bar::received(false, Times::Once).no_other_calls();
        });
    }

    #[test]
    fn OnlyFn_WithVerification_Panics() {
        // Arrange
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
            || {
                verify_call_order(|| {
                    foo::received(1, Times::Once);
                    foo::received(2, Times::Once);
                    foo::received("amogus", Times::Once).no_other_calls();
                    bar::received(false, Times::Once).no_other_calls();
                })
            },
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

        // Act
        foo(1);
        trait_mock.traiting(2);
        struct_mock.structing(3);
        bar(4);
        struct_mock.traiting(5);

        // Assert
        foo::received(1, Times::Once).no_other_calls();
        trait_mock
            .received()
            .traiting(2, Times::Once)
            .no_other_calls();
        struct_mock
            .received()
            .structing(3, Times::Once)
            .no_other_calls();
        bar::received(4, Times::Once).no_other_calls();
        struct_mock.received().as_Trait().traiting(5, Times::Once);
    }

    #[test]
    fn AllTogether_NoVerificationInvalidOrder_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        Struct::static_setup().new().call_base();
        let mut struct_mock = Struct::new();

        // Act
        struct_mock.traiting(5);
        bar(4);
        struct_mock.structing(3);
        trait_mock.traiting(2);
        foo(1);

        // Assert
        foo::received(1, Times::Once).no_other_calls();
        trait_mock
            .received()
            .traiting(2, Times::Once)
            .no_other_calls();
        struct_mock.received().structing(3, Times::Once);
        bar::received(4, Times::Once).no_other_calls();
        struct_mock
            .received()
            .as_Trait()
            .traiting(5, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn AllTogether_WithVerificationValidOrder_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        Struct::static_setup().new().call_base();
        let mut struct_mock = Struct::new();

        // Act
        foo(1);
        trait_mock.traiting(2);
        struct_mock.structing(3);
        bar(4);
        struct_mock.traiting(5);

        // Assert
        verify_call_order(|| {
            foo::received(1, Times::Once).no_other_calls();
            trait_mock
                .received()
                .traiting(2, Times::Once)
                .no_other_calls();
            struct_mock
                .received()
                .structing(3, Times::Once)
                .no_other_calls();
            bar::received(4, Times::Once).no_other_calls();
            struct_mock.received().as_Trait().traiting(5, Times::Once);
        });
    }

    #[test]
    fn AllTogether_WithVerificationInvalidOrder_Panics() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        Struct::static_setup().new().call_base();
        let mut struct_mock = Struct::new();

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
            || {
                verify_call_order(|| {
                    foo::received(1, Times::Once).no_other_calls();
                    trait_mock
                        .received()
                        .traiting(2, Times::Once)
                        .no_other_calls();
                    struct_mock.received().structing(3, Times::Once);
                    bar::received(4, Times::Once).no_other_calls();
                    struct_mock
                        .received()
                        .as_Trait()
                        .traiting(5, Times::Once)
                        .no_other_calls();
                });
            },
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
