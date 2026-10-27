use automock::*;

#[mock]
fn accept_ref(_: &i32) {}

const BASE_RETURN_REF: &i32 = &1000;
#[mock]
fn return_ref() -> &'static i32 {
    BASE_RETURN_REF
}

const BASE_ACCEPT_REF_RETURN_REF: &i32 = &2000;
#[mock]
fn accept_ref_return_ref(_: &i32) -> &'static i32 {
    BASE_ACCEPT_REF_RETURN_REF
}

#[mock]
fn accept_two_refs(_: &i32, _: &f32) {}

const ACCEPT_TWO_REFS_RETURN_REF: &str = "quo vadis";
#[mock]
fn accept_two_refs_return_ref(_: &i32, _: &f32) -> &'static str {
    ACCEPT_TWO_REFS_RETURN_REF
}

#[mock]
#[allow(unused)]
fn accept_mut_ref(r: &mut i32) {}

static mut BASE_RETURN_MUT_REF: i32 = 12;
#[mock]
#[allow(unused)]
fn return_mut_ref() -> &'static mut i32 {
    unsafe { &mut *&raw mut BASE_RETURN_MUT_REF }
}

#[mock]
#[allow(unused)]
fn accept_mut_ref_return_mut_ref(_: &mut i32) -> &'static i32 {
    BASE_ACCEPT_REF_RETURN_REF
}

#[mock]
#[allow(unused)]
fn accept_two_mut_refs(_: &mut i32, _: &mut f32) {}

static mut ACCEPT_TWO_REFS_RETURN_MUT_REF: i32 = 382;
#[mock]
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
                accept_ref::setup(r2)
                    .expect_once()
                    .and_does(|(ref _unused,)| {})
                    .setup(r)
                    .expect_once();
                accept_ref(r2);
            }

            let _some_data_for_stack_filling =
                core::hint::black_box([15, 22, 32, 42, 52, 62, 72, 82, 92, 102]);

            // Act
            accept_ref(r);

            // Assert
            accept_ref::verify_calls();
        }

        #[test]
        fn accept_ref_NotEnoughCalls_Panics() {
            // Arrange
            let r = &11;
            accept_ref::setup(Arg::Any).expect(3.times());

            // Act
            accept_ref(r);

            // Assert
            assert_panics(
                accept_ref::verify_calls,
                "Expected following calls:
	accept_ref((&i32): any) 2 times",
            );
        }

        #[test]
        fn accept_ref_NotMatchingRef_Panics() {
            // Arrange
            let r = &11;
            let invalid_r = &22;
            accept_ref::setup(Arg::ref_eq(invalid_r)).expect_once();

            // Act
            let actual_error_msg = record_panic(|| accept_ref(r));

            // Assert
            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call:
	accept_ref({r})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref(*{r}*)"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
        }
    }

    mod return_ref_tests {
        use super::*;

        #[test]
        fn return_ref_Ok() {
            // Arrange
            let r = Box::leak(Box::new(11));
            return_ref::setup().returns_once(r);

            // Act
            let actual_r = return_ref();

            // Assert
            assert_eq!(r, actual_r);
            return_ref::verify_calls();
        }

        #[test]
        fn return_ref_CallBase_Ok() {
            // Arrange
            return_ref::setup().call_base();

            // Act
            let actual_r = return_ref();

            // Assert
            assert_eq!(BASE_RETURN_REF, actual_r);
            return_ref::verify_calls();
        }
    }

    mod accept_ref_return_ref_tests {
        use super::*;

        #[test]
        fn accept_ref_return_ref_Ok() {
            // Arrange
            let accepted_r = &10;
            let returned_r = &20;
            accept_ref_return_ref::setup(accepted_r).returns_once(returned_r);

            // Act
            let actual_returned_r = accept_ref_return_ref(accepted_r);

            // Assert
            assert_eq!(returned_r, actual_returned_r);
            accept_ref_return_ref::verify_calls();
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
            accept_ref_return_ref::verify_calls();
        }
    }

    mod accept_two_refs_tests {
        use super::*;

        #[test]
        fn accept_two_refs_Ok() {
            // Arrange
            let r1 = &10;
            let r2 = &20.2;

            accept_two_refs::setup(r1, r2).expect_once();

            // Act
            accept_two_refs(r1, r2);

            // Assert
            accept_two_refs::verify_calls();
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
            accept_two_refs_return_ref::setup(r1, r2).returns_once(returned_r);

            // Act
            let actual_returned_r = accept_two_refs_return_ref(r1, r2);

            // Assert
            assert_eq!(returned_r, actual_returned_r);
            accept_two_refs_return_ref::verify_calls();
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
            accept_two_refs_return_ref::verify_calls();
        }
    }
}
