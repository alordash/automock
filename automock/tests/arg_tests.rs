use automock::*;
use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
struct Payload<'a> {
    value: i32,
    reference: &'a i32,
}

impl<'a> PartialEq for Payload<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'a> Deref for Payload<'a> {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        self.reference
    }
}

#[mock]
fn accept_deref<'a>(_: Payload<'a>) {}

#[mock]
fn accept_ref(_: &i32) {}

#[mock]
fn accept_ptr(_: *const i32) {}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::assert_panics;

    #[test]
    fn accept_deref_ArgEq_ComparesUsingPartialEq() {
        // Arrange
        let reference = &1;
        let first = Payload {
            value: 10,
            reference,
        };
        let second = Payload {
            value: 20,
            reference,
        };

        // Act
        accept_deref(first);

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        accept_deref::received(Arg::eq(first), Times::Once)
            .received(Arg::eq(second), Times::Never)
            .no_other_calls();
    }

    #[test]
    fn accept_deref_ArgEq_WhenPartialEqReturnsFalse_Panics() {
        // Arrange
        let reference = &1;
        let first = Payload {
            value: 10,
            reference,
        };
        let second = Payload {
            value: 20,
            reference,
        };

        // Act
        accept_deref(first);

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        assert_panics(
            || accept_deref::received(Arg::eq(second), Times::Once),
            "Expected to receive a call exactly once matching:
	accept_deref((arg_tests::Payload<'_>): equal to Payload { value: 20, reference: 1 })
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_deref(*Payload { value: 10, reference: 1 }*)
	1. __arg0 (arg_tests::Payload<'_>):
		Expected: Payload { value: 20, reference: 1 }
		Actual:   Payload { value: 10, reference: 1 }",
        );
    }

    #[test]
    fn accept_deref_ArgRefEq_ComparesUsingDeref() {
        // Arrange
        let reference = &1;
        let first = Payload {
            value: 10,
            reference,
        };
        let second = Payload {
            value: 20,
            reference,
        };

        // Act
        accept_deref(first);

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        accept_deref::received(Arg::ref_eq(first), Times::Once)
            .received(Arg::ref_eq(second), Times::Once)
            .no_other_calls();
    }

    #[test]
    fn accept_deref_ArgRefNotEq_ComparesUsingDeref() {
        // Arrange
        let reference = &1;
        let first = Payload {
            value: 10,
            reference,
        };
        let second = Payload {
            value: 20,
            reference,
        };

        // Act
        accept_deref(first);

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        accept_deref::received(Arg::ref_not_eq(first), Times::Never)
            .received(Arg::ref_not_eq(second), Times::Never);
    }

    #[test]
    fn accept_deref_ArgRefEq_WhenDerefReturnsDifferentRef_Panics() {
        // Arrange
        let first_reference = &core::hint::black_box(1);
        let first = Payload {
            value: 10,
            reference: first_reference,
        };
        let second_reference = &core::hint::black_box(1);
        let second = Payload {
            value: 20,
            reference: second_reference,
        };

        // Act
        accept_deref(first);

        // Assert
        assert!(!core::ptr::eq(first_reference, second_reference));
        let first_ptr = first_reference as *const _;
        let second_ptr = second_reference as *const _;
        assert_panics(
            || accept_deref::received(Arg::ref_eq(second), Times::Once),
            format!(
                "Expected to receive a call exactly once matching:
	accept_deref((arg_tests::Payload<'_>): equal to Payload {{ value: 20, reference: 1 }})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_deref(*Payload {{ value: 10, reference: 1 }}*)
	1. __arg0 (arg_tests::Payload<'_>):
		Expected (ptr: {second_ptr:?}): Payload {{ value: 20, reference: 1 }}
		Actual   (ptr: {first_ptr:?}): Payload {{ value: 10, reference: 1 }}"
            ),
        );
    }

    #[test]
    fn accept_deref_ArgRefNotEq_WhenDerefReturnsDifferentRef_Panics() {
        // Arrange
        let first_reference = &core::hint::black_box(1);
        let first = Payload {
            value: 10,
            reference: first_reference,
        };
        let second_reference = &core::hint::black_box(1);
        let second = Payload {
            value: 20,
            reference: second_reference,
        };

        // Act
        accept_deref(first);

        // Assert
        assert!(!core::ptr::eq(first_reference, second_reference));
        assert_panics(
            || accept_deref::received(Arg::ref_not_eq(second), Times::Never),
            "Expected to never receive a call matching:
	accept_deref((arg_tests::Payload<'_>): NOT equal to Payload { value: 20, reference: 1 })
Actually received 1 matching call:
	accept_deref(Payload { value: 10, reference: 1 })
Received no non-matching calls",
        );
    }

    #[test]
    fn accept_ref_ArgEq_ComparesByReference() {
        // Arrange
        let first_reference = &core::hint::black_box(1);
        let second_reference = &core::hint::black_box(1);

        // Act
        accept_ref(first_reference);

        // Assert
        accept_ref::received(Arg::eq(first_reference), Times::Once).no_other_calls();
        accept_ref::received(Arg::eq(second_reference), Times::Once).no_other_calls();
    }

    #[test]
    fn accept_ref_ArgRefEq_ComparesByReference() {
        // Arrange
        let first_reference = &core::hint::black_box(1);
        let second_reference = &core::hint::black_box(1);

        // Act
        accept_ref(first_reference);

        // Assert
        accept_ref::received(Arg::ref_eq(first_reference), Times::Once).no_other_calls();
        accept_ref::received(Arg::ref_eq(second_reference), Times::Never).no_other_calls();
    }

    #[test]
    fn accept_ref_ArgRefNotEq_ComparesByReference() {
        // Arrange
        let first_reference = &core::hint::black_box(1);
        let second_reference = &core::hint::black_box(1);

        // Act
        accept_ref(first_reference);

        // Assert
        accept_ref::received(Arg::ref_not_eq(first_reference), Times::Never);
        accept_ref::received(Arg::ref_not_eq(second_reference), Times::Once).no_other_calls();
    }

    #[test]
    fn accept_ptr_ArgEq_ComparesByPointer() {
        // Arrange
        let first_pointer = &core::hint::black_box(1) as *const i32;
        let second_pointer = &core::hint::black_box(1) as *const i32;

        // Act
        accept_ptr(first_pointer);

        // Assert
        accept_ptr::received(Arg::eq(first_pointer), Times::Once).no_other_calls();
        accept_ptr::received(Arg::eq(second_pointer), Times::Never).no_other_calls();
    }
}
