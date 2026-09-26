#![allow(unused)]
use automock::*;
use std::fmt::*;

#[mock]
struct Struct {
    number: i32,
}

#[mock]
impl Struct {
    fn first_struct_impl(&self) {}
}

#[mock(base)]
impl MyTrait for Struct {
    fn work(&self, value: i32) -> String {
        return "working...".to_owned();
    }
}

#[mock(base)]
impl Struct {
    pub fn new(number: i32) -> Self {
        Self { number }
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }

    pub fn format(&self) -> String {
        let number = self.get_number();
        let work_result = self.work(number);
        let result = format!("Struct, number = {number}, work_result = {work_result}");
        return result;
    }
}

impl Debug for Struct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Struct = {{ number = {} }}", self.number);
    }
}

trait MyTrait {
    fn work(&self, value: i32) -> String;
}

trait Gen<T> {}

mod tests {
    use super::*;

    #[test]
    fn struct_test() {
        // Arrange
        let mock_number = 10;
        Struct::static_setup().new(Arg::Any).call_base();
        let mut mock = Struct::new(mock_number);

        let get_number_returned_value = 22;
        mock.setup()
            .get_number()
            .always_returns(get_number_returned_value)
            .format()
            .call_base();

        let my_trait_work_returned_value_for_format = "for format!".to_owned();
        let my_trait_work_accepted_value_for_call_base = 333;
        let my_trait_work_returned_value_for_mock = "Mocked value!".to_owned();
        let my_trait_work_accepted_value_for_mock = 4;
        mock.setup()
            .as_MyTrait()
            .work(get_number_returned_value)
            .returns(my_trait_work_returned_value_for_format.clone())
            .work(my_trait_work_accepted_value_for_call_base)
            .call_base()
            .work(my_trait_work_accepted_value_for_mock)
            .returns(my_trait_work_returned_value_for_mock.clone());

        // Act
        let actual_get_number_returned_value = mock.get_number();
        let actual_format_value = mock.format();

        let actual_my_trait_work_call_base_value =
            mock.work(my_trait_work_accepted_value_for_call_base);
        let actual_my_trait_work_returned_value_for_mock =
            mock.work(my_trait_work_accepted_value_for_mock);

        // Assert
        assert_eq!(get_number_returned_value, actual_get_number_returned_value);
        let expected_format_value = format!(
            "Struct, number = {get_number_returned_value}, work_result = {my_trait_work_returned_value_for_format}"
        );
        assert_eq!(expected_format_value, actual_format_value);

        let expected_my_trait_work_call_base_value = "working...".to_owned();
        assert_eq!(
            expected_my_trait_work_call_base_value,
            actual_my_trait_work_call_base_value
        );
        assert_eq!(
            my_trait_work_returned_value_for_mock,
            actual_my_trait_work_returned_value_for_mock
        );

        mock.received().get_number(Times::Exactly(2));
        mock.received()
            .as_MyTrait()
            .work(my_trait_work_accepted_value_for_mock, Times::Once)
            .work(get_number_returned_value, Times::Once);
        mock.received().no_other_calls();
    }
}
