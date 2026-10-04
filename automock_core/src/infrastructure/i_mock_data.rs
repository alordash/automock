use crate::infrastructure::*;

#[cfg_attr(test, automock::mock)]
pub trait IMockData {
    fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>>;

    fn verify_received_nothing_else(&self) {
        let all_error_msgs: Vec<_> = self.get_received_nothing_else_error_msgs();
        if all_error_msgs.first().is_none_or(|x| x.is_empty()) {
            return;
        }
        let error_msgs: Vec<_> = all_error_msgs.into_iter().flatten().collect();
        panic_received_unexpected_calls_error(error_msgs);
    }
}

#[cfg_attr(test, automock::mock)]
fn panic_received_unexpected_calls_error(error_msgs: Vec<String>) {
    error_printing::panic_received_unexpected_calls_error(error_msgs);
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn verify_received_nothing_else_NoReceivedNothingElseErrorMsgs_DoesNothing() {
        // Arrange
        let mut mock_data = IMockDataMock::new();
        let received_nothing_else_error_msgs = Vec::new();
        mock_data
            .setup()
            .get_received_nothing_else_error_msgs()
            .returns(received_nothing_else_error_msgs)
            .verify_received_nothing_else()
            .call_base();

        // Act
        mock_data.verify_received_nothing_else();

        // Assert
        panic_received_unexpected_calls_error::received_nothing();

        mock_data
            .received()
            .get_received_nothing_else_error_msgs(automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn verify_received_nothing_else_FirstReceivedNothingElseErrorMsgIsEmpty_DoesNothing() {
        // Arrange
        let mut mock_data = IMockDataMock::new();
        let received_nothing_else_error_msgs =
            vec![Vec::new(), vec!["veridis".to_owned(), "quo".to_owned()]];
        mock_data
            .setup()
            .get_received_nothing_else_error_msgs()
            .returns(received_nothing_else_error_msgs)
            .verify_received_nothing_else()
            .call_base();

        // Act
        mock_data.verify_received_nothing_else();

        // Assert
        panic_received_unexpected_calls_error::received_nothing();

        mock_data
            .received()
            .get_received_nothing_else_error_msgs(automock::Times::Once)
            .no_other_calls();
    }

    #[test]
    fn verify_received_nothing_else_HasReceivedNothingElseErrorMsgs_DoesNothing() {
        // Arrange
        let mut mock_data = IMockDataMock::new();
        let received_nothing_else_error_msgs = vec![
            vec!["quo".to_owned(), "vadis".to_owned()],
            vec!["veridis".to_owned(), "quo".to_owned()],
        ];
        mock_data
            .setup()
            .get_received_nothing_else_error_msgs()
            .returns(received_nothing_else_error_msgs.clone())
            .verify_received_nothing_else()
            .call_base();

        // Act
        mock_data.verify_received_nothing_else();

        // Assert
        let expected_error_msgs: Vec<_> = received_nothing_else_error_msgs
            .into_iter()
            .flatten()
            .collect();
        panic_received_unexpected_calls_error::received(expected_error_msgs, automock::Times::Once)
            .no_other_calls();

        mock_data
            .received()
            .get_received_nothing_else_error_msgs(automock::Times::Once)
            .no_other_calls();
    }
}
