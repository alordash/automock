use automock::*;

#[mock(base)]
fn accept_ref(_: &i32) {}

const BASE_RETURN_REF: &i32 = &1000;
#[mock(base)]
fn return_ref() -> &'static i32 {
    BASE_RETURN_REF
}

const BASE_ACCEPT_REF_RETURN_REF: &i32 = &2000;
#[mock(base)]
fn accept_ref_return_ref(_: &i32) -> &'static i32 {
    BASE_ACCEPT_REF_RETURN_REF
}

#[mock(base)]
fn accept_two_refs(_: &i32, _: &f32) {}

const ACCEPT_TWO_REFS_RETURN_REF: &str = "quo vadis";
#[mock(base)]
fn accept_two_refs_return_ref(_: &i32, _: &f32) -> &'static str {
    ACCEPT_TWO_REFS_RETURN_REF
}

#[mock(base)]
#[allow(unused)]
fn accept_mut_ref(r: &mut i32) {}

static mut BASE_RETURN_MUT_REF: i32 = 12;
#[mock(base)]
#[allow(unused)]
fn return_mut_ref() -> &'static mut i32 {
    unsafe { &mut *&raw mut BASE_RETURN_MUT_REF }
}

#[mock(base)]
#[allow(unused)]
fn accept_mut_ref_return_mut_ref(_: &mut i32) -> &'static i32 {
    BASE_ACCEPT_REF_RETURN_REF
}

#[mock(base)]
#[allow(unused)]
fn accept_two_mut_refs(_: &mut i32, _: &mut f32) {}

static mut ACCEPT_TWO_REFS_RETURN_MUT_REF: i32 = 382;
#[mock(base)]
#[allow(unused)]
fn accept_two_mut_refs_return_mut_ref(_: &mut i32, _: &mut f32) -> &'static mut i32 {
    unsafe { &mut *&raw mut ACCEPT_TWO_REFS_RETURN_MUT_REF }
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
            let r = &12;

            {
                let v2 = 24;
                let r2 = &v2;
                accept_ref::setup(r2).does(|(ref _unused,)| {});
                accept_ref(r2);
            }

            let _some_data_for_stack_filling =
                core::hint::black_box([15, 22, 32, 42, 52, 62, 72, 82, 92, 102]);

            // Act
            accept_ref(r);

            // Assert
            accept_ref::received(r, Times::Once)
                .received(
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_return)]
                    Arg::is(|ref _unused| {
                        return true;
                    }),
                    Times::Exactly(2),
                )
                .no_other_calls();
        }

        #[test]
        fn accept_value_Panics() {
            // Arrange
            let r = &11;
            let r_ptr = core::ptr::from_ref(r);

            // Act
            accept_ref(r);

            // Assert
            assert_panics(
                || accept_ref::received(Arg::Any, Times::Never),
                format!(
                    "Expected to never receive a call matching:
	accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
                ),
            );

            assert_panics(
                || accept_ref::received(Arg::Any, Times::Exactly(3)),
                format!(
                    "Expected to receive a call 3 times matching:
	accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
                ),
            );

            let invalid_r = &22;
            let invalid_r_ptr = core::ptr::from_ref(invalid_r);
            assert_panics(
                || accept_ref::received(Arg::ref_eq(invalid_r), Times::Once),
                format!(
                    "Expected to receive a call exactly once matching:
	accept_ref((&i32): equal to {invalid_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref(*{r}*)
	1. __arg0 (&i32):
		Expected (ptr: {invalid_r_ptr:?}): {invalid_r}
		Actual   (ptr: {r_ptr:?}): {r}"
                ),
            )
        }
    }

    mod return_ref_tests {
        use super::*;

        #[test]
        fn return_ref_Ok() {
            // Arrange
            let r = Box::leak(Box::new(11));
            return_ref::setup().returns(r);

            // Act
            let actual_r = return_ref();

            // Assert
            assert_eq!(r, actual_r);
        }

        #[test]
        fn return_ref_CallBase_Ok() {
            // Arrange
            return_ref::setup().call_base();

            // Act
            let actual_r = return_ref();

            // Assert
            assert_eq!(BASE_RETURN_REF, actual_r);
        }
    }

    mod accept_ref_return_ref_tests {
        use super::*;

        #[test]
        fn accept_ref_return_ref_Ok() {
            // Arrange
            let accepted_r = &10;
            let returned_r = &20;
            accept_ref_return_ref::setup(accepted_r).returns(returned_r);

            // Act
            let actual_returned_r = accept_ref_return_ref(accepted_r);

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            accept_ref_return_ref::received(accepted_r, Times::Once)
                .received(Arg::not_eq(accepted_r), Times::Never)
                .no_other_calls();
        }

        #[test]
        fn accept_ref_return_ref_CallBase_Ok() {
            // Arrange
            let accepted_r = &10;
            accept_ref_return_ref::setup(accepted_r).call_base();

            // Act
            let actual_returned_r = accept_ref_return_ref(accepted_r);

            // Assert
            assert_eq!(BASE_ACCEPT_REF_RETURN_REF, actual_returned_r);
            accept_ref_return_ref::received(accepted_r, Times::Once).no_other_calls();
        }
    }

    mod accept_two_refs_tests {
        use super::*;

        #[test]
        fn accept_two_refs_Ok() {
            // Arrange
            let r1 = &10;
            let r2 = &20.2;

            // Act
            accept_two_refs(r1, r2);

            // Assert
            accept_two_refs::received(r1, r2, Times::Once)
                .received(Arg::not_eq(r1), Arg::not_eq(r2), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_refs_return_ref_tests {
        use super::*;

        #[test]
        fn accept_two_refs_return_ref_Ok() {
            // Arrange
            let r1 = &10;
            let r2 = &20.2;
            let returned_r = "veridis quo";
            accept_two_refs_return_ref::setup(r1, r2).returns(returned_r);

            // Act
            let actual_returned_r = accept_two_refs_return_ref(r1, r2);

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            accept_two_refs_return_ref::received(r1, r2, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_two_refs_return_ref_CallBase_Ok() {
            // Arrange
            let r1 = &10;
            let r2 = &20.2;
            accept_two_refs_return_ref::setup(r1, r2).call_base();

            // Act
            let actual_returned_r = accept_two_refs_return_ref(r1, r2);

            // Assert
            assert_eq!(ACCEPT_TWO_REFS_RETURN_REF, actual_returned_r);

            accept_two_refs_return_ref::received(r1, r2, Times::Once).no_other_calls();
        }
    }
}
