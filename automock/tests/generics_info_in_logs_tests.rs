#![allow(clippy::needless_lifetimes)]
use automock::*;

mod common;

#[mock]
trait Trait<'a, T1, const B: bool> {
    fn work<T2, 'b, const N: usize>(&self, v: &'b T1) -> T2;

    fn static_work<'b, T2, const N: usize>(v: &'b T1) -> T2;
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    #[test]
    fn work_NoConfigs_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 5>(&14));

        // Assert
        let actual_debug_string = common::debug_string("14");
        let expected_panic_msg = format!(
            "Mock wasn't configured to handle following call:
	Trait<i32, true>::work<f32, 5>({actual_debug_string})"
        );
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn work_OnlyUnsuitableConfigs_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let value = 5;
        mock.setup().work::<f32, 1>(&2).returns_once(3.0f32);
        mock.setup().work::<f32, 10>(&value).returns_once(3.0f32);
        mock.setup()
            .work::<[u8; 4], 10>(&value)
            .returns_once([1, 2, 3, 4]);
        mock.setup().work::<[u8; 4], 10>(&2).returns_once([1, 2, 3, 4]);

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 1>(&value));

        // Assert
        let actual_debug_string = common::debug_string("5");
        let expected_panic_msg = format!("Mock wasn't configured to handle following call:
	Trait<i32, true>::work<f32, 1>({actual_debug_string})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: work(*{actual_debug_string}*)");
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn work_NoReturnValue_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup().work::<f32, 1>(&value);
        mock.setup().work::<f32, 1>(&value).returns_once(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);
        mock.verify_calls();
    }

    #[test]
    fn work_DidNotReceiveSameGenerics_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        const N: usize = 1;
        mock.setup()
            .work::<f32, N>(&value)
            .returns(returned_value, 2.times());

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&value);
        let panic_msg = record_panic(|| mock.verify_calls());

        // Assert
        assert_eq!(returned_value, actual_returned_value);
        let arg_debug_string = common::debug_string("&i32 == 5");
        let expected_panic_msg = format!(
            "Expected following calls:
	Trait<i32, true>::work<f32, {N}>({arg_debug_string}) 1 time"
        );

        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn work_DidNotReceiveDifferentGenerics_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup()
            .work::<f32, 1>(&value)
            .returns_once(returned_value)
            .work::<String, 124>(&value)
            .returns_once(String::from("unused"));

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&value);
        let panic_msg = record_panic(|| mock.verify_calls());

        // Assert
        assert_eq!(returned_value, actual_returned_value);
        let arg_debug_string = common::debug_string("&i32 == 5");
        let string_name = core::any::type_name::<String>();
        let expected_panic_msg = format!(
            "Expected following calls:
	Trait<i32, true>::work<{string_name}, 124>({arg_debug_string}) 1 time"
        );
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn work_ReceivedUnexpectedCalls_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let first_value = 5;
        let first_returned_value = 3.0f32;
        const FIRST_N: usize = 1;
        let second_value = 100;
        let second_returned_value = [4; 3];
        const SECOND_N: usize = 200;
        mock.setup()
            .work::<f32, FIRST_N>(&first_value)
            .returns_once(first_returned_value);
        mock.setup()
            .work::<_, SECOND_N>(&second_value)
            .returns_once(second_returned_value);

        // Act
        let actual_first_returned_value = mock.work::<f32, FIRST_N>(&first_value);
        let actual_second_returned_value = mock.work::<[i32; 3], SECOND_N>(&second_value);
        let panic_msg = record_panic(|| mock.work::<[i32; 3], SECOND_N>(&second_value));

        // Assert
        assert_eq!(first_returned_value, actual_first_returned_value);
        assert_eq!(second_returned_value, actual_second_returned_value);

        let actual_second_debug_string = common::debug_string("100");
        let expected_panic_msg = format!(
            "Mock wasn't configured to handle following call:
	Trait<i32, true>::work<[i32; 3], 200>({actual_second_debug_string})"
        );
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn static_work_NoConfigs_Ok() {
        // Arrange
        // Act
        let panic_msg = record_panic(|| TraitMock::<i32, false>::static_work::<f32, 5>(&14));

        // Assert
        let expected_debug_string = common::debug_string("14");
        let expected_panic_msg = format!(
            "Mock wasn't configured to handle following call:
	Trait<i32, false>::static_work<f32, 5>({expected_debug_string})"
        );
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn static_work_OnlyUnsuitableConfigs_Ok() {
        // Arrange

        let value = 5;
        TraitMock::<i32, false>::static_setup()
            .static_work::<f32, 1>(&2)
            .returns_once(3.0f32)
            .static_work::<f32, 10>(&value)
            .returns_once(3.0f32)
            .static_work::<[u8; 4], 10>(&value)
            .returns_once([1, 2, 3, 4])
            .static_work::<[u8; 4], 10>(&2)
            .returns_once([1, 2, 3, 4]);

        // Act
        let panic_msg = record_panic(|| TraitMock::<i32, false>::static_work::<f32, 1>(&value));

        // Assert
        let expected_debug_string = common::debug_string("5");
        let expected_panic_msg = format!("Mock wasn't configured to handle following call:
	Trait<i32, false>::static_work<f32, 1>({expected_debug_string})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: static_work(*{expected_debug_string}*)");
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn static_work_NoReturnValue_Ok() {
        // Arrange
        let value = 5;
        let returned_value = 3.0f32;
        TraitMock::<i32, false>::static_setup().static_work::<f32, 1>(&value);
        TraitMock::<i32, false>::static_setup()
            .static_work::<f32, 1>(&value)
            .returns_once(returned_value);

        // Act
        let actual_returned_value = TraitMock::<i32, false>::static_work::<f32, 1>(&value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);
        TraitMock::<i32, false>::verify_static_calls();
    }

    #[test]
    fn static_work_DidNotReceiveSameGenerics_Ok() {
        // Arrange
        let value = 5;
        let returned_value = 3.0f32;
        const N: usize = 1;
        TraitMock::<i32, false>::static_setup()
            .static_work::<f32, N>(&value)
            .returns(returned_value, 2.times());

        // Act
        let actual_returned_value = TraitMock::<i32, false>::static_work::<f32, 1>(&value);
        let panic_msg = record_panic(TraitMock::<i32, false>::verify_static_calls);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let arg_debug_string = common::debug_string("&i32 == 5");
        let expected_panic_msg = format!(
            "Expected following calls:
	Trait<i32, false>::static_work<f32, {N}>({arg_debug_string}) 1 time"
        );

        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn static_work_DidNotReceiveDifferentGenerics_Ok() {
        // Arrange
        let value = 5;
        let returned_value = 3.0f32;
        TraitMock::<i32, false>::static_setup()
            .static_work::<f32, 1>(&value)
            .returns_once(returned_value)
            .static_work::<String, 124>(&value)
            .returns_once(String::from("unused"));

        // Act
        let actual_returned_value = TraitMock::<i32, false>::static_work::<f32, 1>(&value);
        let panic_msg = record_panic(TraitMock::<i32, false>::verify_static_calls);

        // Assert
        assert_eq!(returned_value, actual_returned_value);
        let arg_debug_string = common::debug_string("&i32 == 5");
        let string_name = core::any::type_name::<String>();
        let expected_panic_msg = format!(
            "Expected following calls:
	Trait<i32, false>::static_work<{string_name}, 124>({arg_debug_string}) 1 time"
        );
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn static_work_ReceivedUnexpectedCalls_Ok() {
        // Arrange
        let first_value = 5;
        let first_returned_value = 3.0f32;
        const FIRST_N: usize = 1;
        let second_value = 100;
        let second_returned_value = [4; 3];
        const SECOND_N: usize = 200;
        TraitMock::<i32, false>::static_setup()
            .static_work::<f32, FIRST_N>(&first_value)
            .returns_once(first_returned_value)
            .static_work::<_, SECOND_N>(&second_value)
            .returns_once(second_returned_value);

        // Act
        let actual_first_returned_value =
            TraitMock::<i32, false>::static_work::<f32, FIRST_N>(&first_value);
        let actual_second_returned_value =
            TraitMock::<i32, false>::static_work::<[i32; 3], SECOND_N>(&second_value);
        let panic_msg = record_panic(|| {
            TraitMock::<i32, false>::static_work::<[i32; 3], SECOND_N>(&second_value)
        });

        // Assert
        assert_eq!(first_returned_value, actual_first_returned_value);
        assert_eq!(second_returned_value, actual_second_returned_value);

        let actual_second_debug_string = common::debug_string("100");
        let expected_panic_msg = format!(
            "Mock wasn't configured to handle following call:
	Trait<i32, false>::static_work<[i32; 3], 200>({actual_second_debug_string})"
        );
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }
}
