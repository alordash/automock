#![allow(clippy::arc_with_non_send_sync)]
use automock::*;

#[mock]
trait Trait {
    fn f(&self);

    #[allow(unused)]
    fn mutate(&mut self);

    #[allow(unused)]
    fn consume(self) -> i32;
}

#[mock]
#[allow(unused)]
trait AnotherTestTrait {}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::*;
    use std::cell::{Cell, RefCell};
    use std::sync::Arc;

    #[test]
    fn no_support() {
        // Arrange
        let flag = Arc::new(Cell::new(false));
        let flag_copy = flag.clone();
        let mut mock = TraitMock::new();
        {
            mock.setup().mutate().does(move |_, _| flag_copy.set(true));
            mock.setup().consume().returns(33);
        }

        mock.mutate();
        assert!(flag.get());

        let v = mock.consume();
        assert_eq!(33, v);
    }

    #[test]
    fn f_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        let callback_flag = Arc::new(RefCell::new(false));
        let callback_flag_clone = callback_flag.clone();
        mock.setup()
            .f()
            .does(move |_, _| *callback_flag_clone.borrow_mut() = true);

        // Act
        mock.f();

        // Assert
        assert!(*callback_flag.borrow());
        mock.received().f(Times::Once).no_other_calls();
    }

    #[test]
    fn f_NoConfig_Ok() {
        // Arrange
        let mut mock = TraitMock::new();

        // Act
        mock.f();

        // Assert
        mock.received().f(Times::Once).no_other_calls();
    }

    #[test]
    fn f_MultipleTimes_Ok() {
        // Arrange
        let mut mock = TraitMock::new();

        // Act
        mock.f();
        mock.f();
        mock.f();

        // Assert
        mock.received().f(Times::Exactly(3)).no_other_calls();
    }

    #[test]
    fn f_MultipleTimes_Panics() {
        // Arrange
        let mut mock = TraitMock::new();

        // Act
        mock.f();
        mock.f();
        mock.f();

        // Assert
        assert_panics(
            || mock.received().f(Times::Once),
            r#"Expected to receive a call exactly once matching:
	Trait::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || mock.received().f(Times::Exactly(1)),
            r#"Expected to receive a call exactly once matching:
	Trait::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || mock.received().f(Times::Exactly(2)),
            r#"Expected to receive a call 2 times matching:
	Trait::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || mock.received().f(Times::Exactly(4)),
            r#"Expected to receive a call 4 times matching:
	Trait::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );
    }
}
