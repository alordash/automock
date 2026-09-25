use crate::fn_parameters::DynCall;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::*;

mod formatting;
pub(crate) use formatting::*;

#[cfg_attr(test, allow(unused))]
static CALL_ORDER_NUMBER: AtomicUsize = AtomicUsize::new(0);

// #[cfg_attr(test, automock::mock)]
fn get_next_call_order_number() -> usize {
    CALL_ORDER_NUMBER.fetch_add(1, Ordering::AcqRel)
}

pub struct CallCheck<'rs> {
    pub number: usize,
    verified: Cell<bool>,
    call: Rc<DynCall<'rs>>,
}

impl<'rs> CallCheck<'rs> {
    pub fn new(call: Rc<DynCall<'rs>>) -> Self {
        Self {
            number: get_next_call_order_number(),
            verified: Cell::new(false),
            call,
        }
    }

    pub fn mark_as_verified(&self) {
        self.verified.set(true);
    }

    pub fn is_not_verified(&self) -> bool {
        !self.verified.get()
    }

    pub fn get_call(&self) -> &DynCall<'rs> {
        &self.call
    }
}

// #[cfg(test)]
// mod tests {
//     #![allow(non_snake_case)]
// 
//     use super::*;
//     use crate::fn_parameters::tests::CallMock;
//     use crate::fn_parameters::*;
// 
//     #[test]
//     fn new_Ok() {
//         // Arrange
//         let number = 12usize;
//         get_next_call_order_number::setup().returns(number);
//         let call = Rc::new(DynCall::new(CallMock::new()));
// 
//         // Act
//         let result = CallCheck::new(call.clone());
// 
//         // Assert
//         assert_eq!(result.number, number);
//         assert!(!result.verified.get());
//         assert!(Rc::ptr_eq(&result.call, &call))
//     }
// 
//     #[test]
//     fn mark_as_verified_Ok() {
//         // Arrange
//         let call_check = CallCheck {
//             number: 1,
//             verified: Cell::new(false),
//             call: Rc::new(DynCall::new(CallMock::new())),
//         };
// 
//         // Act
//         call_check.mark_as_verified();
// 
//         // Assert
//         assert!(call_check.verified.get());
//     }
// 
//     #[test]
//     fn is_not_verified_WhenNotVerified_ReturnsTrue() {
//         // Arrange
//         let call_check = CallCheck {
//             number: 1,
//             verified: Cell::new(false),
//             call: Rc::new(DynCall::new(CallMock::new())),
//         };
// 
//         // Act
//         let result = call_check.is_not_verified();
// 
//         // Assert
//         assert!(result);
//     }
// 
//     #[test]
//     fn is_not_verified_WhenVerified_ReturnsFalse() {
//         // Arrange
//         let call_check = CallCheck {
//             number: 1,
//             verified: Cell::new(true),
//             call: Rc::new(DynCall::new(CallMock::new())),
//         };
// 
//         // Act
//         let result = call_check.is_not_verified();
// 
//         // Assert
//         assert!(!result);
//     }
// 
//     #[test]
//     fn get_call_Ok() {
//         // Arrange
//         let call = Rc::new(DynCall::new(CallMock::new()));
// 
//         let call_check = CallCheck {
//             number: 1,
//             verified: Cell::new(false),
//             call: call.clone(),
//         };
// 
//         // Act
//         let result = call_check.get_call();
// 
//         // Assert
//         assert!(core::ptr::eq(result, call.as_ref()));
//     }
// }
