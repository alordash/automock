use automock::*;
use std::rc::Rc;

#[mock]
trait Trait {
    fn accept_rc(&self, r: Rc<i32>);

    fn return_rc(&self) -> Rc<i32>;

    fn accept_rc_return_rc(&self, r: Rc<i32>) -> Rc<i32>;

    fn accept_two_rcs(&self, r1: Rc<i32>, r2: Rc<f32>);

    fn accept_two_rcs_return_rc(&self, r1: Rc<i32>, r2: Rc<f32>) -> Rc<String>;
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    mod accept_rc_tests {
        use super::*;

        #[test]
        fn accept_rc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let r = Rc::new(1);

            // Act
            mock.accept_rc(r.clone());

            // Assert
            mock.received().accept_rc(r, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_rc_Panics() {
            // Arrange
            let mut mock = TraitMock::new();
            let r = Rc::new(11);
            let r_ptr = Rc::as_ptr(&r);

            // Act
            mock.accept_rc(r.clone());

            // Assert
            assert_panics(
                || mock.received().accept_rc(Arg::Any, Times::Never),
                format!(
                    "Expected to never receive a call matching:
	Trait::accept_rc((alloc::rc::Rc<i32>): any)
Actually received 1 matching call:
	accept_rc({r})
Received no non-matching calls"
                ),
            );

            assert_panics(
                || mock.received().accept_rc(Arg::Any, Times::Exactly(3)),
                format!(
                    "Expected to receive a call 3 times matching:
	Trait::accept_rc((alloc::rc::Rc<i32>): any)
Actually received 1 matching call:
	accept_rc({r})
Received no non-matching calls"
                ),
            );

            let invalid_r = Rc::new(11);
            let invalid_r_ptr = Rc::as_ptr(&invalid_r);
            assert_panics(
                || {
                    mock.received()
                        .accept_rc(Arg::ref_eq(invalid_r.clone()), Times::Once)
                },
                format!(
                    "Expected to receive a call exactly once matching:
	Trait::accept_rc((alloc::rc::Rc<i32>): equal to {invalid_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_rc(*{r}*)
	1. r (alloc::rc::Rc<i32>):
		Expected (ptr: {invalid_r_ptr:?}): {invalid_r}
		Actual   (ptr: {r_ptr:?}): {r}"
                ),
            )
        }
    }

    mod return_rc_tests {
        use super::*;

        #[test]
        fn return_rc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let r = Rc::new(10);
            mock.setup().return_rc().returns(r.clone());

            // Act
            let actual_r = mock.return_rc();

            // Assert
            assert_eq!(r, actual_r);
        }
    }

    mod accept_rc_return_rc_tests {
        use super::*;

        #[test]
        fn accept_rc_return_rc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let accepted_r = Rc::new(10);
            let returned_r = Rc::new(20);
            mock.setup()
                .accept_rc_return_rc(accepted_r.clone())
                .returns(returned_r.clone());

            // Act
            let actual_returned_r = mock.accept_rc_return_rc(accepted_r.clone());

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            mock.received()
                .accept_rc_return_rc(accepted_r.clone(), Times::Once)
                .accept_rc_return_rc(Arg::not_eq(accepted_r), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_rcs_tests {
        use super::*;

        #[test]
        fn accept_two_rcs_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let r1 = Rc::new(10);
            let r2 = Rc::new(20.2);

            // Act
            mock.accept_two_rcs(r1.clone(), r2.clone());

            // Assert
            mock.received()
                .accept_two_rcs(r1.clone(), r2.clone(), Times::Once)
                .accept_two_rcs(Arg::not_eq(r1), Arg::not_eq(r2), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_rcs_return_rc_tests {
        use super::*;

        #[test]
        fn accept_two_rcs_return_rc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let r1 = Rc::new(10);
            let r2 = Rc::new(20.2);
            let returned_r = Rc::new(String::from("veridis quo"));
            mock.setup()
                .accept_two_rcs_return_rc(r1.clone(), r2.clone())
                .returns(returned_r.clone());

            // Act
            let actual_returned_r = mock.accept_two_rcs_return_rc(r1.clone(), r2.clone());

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            mock.received()
                .accept_two_rcs_return_rc(r1, r2, Times::Once)
                .no_other_calls();
        }
    }
}
