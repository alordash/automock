#![allow(clippy::arc_with_non_send_sync)]
use automock::*;
use std::cell::RefCell;
use std::sync::Arc;

#[mock]
fn f() {}

#[mock(base)]
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
            f::setup().does(move |_| *callback_flag_clone.borrow_mut() = true);

            // Act
            f();

            // Assert
            assert!(*callback_flag.borrow());
            f::received(Times::Once).no_other_calls();
        }

        #[test]
        fn f_NoConfig_Ok() {
            // Arrange
            f::setup();

            // Act
            f();

            // Assert
            f::received(Times::Once).no_other_calls();
        }

        #[test]
        fn f_MultipleTime_Ok() {
            // Arrange
            f::setup();

            // Act
            f();
            f();
            f();

            // Assert
            f::received(Times::Exactly(3)).no_other_calls();
        }

        #[test]
        fn f_MultipleTimes_Panics() {
            // Arrange
            f::setup();

            // Act
            f();
            f();
            f();

            // Assert
            assert_panics(
                || f::received(Times::Once),
                r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );

            assert_panics(
                || f::received(Times::Exactly(1)),
                r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );

            assert_panics(
                || f::received(Times::Exactly(2)),
                r#"Expected to receive a call 2 times matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );

            assert_panics(
                || f::received(Times::Exactly(4)),
                r#"Expected to receive a call 4 times matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
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
            f_base::setup().does(move |_| *callback_flag_clone.borrow_mut() = true);

            // Act
            f_base();

            // Assert
            assert!(*callback_flag.borrow());
            f_base::received(Times::Once).no_other_calls();
        }

        #[test]
        fn f_base_NoConfig_Ok() {
            // Arrange
            f_base::setup();

            // Act
            f_base();

            // Assert
            f_base::received(Times::Once).no_other_calls();
        }

        #[test]
        fn f_base_MultipleTime_Ok() {
            // Arrange
            f_base::setup();

            // Act
            f_base();
            f_base();
            f_base();

            // Assert
            f_base::received(Times::Exactly(3)).no_other_calls();
        }

        #[test]
        fn f_base_MultipleTimes_Panics() {
            // Arrange
            f_base::setup();

            // Act
            f_base();
            f_base();
            f_base();

            // Assert
            assert_panics(
                || f_base::received(Times::Once),
                r#"Expected to receive a call exactly once matching:
	f_base()
Actually received 3 matching calls:
	f_base()
	f_base()
	f_base()
Received no non-matching calls"#,
            );

            assert_panics(
                || f_base::received(Times::Exactly(1)),
                r#"Expected to receive a call exactly once matching:
	f_base()
Actually received 3 matching calls:
	f_base()
	f_base()
	f_base()
Received no non-matching calls"#,
            );

            assert_panics(
                || f_base::received(Times::Exactly(2)),
                r#"Expected to receive a call 2 times matching:
	f_base()
Actually received 3 matching calls:
	f_base()
	f_base()
	f_base()
Received no non-matching calls"#,
            );

            assert_panics(
                || f_base::received(Times::Exactly(4)),
                r#"Expected to receive a call 4 times matching:
	f_base()
Actually received 3 matching calls:
	f_base()
	f_base()
	f_base()
Received no non-matching calls"#,
            );
        }
    }
}
