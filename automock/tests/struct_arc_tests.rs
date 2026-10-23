use automock::*;
use std::sync::Arc;

#[mock]
struct Struct {
    #[allow(unused)]
    pub number: i32,
}

#[mock]
impl Struct {
    #[allow(unused)]
    pub fn new(number: i32) -> Self {
        Self { number }
    }
}

#[mock]
#[allow(unused)]
impl Struct {
    pub(crate) fn accept_arc(&self, r: Arc<i32>) {}

    pub(crate) fn return_arc(&self) -> Arc<i32> {
        unreachable!()
    }

    pub(crate) fn accept_arc_return_arc(&self, r: Arc<i32>) -> Arc<i32> {
        unreachable!()
    }

    pub(crate) fn accept_two_arcs(&self, r1: Arc<i32>, r2: Arc<f32>) {}

    pub(crate) fn accept_two_arcs_return_arc(&self, r1: Arc<i32>, r2: Arc<f32>) -> Arc<String> {
        unreachable!()
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    mod accept_arc_tests {
        use super::*;

        #[test]
        fn accept_arc_Ok() {
            // Arrange
            Struct::static_setup().new(3).call_base();
            let mut mock = Struct::new(3);
            let r = Arc::new(1);
            mock.setup().accept_arc(r.clone()).expect_once();

            // Act
            mock.accept_arc(r.clone());

            // Assert
            mock.verify_calls();
        }

        #[test]
        fn accept_arc_Panics() {
            // Arrange
            Struct::static_setup().new(3).call_base();
            let mut mock = Struct::new(3);
            let r = Arc::new(11);
            let invalid_r = Arc::new(22);
            mock.setup()
                .accept_arc(Arg::ref_eq(invalid_r.clone()))
                .expect_once();

            // Act
            let actual_error_msg = record_panic(|| mock.accept_arc(r.clone()));

            // Assert
            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call:
	Struct::accept_arc({r})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_arc(*{r}*)"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
            let arc_type_name = std::any::type_name::<Arc<i32>>();
            assert_panics(
                || mock.verify_calls(),
                format!(
                    "Expected following calls:
	Struct::accept_arc({arc_type_name} == {invalid_r}) 1 time"
                ),
            );
        }
    }

    mod return_arc_tests {
        use super::*;

        #[test]
        fn return_arc_Ok() {
            // Arrange
            Struct::static_setup().new(3).call_base();
            let mut mock = Struct::new(3);
            let r = Arc::new(10);
            mock.setup().return_arc().returns_once(r.clone());

            // Act
            let actual_r = mock.return_arc();

            // Assert
            assert_eq!(r, actual_r);
            mock.verify_calls();
        }
    }

    mod accept_arc_return_arc_tests {
        use super::*;

        #[test]
        fn accept_arc_return_arc_Ok() {
            // Arrange
            Struct::static_setup().new(3).call_base();
            let mut mock = Struct::new(3);
            let accepted_r = Arc::new(10);
            let returned_r = Arc::new(20);
            mock.setup()
                .accept_arc_return_arc(accepted_r.clone())
                .returns_once(returned_r.clone());

            // Act
            let actual_returned_r = mock.accept_arc_return_arc(accepted_r.clone());

            // Assert
            assert_eq!(returned_r, actual_returned_r);
            mock.verify_calls();
        }
    }

    mod accept_two_arcs_tests {
        use super::*;

        #[test]
        fn accept_two_arcs_Ok() {
            // Arrange
            Struct::static_setup().new(3).call_base();
            let mut mock = Struct::new(3);
            let r1 = Arc::new(10);
            let r2 = Arc::new(20.2);
            mock.setup()
                .accept_two_arcs(r1.clone(), r2.clone())
                .expect_once();

            // Act
            mock.accept_two_arcs(r1.clone(), r2.clone());

            // Assert
            mock.verify_calls();
        }
    }

    mod accept_two_arcs_return_arc_tests {
        use super::*;

        #[test]
        fn accept_two_arcs_return_arc_Ok() {
            // Arrange
            Struct::static_setup().new(3).call_base();
            let mut mock = Struct::new(3);
            let r1 = Arc::new(10);
            let r2 = Arc::new(20.2);
            let returned_r = Arc::new(String::from("veridis quo"));
            mock.setup()
                .accept_two_arcs_return_arc(r1.clone(), r2.clone())
                .returns_once(returned_r.clone());

            // Act
            let actual_returned_r = mock.accept_two_arcs_return_arc(r1.clone(), r2.clone());

            // Assert
            assert_eq!(returned_r, actual_returned_r);
            mock.verify_calls();
        }
    }
}
