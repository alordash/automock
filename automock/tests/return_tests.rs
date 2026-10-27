use automock::*;

#[mock]
fn work() -> i32 {
    1
}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::*;

    #[test]
    fn returns_SingleTime() {
        // Arrange
        work::setup().returns_once(10);

        // Act
        let result = work();
        let panic_message = record_panic(work);

        // Assert
        assert_eq!(result, 10);
        assert_eq!(
            panic_message,
            Some(
                r#"Mock wasn't configured to handle following call:
	work()"#
                    .to_owned()
            )
        );
        work::verify_calls();
    }

    #[test]
    fn returns_many_MultipleTimes() {
        // Arrange
        work::setup().returns_many([10, 20, 30]);

        // Act
        let first = work();
        let second = work();
        let third = work();
        let panic_message = record_panic(work);

        // Assert
        assert_eq!(first, 10);
        assert_eq!(second, 20);
        assert_eq!(third, 30);
        assert_eq!(
            panic_message,
            Some(
                r#"Mock wasn't configured to handle following call:
	work()"#
                    .to_owned()
            )
        );
        work::verify_calls();
    }

    #[test]
    fn returns_once_SameArgsTwoConfigs_UsesNextConfigAfterFirstIsExhausted() {
        // Arrange
        work::setup().returns_once(10).setup().returns_once(20);

        // Act
        let first = work();
        let second = work();

        // Assert
        assert_eq!(first, 10);
        assert_eq!(second, 20);
        work::verify_calls();
    }

    #[test]
    fn returns_MultipleTimes() {
        // Arrange
        work::setup().returns(10, 2.times());

        // Act
        let first = work();
        let second = work();

        // Assert
        assert_eq!(first, 10);
        assert_eq!(second, 10);
        work::verify_calls();
    }

    #[test]
    fn returns_with_MultipleTimes() {
        // Arrange
        work::setup().returns_with(|_| 10, 2.times());

        // Act
        let first = work();
        let second = work();

        // Assert
        assert_eq!(first, 10);
        assert_eq!(second, 10);
        work::verify_calls();
    }
}
