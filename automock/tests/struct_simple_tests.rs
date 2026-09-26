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

#[mock(base)]
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
            Struct::<i32>::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(value);
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            mock.setup()
                .f()
                .does(move |_, _| *callback_flag_clone.borrow_mut() = true);

            // Act
            Struct::non_associative();
            mock.f();

            // Assert
            assert_eq!(value, mock.value);
            assert!(*callback_flag.borrow());
            mock.received().f(Times::Once).no_other_calls();
        }

        #[test]
        fn f_NoConfig_Ok() {
            // Arrange
            Struct::<i32>::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(1);

            // Act
            mock.f();

            // Assert
            mock.received().f(Times::Once).no_other_calls();
        }

        #[test]
        fn f_MultipleTimes_Ok() {
            // Arrange
            Struct::<i32>::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(1);

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
            Struct::<i32>::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(1);

            // Act
            mock.f();
            mock.f();
            mock.f();

            // Assert
            assert_panics(
                || mock.received().f(Times::Once),
                r#"Expected to receive a call exactly once matching:
	Struct::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );

            assert_panics(
                || mock.received().f(Times::Exactly(1)),
                r#"Expected to receive a call exactly once matching:
	Struct::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );

            assert_panics(
                || mock.received().f(Times::Exactly(2)),
                r#"Expected to receive a call 2 times matching:
	Struct::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );

            assert_panics(
                || mock.received().f(Times::Exactly(4)),
                r#"Expected to receive a call 4 times matching:
	Struct::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );
        }
    }

    mod trait_tests {
        use super::*;

        #[test]
        fn Trait_f_Ok() {
            // Arrange
            let value = 22;
            Struct::<i32>::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(value);
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            mock.setup()
                .as_Trait()
                .f()
                .does(move |_, _| *callback_flag_clone.borrow_mut() = true);

            // Act
            Struct::non_associative();
            Trait::f(&mock);

            // Assert
            assert_eq!(value, mock.value);
            assert!(*callback_flag.borrow());
            mock.received().as_Trait().f(Times::Once);
            mock.received().no_other_calls()
        }

        #[test]
        fn Trait_f_NoConfig_Ok() {
            // Arrange
            Struct::<i32>::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(1);

            // Act
            Trait::f(&mock);

            // Assert
            mock.received().as_Trait().f(Times::Once);
            mock.received().no_other_calls();
        }

        #[test]
        fn Trait_f_MultipleTimes_Ok() {
            // Arrange
            Struct::<i32>::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(1);

            // Act
            Trait::f(&mock);
            Trait::f(&mock);
            Trait::f(&mock);

            // Assert
            mock.received().as_Trait().f(Times::Exactly(3));
            mock.received().no_other_calls();
        }

        #[test]
        fn Trait_f_MultipleTimes_Panics() {
            // Arrange
            Struct::<i32>::static_setup().new(Arg::Any).call_base();
            let mut mock = Struct::new(1);

            // Act
            Trait::f(&mock);
            Trait::f(&mock);
            Trait::f(&mock);

            // Assert
            assert_panics(
                || mock.received().as_Trait().f(Times::Once),
                r#"Expected to receive a call exactly once matching:
	<Struct as Trait>::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );

            assert_panics(
                || mock.received().as_Trait().f(Times::Exactly(1)),
                r#"Expected to receive a call exactly once matching:
	<Struct as Trait>::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );

            assert_panics(
                || mock.received().as_Trait().f(Times::Exactly(2)),
                r#"Expected to receive a call 2 times matching:
	<Struct as Trait>::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );

            assert_panics(
                || mock.received().as_Trait().f(Times::Exactly(4)),
                r#"Expected to receive a call 4 times matching:
	<Struct as Trait>::f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
            );
        }
    }
}
