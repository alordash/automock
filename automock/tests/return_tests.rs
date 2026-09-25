use automock::*;

#[mock(base)]
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
        work::setup().returns(10);

        // Act
        let result = work();
        let panic_message = record_panic(work);

        // Assert
        assert_eq!(result, 10);
        assert_eq!(panic_message, Some(r#"Mock wasn't configured to handle following call because no return value was provided:
	work()"#.to_owned()));
        work::received(1.times()).no_other_calls();
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
        assert_eq!(panic_message, Some(r#"Mock wasn't configured to handle following call because no return value was provided:
	work()"#.to_owned()));
        work::received(3.times()).no_other_calls();
    }

    #[test]
    fn always_returns_AtLeastTwoTimes() {
        // Arrange
        work::setup().always_returns(10);

        // Act
        let first = work();
        let second = work();

        // Assert
        assert_eq!(first, 10);
        assert_eq!(second, 10);
        work::received(2.times()).no_other_calls();
    }

    #[test]
    fn returns_with_AtLeastTwoTimes() {
        // Arrange
        work::setup().returns_with(|_| 10);

        // Act
        let first = work();
        let second = work();

        // Assert
        assert_eq!(first, 10);
        assert_eq!(second, 10);
        work::received(2.times()).no_other_calls();
    }
}
