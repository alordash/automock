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

trait IMutable {
    fn mutate(&mut self);
}
#[mock]
#[derive(Clone)]
struct Mutable;
#[mock]
impl IMutable for Mutable {
    fn mutate(&mut self) {}
}
#[mock]
impl Mutable {
    pub fn new() -> Self {
        Self
    }
}

#[mock]
fn accept_mut_impl(mut mutable: impl IMutable) {
    mutable.mutate()
}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::*;

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
        accept_deref::setup(Arg::eq(first)).expect_once();

        // Act
        accept_deref(first);

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        accept_deref::verify_calls();
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
        accept_deref::setup(Arg::eq(second)).expect_once();

        // Act
        let actual_error_msg = record_panic(|| accept_deref(first));

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        let expected_error_msg = "Mock wasn't configured to handle following call:
	accept_deref(Payload { value: 10, reference: 1 })
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_deref(*Payload { value: 10, reference: 1 }*)";
        assert_eq!(Some(expected_error_msg.to_string()), actual_error_msg);
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
        accept_deref::setup(Arg::ref_eq(second)).expect_once();

        // Act
        accept_deref(first);

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        accept_deref::verify_calls();
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
        accept_deref::setup(Arg::ref_not_eq(second)).expect_once();

        // Act
        let actual_error_msg = record_panic(|| accept_deref(first));

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        let expected_error_msg = "Mock wasn't configured to handle following call:
	accept_deref(Payload { value: 10, reference: 1 })
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_deref(*Payload { value: 10, reference: 1 }*)";
        assert_eq!(Some(expected_error_msg.to_string()), actual_error_msg);
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
        accept_deref::setup(Arg::ref_eq(second)).expect_once();

        // Act
        let actual_error_msg = record_panic(|| accept_deref(first));

        // Assert
        assert!(!core::ptr::eq(first_reference, second_reference));
        let expected_error_msg = "Mock wasn't configured to handle following call:
	accept_deref(Payload { value: 10, reference: 1 })
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_deref(*Payload { value: 10, reference: 1 }*)";
        assert_eq!(Some(expected_error_msg.to_string()), actual_error_msg);
    }

    #[test]
    fn accept_deref_ArgRefNotEq_WhenDerefReturnsDifferentRef_Ok() {
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
        accept_deref::setup(Arg::ref_not_eq(second)).expect_once();

        // Act
        accept_deref(first);

        // Assert
        assert!(!core::ptr::eq(first_reference, second_reference));
        accept_deref::verify_calls();
    }

    #[test]
    fn accept_ref_ArgEq_ComparesByReference() {
        // Arrange
        let first_reference = &core::hint::black_box(1);
        let second_reference = &core::hint::black_box(1);
        accept_ref::setup(Arg::eq(second_reference)).expect_once();

        // Act
        accept_ref(first_reference);

        // Assert
        accept_ref::verify_calls();
    }

    #[test]
    fn accept_ref_ArgRefEq_ComparesByReference() {
        // Arrange
        let first_reference = &core::hint::black_box(1);
        let second_reference = &core::hint::black_box(1);
        accept_ref::setup(Arg::ref_eq(first_reference)).expect_once();

        // Act
        accept_ref(first_reference);

        // Assert
        assert!(!core::ptr::eq(first_reference, second_reference));
        accept_ref::verify_calls();
    }

    #[test]
    fn accept_ref_ArgRefNotEq_ComparesByReference() {
        // Arrange
        let first_reference = &core::hint::black_box(1);
        let second_reference = &core::hint::black_box(1);
        accept_ref::setup(Arg::ref_not_eq(second_reference)).expect_once();

        // Act
        accept_ref(first_reference);

        // Assert
        accept_ref::verify_calls();
    }

    #[test]
    fn accept_ptr_ArgEq_ComparesByPointer() {
        // Arrange
        let first_pointer = &core::hint::black_box(1) as *const i32;
        let second_pointer = &core::hint::black_box(1) as *const i32;
        accept_ptr::setup(Arg::eq(first_pointer)).expect_once();

        // Act
        accept_ptr(first_pointer);

        // Assert
        assert_ne!(first_pointer, second_pointer);
        accept_ptr::verify_calls();
    }

    #[test]
    fn accept_mut_impl_NotBase_Ok() {
        // Arrange
        Mutable::static_setup().new().call_base();
        let mutable = Mutable::new();
        accept_mut_impl::setup(Arg::Any).expect_once();

        // Act
        accept_mut_impl(mutable.clone());

        // Assert
        accept_mut_impl::verify_calls();
    }

    #[test]
    fn accept_mut_impl_Base_Ok() {
        // Arrange
        Mutable::static_setup().new().call_base();
        let mut mutable = Mutable::new();
        mutable.setup().as_IMutable().mutate().expect_once();
        accept_mut_impl::setup(Arg::Any).call_base();

        // Act
        accept_mut_impl(mutable.clone());

        // Assert
        accept_mut_impl::verify_calls();

        mutable.verify_calls();
    }
}
