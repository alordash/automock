use automock::*;

#[mock]
trait Trait {
    fn accept_ref(&self, r: &i32);

    fn return_ref(&self) -> &'static i32;

    fn accept_ref_return_ref(&self, r: &i32) -> &'static i32;

    fn accept_two_refs(&self, r1: &i32, r2: &f32);

    fn accept_two_refs_return_ref(&self, r1: &i32, r2: &f32) -> &'static str;
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
            let mut mock = TraitMock::new();
            let r = &1;
            mock.setup().accept_ref(r).expect_once();

            // Act
            mock.accept_ref(r);

            // Assert
            mock.verify_calls();
        }

        #[test]
        fn accept_ref_NotEnoughCalls_Panics() {
            // Arrange
            let mut mock = TraitMock::new();
            let r = &11;
            mock.setup().accept_ref(Arg::Any).expect(3.times());

            // Act
            mock.accept_ref(r);

            // Assert
            assert_panics(
                || mock.verify_calls(),
                "Expected following calls:
	Trait::accept_ref((&i32): any) 2 times",
            );
        }

        #[test]
        fn accept_ref_NotMatchingValue_Panics() {
            // Arrange
            let mut mock = TraitMock::new();
            let r = &11;
            let invalid_r = &22;
            mock.setup().accept_ref(invalid_r).expect_once();

            // Act
            let actual_error_msg = record_panic(|| mock.accept_ref(r));

            // Assert
            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call:
	Trait::accept_ref({r})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref(*{r}*)"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
        }

        #[test]
        fn accept_ref_NotMatchingRef_Panics() {
            // Arrange
            let mut mock = TraitMock::new();
            let r = &11;
            let invalid_r = &22;
            mock.setup().accept_ref(Arg::ref_eq(invalid_r)).expect_once();

            // Act
            let actual_error_msg = record_panic(|| mock.accept_ref(r));

            // Assert
            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call:
	Trait::accept_ref({r})
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
            let mut mock = TraitMock::new();
            let r = Box::leak(Box::new(11));
            mock.setup().return_ref().returns_once(r);

            // Act
            let actual_r = mock.return_ref();

            // Assert
            assert_eq!(r, actual_r);
            mock.verify_calls();
        }
    }

    mod accept_ref_return_ref_tests {
        use super::*;

        #[test]
        fn accept_ref_return_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let accepted_r = &10;
            let returned_r = &20;
            mock.setup()
                .accept_ref_return_ref(accepted_r)
                .returns_once(returned_r);

            // Act
            let actual_returned_r = mock.accept_ref_return_ref(accepted_r);

            // Assert
            assert_eq!(returned_r, actual_returned_r);
            mock.verify_calls();
        }
    }

    mod accept_two_refs_tests {
        use super::*;

        #[test]
        fn accept_two_refs_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let r1 = &10;
            let r2 = &20.2;
            mock.setup().accept_two_refs(r1, r2).expect_once();

            // Act
            mock.accept_two_refs(r1, r2);

            // Assert
            mock.verify_calls();
        }
    }

    mod accept_two_refs_return_ref_tests {
        use super::*;

        #[test]
        fn accept_two_refs_return_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let r1 = &10;
            let r2 = &20.2;
            let returned_r = "veridis quo";
            mock.setup()
                .accept_two_refs_return_ref(r1, r2)
                .returns_once(returned_r);

            // Act
            let actual_returned_r = mock.accept_two_refs_return_ref(r1, r2);

            // Assert
            assert_eq!(returned_r, actual_returned_r);
            mock.verify_calls();
        }
    }
}
