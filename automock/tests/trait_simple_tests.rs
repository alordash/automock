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
            mock.setup()
                .mutate()
                .expect_once()
                .and_does(move |_, _| flag_copy.set(true));
            mock.setup().consume().returns_once(33);
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
            .expect_once()
            .and_does(move |_, _| *callback_flag_clone.borrow_mut() = true);

        // Act
        mock.f();

        // Assert
        assert!(*callback_flag.borrow());
        mock.verify_calls();
    }

    #[test]
    fn f_NoConfig_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        mock.setup().f().expect_once();

        // Act
        mock.f();

        // Assert
        mock.verify_calls();
    }

    #[test]
    fn f_MultipleTimes_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        mock.setup().f().expect(3.times());

        // Act
        mock.f();
        mock.f();
        mock.f();

        // Assert
        mock.verify_calls();
    }

    #[test]
    fn f_TooManyCalls_Panics() {
        // Arrange
        let mut mock = TraitMock::new();
        mock.setup().f().expect(2.times());

        // Act
        mock.f();
        mock.f();

        // Assert
        assert_panics(
            || mock.f(),
            r#"Mock wasn't configured to handle following call:
	Trait::f()"#,
        );
        mock.verify_calls();
    }

    #[test]
    fn f_NotEnoughCalls_Panics() {
        // Arrange
        let mut mock = TraitMock::new();
        mock.setup().f().expect(4.times());

        // Act
        mock.f();
        mock.f();
        mock.f();

        // Assert
        assert_panics(
            || mock.verify_calls(),
            r#"Expected following calls:
	Trait::f() 1 time"#,
        );
    }
}
