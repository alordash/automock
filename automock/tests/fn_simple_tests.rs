#![allow(clippy::arc_with_non_send_sync)]
use automock::*;
use std::cell::RefCell;
use std::sync::Arc;

#[mock]
fn f() {}

#[mock]
fn f_base() {}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    mod simple {
        use super::*;

        #[test]
        fn f_Ok() {
            // Arrange
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            f::setup()
                .expect_once()
                .and_does(move |_| *callback_flag_clone.borrow_mut() = true);

            // Act
            f();

            // Assert
            assert!(*callback_flag.borrow());
            f::verify_calls();
        }

        #[test]
        fn f_NoConfig_Ok() {
            // Arrange
            f::setup().expect_once();

            // Act
            f();

            // Assert
            f::verify_calls();
        }

        #[test]
        fn f_MultipleTime_Ok() {
            // Arrange
            f::setup().expect(3.times());

            // Act
            f();
            f();
            f();

            // Assert
            f::verify_calls();
        }

        #[test]
        fn f_TooManyCalls_Panics() {
            // Arrange
            f::setup().expect(2.times());

            // Act
            f();
            f();

            // Assert
            assert_panics(
                f,
                r#"Mock wasn't configured to handle following call:
	f()"#,
            );
            f::verify_calls();
        }

        #[test]
        fn f_NotEnoughCalls_Panics() {
            // Arrange
            f::setup().expect(4.times());

            // Act
            f();
            f();
            f();

            // Assert
            assert_panics(
                f::verify_calls,
                r#"Expected following calls:
	f() 1 time"#,
            );
        }
    }

    mod base {
        use super::*;

        #[test]
        fn f_base_Ok() {
            // Arrange
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            f_base::setup()
                .expect_once()
                .and_does(move |_| *callback_flag_clone.borrow_mut() = true);

            // Act
            f_base();

            // Assert
            assert!(*callback_flag.borrow());
            f_base::verify_calls();
        }

        #[test]
        fn f_base_NoConfig_Ok() {
            // Arrange
            f_base::setup().expect_once();

            // Act
            f_base();

            // Assert
            f_base::verify_calls();
        }

        #[test]
        fn f_base_MultipleTime_Ok() {
            // Arrange
            f_base::setup().expect(3.times());

            // Act
            f_base();
            f_base();
            f_base();

            // Assert
            f_base::verify_calls();
        }

        #[test]
        fn f_base_TooManyCalls_Panics() {
            // Arrange
            f_base::setup().expect(2.times());

            // Act
            f_base();
            f_base();

            // Assert
            assert_panics(
                f_base,
                r#"Mock wasn't configured to handle following call:
	f_base()"#,
            );
            f_base::verify_calls();
        }

        #[test]
        fn f_base_NotEnoughCalls_Panics() {
            // Arrange
            f_base::setup().expect(4.times());

            // Act
            f_base();
            f_base();
            f_base();

            // Assert
            assert_panics(
                f_base::verify_calls,
                r#"Expected following calls:
	f_base() 1 time"#,
            );
        }
    }
}
