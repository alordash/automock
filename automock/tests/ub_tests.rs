use automock::*;

#[mock]
trait Trait {
    fn work(&self, r: &i32);
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    /// Invalid usage example.
    #[test]
    pub fn work_ReferenceToTemporaryValue_ReceivesDanglingReference() {
        // Arrange
        let mut mock = TraitMock::new();
        mock.setup()
            .work(Arg::is(|reference: &&i32| {
                let actual_ptr_to_temporary_value = core::ptr::from_ref(*reference);
                return unsafe { PTR_TO_TEMPORARY_VALUE } == actual_ptr_to_temporary_value;
            }))
            .expect_once();

        // Act
        static mut PTR_TO_TEMPORARY_VALUE: *const i32 = core::ptr::null();
        #[inline(never)]
        fn inner_call(mock: &TraitMock) {
            let temporary_value = 5;
            unsafe { PTR_TO_TEMPORARY_VALUE = &temporary_value };
            mock.work(&temporary_value);
        }
        inner_call(&mock);

        // Assert
        mock.verify_calls();
    }

    /// Fix by moving `received` section to the place where all arguments data is still alive.
    #[test]
    pub fn work_ReferenceToTemporaryValue_FixByReceivingBeforeDrop_ReceivesValidReference() {
        // Arrange
        let mut mock = TraitMock::new();

        // Act & Assert
        #[inline(never)]
        fn inner_call(mock: &mut TraitMock) {
            let temporary_value = 5;
            mock.setup()
                .work(Arg::is(|reference: &&i32| {
                    return **reference == temporary_value;
                }))
                .expect_once();
            mock.work(&temporary_value);
            mock.verify_calls();
        }
        inner_call(&mut mock);
    }

    /// Fix by moving temporary value to the scope of mock.
    #[test]
    pub fn work_FixByNotUsingTemporaryValue_ReceivesValidReference() {
        // Arrange
        let mut mock = TraitMock::new();
        let temporary_value = 5;
        mock.setup().work(&temporary_value).expect_once();

        // Act
        mock.work(&temporary_value);

        // Assert
        mock.verify_calls();
    }
}
