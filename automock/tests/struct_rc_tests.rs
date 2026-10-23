use automock::*;
use std::rc::Rc;

#[mock]
struct Struct;

#[mock]
impl Struct {
    pub fn new() -> Self {
        Self
    }
}

#[mock]
impl Struct {
    pub(crate) fn accept_rc(&self, _: Rc<i32>) {}

    pub(crate) fn return_rc(&self) -> Rc<i32> {
        unreachable!()
    }

    pub(crate) fn accept_rc_return_rc(&self, _: Rc<i32>) -> Rc<i32> {
        unreachable!()
    }

    pub(crate) fn accept_two_rcs(&self, _: Rc<i32>, _: Rc<f32>) {}

    pub(crate) fn accept_two_rcs_return_rc(&self, _: Rc<i32>, _: Rc<f32>) -> Rc<String> {
        unreachable!()
    }
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
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            let r = Rc::new(1);
            mock.setup().accept_rc(r.clone()).expect_once();

            // Act
            mock.accept_rc(r.clone());

            // Assert
            mock.verify_calls();
        }

        #[test]
        fn accept_rc_Panics() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            let r = Rc::new(11);
            let invalid_r = Rc::new(22);
            mock.setup()
                .accept_rc(Arg::ref_eq(invalid_r.clone()))
                .expect_once();

            // Act
            let actual_error_msg = record_panic(|| mock.accept_rc(r.clone()));

            // Assert
            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call:
	Struct::accept_rc({r})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_rc(*{r}*)"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
            let rc_type_name = std::any::type_name::<Rc<i32>>();
            assert_panics(
                || mock.verify_calls(),
                format!(
                    "Expected following calls:
	Struct::accept_rc({rc_type_name} == {invalid_r}) 1 time"
                ),
            );
        }
    }

    mod return_rc_tests {
        use super::*;

        #[test]
        fn return_rc_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            let r = Rc::new(10);
            mock.setup().return_rc().returns_once(r.clone());

            // Act
            let actual_r = mock.return_rc();

            // Assert
            assert_eq!(r, actual_r);
            mock.verify_calls();
        }
    }

    mod accept_rc_return_rc_tests {
        use super::*;

        #[test]
        fn accept_rc_return_rc_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            let accepted_r = Rc::new(10);
            let returned_r = Rc::new(20);
            mock.setup()
                .accept_rc_return_rc(accepted_r.clone())
                .returns_once(returned_r.clone());

            // Act
            let actual_returned_r = mock.accept_rc_return_rc(accepted_r.clone());

            // Assert
            assert_eq!(returned_r, actual_returned_r);
            mock.verify_calls();
        }
    }

    mod accept_two_rcs_tests {
        use super::*;

        #[test]
        fn accept_two_rcs_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            let r1 = Rc::new(10);
            let r2 = Rc::new(20.2);
            mock.setup()
                .accept_two_rcs(r1.clone(), r2.clone())
                .expect_once();

            // Act
            mock.accept_two_rcs(r1.clone(), r2.clone());

            // Assert
            mock.verify_calls();
        }
    }

    mod accept_two_rcs_return_rc_tests {
        use super::*;

        #[test]
        fn accept_two_rcs_return_rc_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            let r1 = Rc::new(10);
            let r2 = Rc::new(20.2);
            let returned_r = Rc::new(String::from("veridis quo"));
            mock.setup()
                .accept_two_rcs_return_rc(r1.clone(), r2.clone())
                .returns_once(returned_r.clone());

            // Act
            let actual_returned_r = mock.accept_two_rcs_return_rc(r1.clone(), r2.clone());

            // Assert
            assert_eq!(returned_r, actual_returned_r);
            mock.verify_calls();
        }
    }
}
