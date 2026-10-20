use std::rc::Rc;
use std::sync::{LazyLock, RwLock};

mod sequence;
mod call_order_entry;

pub(crate) use sequence::Sequence;
pub(crate) use call_order_entry::CallOrderEntry;

pub fn in_sequence(mut setups: impl FnMut()) -> Rc<Sequence> {
    let sequence = Sequence {
        expected_calls_order: Default::default(),
    };
    set_current_sequence(sequence);
    setups();
    extract_current_sequence()
}

thread_local! {
    static CURRENT_SEQUENCE: LazyLock<RwLock<Option<Rc<Sequence>>>> = LazyLock::new(Default::default);
}

pub fn try_get_current_sequence() -> Option<Rc<Sequence>> {
    CURRENT_SEQUENCE.with(|x| {
        x.read()
         .expect("TODO generic message bla-bla can not read")
         .clone()
    })
}

fn set_current_sequence(sequence: Sequence) {
    CURRENT_SEQUENCE.with(|x| {
        *x.write()
          .expect("TODO generic message bla-bla can not write") = Some(Rc::new(sequence));
    });
}

fn extract_current_sequence() -> Rc<Sequence> {
    CURRENT_SEQUENCE.with(|x| {
        x.write()
         .expect("TODO generic message bla-bla can not write")
         .take()
         .expect("TODO error - sequence should be present")
    })
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn should_perform_Ok() {
        // Arrange
        let should = true;
        CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.set(should));

        // Act
        let result = should_perform();

        // Assert
        assert_eq!(result, should);
    }

    #[test]
    fn add_call_Ok() {
        // Arrange
        let initial_calls_length = CALL_ORDER_STATE.with(|x| x.expected_calls_order.borrow().len());
        let new_call_order_number = 5usize;
        let call_formatted_string = "quo vadis".to_owned();

        // Act
        add_call(new_call_order_number, call_formatted_string.clone());

        // Act
        CALL_ORDER_STATE.with(|x| {
            let expected_calls_order = x.expected_calls_order.borrow();
            assert_eq!(expected_calls_order.len(), initial_calls_length + 1);

            let new_call_order_entry = &expected_calls_order[initial_calls_length];
            assert_eq!(
                new_call_order_entry.call_order_number,
                new_call_order_number
            );
            assert_eq!(new_call_order_entry.formatted_string, call_formatted_string);
        });
    }

    #[test]
    fn enable_Ok() {
        // Arrange
        CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.set(false));

        // Act
        enable();

        // Assert
        CALL_ORDER_STATE.with(|x| assert!(x.perform_call_order_verification.get()));
    }

    #[test]
    fn disable_Ok() {
        // Arrange
        CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.set(true));

        // Act
        disable();

        // Assert
        CALL_ORDER_STATE.with(|x| assert!(!x.perform_call_order_verification.get()));
    }

    #[test]
    fn validate_actual_calls_order_OrderIsCorrect_DoesNothing() {
        // Arrange
        let calls = vec![
            CallOrderEntry {
                call_order_number: 5,
                formatted_string: "quo vadis".to_owned(),
            },
            CallOrderEntry {
                call_order_number: 10,
                formatted_string: "veridis quo".to_owned(),
            },
        ];
        CALL_ORDER_STATE.with(|x| *x.expected_calls_order.borrow_mut() = calls);

        // Act
        validate_actual_calls_order();

        // Assert
        panic_invalid_calls_order::received_nothing();
    }

    #[test]
    fn validate_actual_calls_order_OrderIsIncorrect_PrintsError() {
        // Arrange
        let calls = vec![
            CallOrderEntry {
                call_order_number: 10,
                formatted_string: "quo vadis".to_owned(),
            },
            CallOrderEntry {
                call_order_number: 5,
                formatted_string: "veridis quo".to_owned(),
            },
        ];
        CALL_ORDER_STATE.with(|x| *x.expected_calls_order.borrow_mut() = calls);

        // Act
        validate_actual_calls_order();

        // Arrange
        CALL_ORDER_STATE.with(|x| {
            let mut mut_calls = x.expected_calls_order.borrow_mut();
            panic_invalid_calls_order::received(
                automock::Arg::ref_eq(mut_calls.as_mut_slice()),
                automock::Times::Once,
            )
            .no_other_calls();
        });
    }
}
