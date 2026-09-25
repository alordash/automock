use automock::*;

#[mock]
struct Struct;

#[mock(base)]
impl Struct {
    pub fn new() -> Self {
        Self
    }
}

#[mock(base)]
impl Struct {
    pub fn accept_ref(&self, _: &i32) {}

    pub fn return_ref(&self) -> &'static i32 {
        unreachable!()
    }

    pub fn accept_ref_return_ref(&self, _: &i32) -> &'static i32 {
        unreachable!()
    }

    pub fn accept_two_refs(&self, _: &i32, _: &f32) {}

    pub fn accept_two_refs_return_ref(&self, _: &i32, _: &f32) -> &'static str {
        unreachable!()
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    mod accept_ref_tests {
        use super::*;

        #[test]
        fn accept_ref_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let r = &1;

            // Act
            mock.accept_ref(r);

            // Assert
            mock.received().accept_ref(r, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_rc_Panics() {
            // Arrange
            let mut mock = Struct::new();
            let r = &11;
            let r_ptr = core::ptr::from_ref(r);

            // Act
            mock.accept_ref(r);

            // Assert
            assert_panics(
                || mock.received().accept_ref(Arg::Any, Times::Never),
                format!(
                    "Expected to never receive a call matching:
	Struct::accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
                ),
            );

            assert_panics(
                || mock.received().accept_ref(Arg::Any, Times::Exactly(3)),
                format!(
                    "Expected to receive a call 3 times matching:
	Struct::accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
                ),
            );

            let invalid_r = &22;
            assert_panics(
                || mock.received().accept_ref(invalid_r, Times::Once),
                format!(
                    "Expected to receive a call exactly once matching:
	Struct::accept_ref((&i32): equal to {invalid_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref(*{r}*)
	1. __arg0 (&i32):
		Expected: {invalid_r}
		Actual:   {r}"
                ),
            );

            let invalid_r = &22;
            let invalid_r_ptr = core::ptr::from_ref(invalid_r);
            assert_panics(
                || {
                    mock.received()
                        .accept_ref(Arg::ref_eq(invalid_r), Times::Once)
                },
                format!(
                    "Expected to receive a call exactly once matching:
	Struct::accept_ref((&i32): equal to {invalid_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref(*{r}*)
	1. __arg0 (&i32):
		Expected (ptr: {invalid_r_ptr:?}): {invalid_r}
		Actual   (ptr: {r_ptr:?}): {r}"
                ),
            );
        }
    }

    mod return_ref_tests {
        use super::*;

        #[test]
        fn return_ref_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let r = Box::leak(Box::new(11));
            mock.setup().return_ref().returns(r);

            // Act
            let actual_r = mock.return_ref();

            // Assert
            assert_eq!(r, actual_r);
        }
    }

    mod accept_ref_return_ref_tests {
        use super::*;

        #[test]
        fn accept_ref_return_ref_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let accepted_r = &10;
            let returned_r = &20;
            mock.setup()
                .accept_ref_return_ref(accepted_r)
                .returns(returned_r);

            // Act
            let actual_returned_r = mock.accept_ref_return_ref(accepted_r);

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            mock.received()
                .accept_ref_return_ref(accepted_r, Times::Once)
                .accept_ref_return_ref(Arg::not_eq(accepted_r), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_refs_tests {
        use super::*;

        #[test]
        fn accept_two_refs_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let r1 = &10;
            let r2 = &20.2;

            // Act
            mock.accept_two_refs(r1, r2);

            // Assert
            mock.received()
                .accept_two_refs(r1, r2, Times::Once)
                .accept_two_refs(Arg::not_eq(r1), Arg::not_eq(r2), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_refs_return_ref_tests {
        use super::*;

        #[test]
        fn accept_two_refs_return_ref_Ok() {
            // Arrange
            let mut mock = Struct::new();
            let r1 = &10;
            let r2 = &20.2;
            let returned_r = "veridis quo";
            mock.setup()
                .accept_two_refs_return_ref(r1, r2)
                .returns(returned_r);

            // Act
            let actual_returned_r = mock.accept_two_refs_return_ref(r1, r2);

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            mock.received()
                .accept_two_refs_return_ref(r1, r2, Times::Once)
                .no_other_calls();
        }
    }
}
