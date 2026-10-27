use automock::*;
use std::fmt::Debug;

mod common;

#[derive(Debug, PartialEq)]
struct Payload(i32);

#[mock]
fn accept<T>(_: T) {}

#[mock]
fn accept_debug<T: Debug>(_: T) {}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::record_panic;

    #[cfg(not(feature = "debug_naming"))]
    #[test]
    fn accept_WithoutDebugNaming_ReturnsQuestions() {
        // Arrange
        accept::setup(Payload(20)).expect_once();

        // Act
        let call_panic_message = record_panic(|| accept(Payload(10)));
        let verify_panic_message = record_panic(accept::verify_calls::<Payload>);

        // Assert
        assert_eq!(
            call_panic_message,
            Some(
                r#"Mock wasn't configured to handle following call:
	accept<debug_naming_tests::Payload>(?)
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept(*?*)"#
                    .to_owned()
            )
        );
        assert_eq!(
            verify_panic_message,
            Some(
                r#"Expected following calls:
	accept<debug_naming_tests::Payload>(?) 1 time"#
                    .to_owned()
            )
        );
    }

    #[cfg(feature = "debug_naming")]
    #[test]
    fn accept_WithDebugNaming_ReturnsDebugStrings() {
        // Arrange
        accept::setup(Payload(20)).expect_once();

        // Act
        let call_panic_message = record_panic(|| accept(Payload(10)));
        let verify_panic_message = record_panic(accept::verify_calls::<Payload>);

        // Assert
        assert_eq!(
            call_panic_message,
            Some(
                r#"Mock wasn't configured to handle following call:
	accept<debug_naming_tests::Payload>(Payload(10))
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept(*Payload(10)*)"#
                    .to_owned()
            )
        );
        assert_eq!(
            verify_panic_message,
            Some(
                r#"Expected following calls:
	accept<debug_naming_tests::Payload>(debug_naming_tests::Payload == Payload(20)) 1 time"#
                    .to_owned()
            )
        );
    }

    #[cfg(not(feature = "debug_naming"))]
    #[test]
    fn accept_debug_WithoutDebugNaming_ReturnsDebugStrings() {
        // Arrange
        accept_debug::setup(Payload(20)).expect_once();

        // Act
        let call_panic_message = record_panic(|| accept_debug(Payload(10)));
        let verify_panic_message = record_panic(accept_debug::verify_calls::<Payload>);

        // Assert
        assert_eq!(
            call_panic_message,
            Some(
                r#"Mock wasn't configured to handle following call:
	accept_debug<debug_naming_tests::Payload>(Payload(10))
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_debug(*Payload(10)*)"#
                    .to_owned()
            )
        );
        assert_eq!(
            verify_panic_message,
            Some(
                r#"Expected following calls:
	accept_debug<debug_naming_tests::Payload>(debug_naming_tests::Payload == Payload(20)) 1 time"#
                    .to_owned()
            )
        );
    }

    #[cfg(feature = "debug_naming")]
    #[test]
    fn accept_debug_WithDebugNaming_ReturnsDebugStrings() {
        // Arrange
        accept_debug::setup(Payload(20)).expect_once();

        // Act
        let call_panic_message = record_panic(|| accept_debug(Payload(10)));
        let verify_panic_message = record_panic(accept_debug::verify_calls::<Payload>);

        // Assert
        assert_eq!(
            call_panic_message,
            Some(
                r#"Mock wasn't configured to handle following call:
	accept_debug<debug_naming_tests::Payload>(Payload(10))
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_debug(*Payload(10)*)"#
                    .to_owned()
            )
        );
        assert_eq!(
            verify_panic_message,
            Some(
                r#"Expected following calls:
	accept_debug<debug_naming_tests::Payload>(debug_naming_tests::Payload == Payload(20)) 1 time"#
                    .to_owned()
            )
        );
    }
}
