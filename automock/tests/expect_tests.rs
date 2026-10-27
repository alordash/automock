use automock::*;
use std::cell::Cell;
use std::rc::Rc;

#[mock]
fn notify(_v: i32) {}

#[mock]
trait Trait {
    fn notify(&self, v: i32);

    fn static_notify(v: i32);
}

#[mock]
struct Struct;

#[mock]
impl Struct {
    pub fn new() -> Self {
        Self
    }

    fn notify(&self, _v: i32) {}
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    mod r#fn {
        use super::*;

        #[test]
        fn expect_Once_Ok() {
            // Arrange
            let v = 10;
            notify::setup(v).expect_once();

            // Act
            notify(v);

            // Assert
            notify::verify_calls();
        }

        #[test]
        fn expect_MultipleTimes_Ok() {
            // Arrange
            let v = 10;
            notify::setup(v).expect(3.times());

            // Act
            notify(v);
            notify(v);
            notify(v);

            // Assert
            notify::verify_calls();
        }

        #[test]
        fn expect_DifferentArgs_Ok() {
            // Arrange
            let (v1, v2) = (10, 20);
            notify::setup(v1).expect_once().setup(v2).expect(2.times());

            // Act
            notify(v2);
            notify(v1);
            notify(v2);

            // Assert
            notify::verify_calls();
        }

        #[test]
        fn expect_AndDoes_CallsCallback() {
            // Arrange
            let v = 10;
            let callback_flag = Rc::new(Cell::new(false));
            let callback_flag_clone = callback_flag.clone();
            notify::setup(v)
                .expect_once()
                .and_does(move |_| callback_flag_clone.set(true));

            // Act
            notify(v);

            // Assert
            assert!(callback_flag.get());
            notify::verify_calls();
        }

        #[test]
        fn expect_NotEnoughCalls_Panics() {
            // Arrange
            let v = 10;
            notify::setup(v).expect(3.times());

            // Act
            notify(v);
            let actual_error_msg = record_panic(notify::verify_calls);

            // Assert
            let expected_error_msg = format!(
                "Expected following calls:
	notify({v}) 2 times"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
        }

        #[test]
        fn expect_TooManyCalls_Panics() {
            // Arrange
            let v = 10;
            notify::setup(v).expect_once();

            // Act
            notify(v);
            let actual_error_msg = record_panic(|| notify(v));

            // Assert
            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call:
	notify({v})"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
            notify::verify_calls();
        }
    }

    mod r#trait {
        use super::*;

        #[test]
        fn expect_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let v = 10;
            mock.setup().notify(v).expect(2.times());

            // Act
            mock.notify(v);
            mock.notify(v);

            // Assert
            mock.verify_calls();
        }

        #[test]
        fn static_expect_Ok() {
            // Arrange
            let v = 10;
            TraitMock::static_setup().static_notify(v).expect(2.times());

            // Act
            TraitMock::static_notify(v);
            TraitMock::static_notify(v);

            // Assert
            TraitMock::verify_static_calls();
        }

        #[test]
        fn expect_NotEnoughCalls_Panics() {
            // Arrange
            let mut mock = TraitMock::new();
            let v = 10;
            mock.setup().notify(v).expect(2.times());

            // Act
            mock.notify(v);
            let actual_error_msg = record_panic(|| mock.verify_calls());

            // Assert
            let expected_error_msg = format!(
                "Expected following calls:
	Trait::notify({v}) 1 time"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
        }

        #[test]
        fn expect_TooManyCalls_Panics() {
            // Arrange
            let mut mock = TraitMock::new();
            let v = 10;
            mock.setup().notify(v).expect_once();

            // Act
            mock.notify(v);
            let actual_error_msg = record_panic(|| mock.notify(v));

            // Assert
            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call:
	Trait::notify({v})"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
            mock.verify_calls();
        }
    }

    mod r#struct {
        use super::*;

        #[test]
        fn expect_Ok() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            let v = 10;
            mock.setup().notify(v).expect(2.times());

            // Act
            mock.notify(v);
            mock.notify(v);

            // Assert
            mock.verify_calls();
        }

        #[test]
        fn expect_NotEnoughCalls_Panics() {
            // Arrange
            Struct::static_setup().new().call_base();
            let mut mock = Struct::new();
            let v = 10;
            mock.setup().notify(v).expect(2.times());

            // Act
            mock.notify(v);
            let actual_error_msg = record_panic(|| mock.verify_calls());

            // Assert
            let expected_error_msg = format!(
                "Expected following calls:
	Struct::notify({v}) 1 time"
            );
            assert_eq!(Some(expected_error_msg), actual_error_msg);
        }
    }
}
