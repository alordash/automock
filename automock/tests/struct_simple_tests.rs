#![allow(clippy::arc_with_non_send_sync)]
use automock::*;
use std::marker::PhantomData;

trait Trait {
    fn f(&self);
}

#[mock]
struct Struct<Q = i32> {
    pub phantom: core::marker::PhantomData<Q>,
    pub value: i32,
}

#[mock]
impl<Q> Struct<Q> {
    pub fn new(value: i32) -> Self {
        Self {
            phantom: PhantomData,
            value,
        }
    }
}

#[mock]
impl Struct {
    pub fn f(&self) {}
}

#[mock]
impl Trait for Struct {
    fn f(&self) {}
}

impl Struct {
    pub fn non_associative() {}
}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::*;
    use std::cell::RefCell;
    use std::sync::Arc;

    mod struct_tests {
        use super::*;

        #[test]
        fn f_Ok() {
            // Arrange
            let value = 22;
            Struct::<i32>::static_setup().new(value).call_base();
            let mut mock = Struct::new(value);
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            mock.setup()
                .f()
                .expect_once()
                .and_does(move |_, _| *callback_flag_clone.borrow_mut() = true);

            // Act
            Struct::non_associative();
            mock.f();

            // Assert
            assert_eq!(value, mock.value);
            assert!(*callback_flag.borrow());
            mock.verify_calls();
        }

        #[test]
        fn f_NoConfig_Ok() {
            // Arrange
            Struct::<i32>::static_setup().new(1).call_base();
            let mut mock = Struct::new(1);
            mock.setup().f().expect_once();

            // Act
            mock.f();

            // Assert
            mock.verify_calls();
        }

        #[test]
        fn f_MultipleTimes_Ok() {
            // Arrange
            Struct::<i32>::static_setup().new(1).call_base();
            let mut mock = Struct::new(1);
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
            Struct::<i32>::static_setup().new(1).call_base();
            let mut mock = Struct::new(1);
            mock.setup().f().expect(2.times());

            // Act
            mock.f();
            mock.f();

            // Assert
            assert_panics(
                || mock.f(),
                r#"Mock wasn't configured to handle following call:
	Struct<i32>::f()"#,
            );
            mock.verify_calls();
        }

        #[test]
        fn f_NotEnoughCalls_Panics() {
            // Arrange
            Struct::<i32>::static_setup().new(1).call_base();
            let mut mock = Struct::new(1);
            mock.setup().f().expect(4.times());

            // Act
            mock.f();
            mock.f();
            mock.f();

            // Assert
            assert_panics(
                || mock.verify_calls(),
                r#"Expected following calls:
	Struct<i32>::f() 1 time"#,
            );
        }
    }

    mod trait_tests {
        use super::*;

        #[test]
        fn Trait_f_Ok() {
            // Arrange
            let value = 22;
            Struct::<i32>::static_setup().new(value).call_base();
            let mut mock = Struct::new(value);
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            mock.setup()
                .as_Trait()
                .f()
                .expect_once()
                .and_does(move |_, _| *callback_flag_clone.borrow_mut() = true);

            // Act
            Struct::non_associative();
            Trait::f(&mock);

            // Assert
            assert_eq!(value, mock.value);
            assert!(*callback_flag.borrow());
            mock.verify_calls();
        }

        #[test]
        fn Trait_f_NoConfig_Ok() {
            // Arrange
            Struct::<i32>::static_setup().new(1).call_base();
            let mut mock = Struct::new(1);
            mock.setup().as_Trait().f().expect_once();

            // Act
            Trait::f(&mock);

            // Assert
            mock.verify_calls();
        }

        #[test]
        fn Trait_f_MultipleTimes_Ok() {
            // Arrange
            Struct::<i32>::static_setup().new(1).call_base();
            let mut mock = Struct::new(1);
            mock.setup().as_Trait().f().expect(3.times());

            // Act
            Trait::f(&mock);
            Trait::f(&mock);
            Trait::f(&mock);

            // Assert
            mock.verify_calls();
        }

        #[test]
        fn Trait_f_TooManyCalls_Panics() {
            // Arrange
            Struct::<i32>::static_setup().new(1).call_base();
            let mut mock = Struct::new(1);
            mock.setup().as_Trait().f().expect(2.times());

            // Act
            Trait::f(&mock);
            Trait::f(&mock);

            // Assert
            assert_panics(
                || Trait::f(&mock),
                r#"Mock wasn't configured to handle following call:
	<Struct<i32> as Trait>::f()"#,
            );
            mock.verify_calls();
        }

        #[test]
        fn Trait_f_NotEnoughCalls_Panics() {
            // Arrange
            Struct::<i32>::static_setup().new(1).call_base();
            let mut mock = Struct::new(1);
            mock.setup().as_Trait().f().expect(4.times());

            // Act
            Trait::f(&mock);
            Trait::f(&mock);
            Trait::f(&mock);

            // Assert
            assert_panics(
                || mock.verify_calls(),
                r#"Expected following calls:
	<Struct<i32> as Trait>::f() 1 time"#,
            );
        }
    }
}
