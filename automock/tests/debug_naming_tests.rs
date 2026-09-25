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
        // Act
        accept(Payload(10));
        let panic_message = record_panic(|| {
            accept::received(Payload(20), 1.time());
        });

        // Assert
        assert_eq!(
            panic_message,
            Some(
                r#"Expected to receive a call exactly once matching:
	accept<debug_naming_tests::Payload>(?)
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept(*?*)
	1. __arg0 (debug_naming_tests::Payload):
		Expected: ?
		Actual:   ?"#
                    .to_owned()
            )
        );
    }

    #[cfg(feature = "debug_naming")]
    #[test]
    fn accept_WithDebugNaming_ReturnsDebugStrings() {
        // Arrange
        // Act
        accept(Payload(10));
        let panic_message = record_panic(|| {
            accept::received(Payload(20), 1.time());
        });

        // Assert
        assert_eq!(
            panic_message,
            Some(
                r#"Expected to receive a call exactly once matching:
	accept<debug_naming_tests::Payload>((debug_naming_tests::Payload): equal to Payload(20))
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept(*Payload(10)*)
	1. __arg0 (debug_naming_tests::Payload):
		Expected: Payload(20)
		Actual:   Payload(10)"#
                    .to_owned()
            )
        );
    }

    #[cfg(not(feature = "debug_naming"))]
    #[test]
    fn accept_debug_WithoutDebugNaming_ReturnsDebugStrings() {
        // Arrange
        // Act
        accept_debug(Payload(10));
        let panic_message = record_panic(|| {
            accept_debug::received(Payload(20), 1.time());
        });

        // Assert
        assert_eq!(
            panic_message,
            Some(
                r#"Expected to receive a call exactly once matching:
	accept_debug<debug_naming_tests::Payload>((debug_naming_tests::Payload): equal to Payload(20))
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_debug(*Payload(10)*)
	1. __arg0 (debug_naming_tests::Payload):
		Expected: Payload(20)
		Actual:   Payload(10)"#
                    .to_owned()
            )
        );
    }

    #[cfg(feature = "debug_naming")]
    #[test]
    fn accept_debug_WithDebugNaming_ReturnsDebugStrings() {
        // Arrange
        // Act
        accept_debug(Payload(10));
        let panic_message = record_panic(|| {
            accept_debug::received(Payload(20), 1.time());
        });

        // Assert
        assert_eq!(
            panic_message,
            Some(
                r#"Expected to receive a call exactly once matching:
	accept_debug<debug_naming_tests::Payload>((debug_naming_tests::Payload): equal to Payload(20))
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_debug(*Payload(10)*)
	1. __arg0 (debug_naming_tests::Payload):
		Expected: Payload(20)
		Actual:   Payload(10)"#
                    .to_owned()
            )
        );
    }
}
