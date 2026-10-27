#![allow(clippy::needless_arbitrary_self_type)]
#![allow(clippy::arc_with_non_send_sync)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::redundant_allocation)]
#![allow(clippy::boxed_local)]

mod common;

use automock::*;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[rustfmt::skip]
#[allow(unused)]
mod consts {
    pub const BY_VALUE:                                       i32 = 1 ;
    pub const BY_VALUE_COLON:                                 i32 = 2 ;
    pub const BY_MUT_VALUE:                                   i32 = 3 ;
    pub const BY_MUT_VALUE_COLON:                             i32 = 4 ;
    pub const BY_REF:                                         i32 = 5 ;
    pub const BY_REF_COLON:                                   i32 = 6 ;
    pub const BY_REF_WITH_LIFETIME:                           i32 = 7 ;
    pub const BY_REF_COLON_WITH_LIFETIME:                     i32 = 8 ;
    pub const BY_REF_MUT:                                     i32 = 9 ;
    pub const BY_REF_MUT_COLON:                               i32 = 10;
    pub const BY_REF_MUT_WITH_LIFETIME:                       i32 = 11;
    pub const BY_REF_MUT_COLON_WITH_LIFETIME:                 i32 = 12;
    pub const BY_BOX:                                         i32 = 13;
    pub const BY_BOX_REF:                                     i32 = 14;
    pub const BY_BOX_REF_MUT:                                 i32 = 15;
    pub const BY_MUT_BOX:                                     i32 = 16;
    pub const BY_MUT_BOX_REF:                                 i32 = 17;
    pub const BY_MUT_BOX_REF_MUT:                             i32 = 18;
    pub const BY_RC:                                          i32 = 19;
    pub const BY_RC_REF:                                      i32 = 20;
    pub const BY_RC_REF_MUT:                                  i32 = 21;
    pub const BY_MUT_RC:                                      i32 = 22;
    pub const BY_MUT_RC_REF:                                  i32 = 23;
    pub const BY_MUT_RC_REF_MUT:                              i32 = 24;
    pub const BY_ARC:                                         i32 = 25;
    pub const BY_ARC_REF:                                     i32 = 26;
    pub const BY_ARC_REF_MUT:                                 i32 = 27;
    pub const BY_MUT_ARC:                                     i32 = 28;
    pub const BY_MUT_ARC_REF:                                 i32 = 29;
    pub const BY_MUT_ARC_REF_MUT:                             i32 = 30;
    pub const BY_PIN_REF:                                     i32 = 31;
    pub const BY_PIN_REF_MUT:                                 i32 = 32;
    pub const BY_MUT_PIN_REF:                                 i32 = 33;
    pub const BY_MUT_PIN_REF_MUT:                             i32 = 34;
    pub const BY_MUT_REF_MUT_BOX_MUT_REF_MUT_WITH_LIFETIMES:  i32 = 35;
    pub const BY_MUT_REF_MUT_RC_MUT_REF_MUT_WITH_LIFETIMES:   i32 = 36;
    pub const BY_MUT_REF_MUT_ARC_MUT_REF_MUT_WITH_LIFETIMES:  i32 = 37;
    pub const BY_MUT_REF_MUT_PIN_MUT_REF_MUT_WITH_LIFETIMES:  i32 = 38;
    pub const NESTED:                                         i32 = 39;
}
use consts::*;

#[rustfmt::skip]
#[mock]
#[allow(unused_mut)]
trait Trait: Sized {
    fn by_value          (    self      ) {}
    fn by_value_colon    (    self: Self) {}
    fn by_mut_value      (mut self      ) {}
    fn by_mut_value_colon(mut self: Self) {}

    fn by_ref                            (&       self              ) {}
    fn by_ref_colon                      (        self: &       Self) {}
    fn by_ref_with_lifetime          <'a>(&'a     self              ) {}
    fn by_ref_colon_with_lifetime    <'a>(        self: &'a     Self) {}
    fn by_ref_mut                        (&   mut self              ) {}
    fn by_ref_mut_colon                  (        self: &   mut Self) {}
    fn by_ref_mut_with_lifetime      <'a>(&'a mut self              ) {}
    fn by_ref_mut_colon_with_lifetime<'a>(        self: &'a mut Self) {}

    fn by_box            (    self: Box<        Self>) {}
    fn by_box_ref        (    self: Box<&       Self>) {}
    fn by_box_ref_mut    (    self: Box<&   mut Self>) {}
    fn by_mut_box        (mut self: Box<        Self>) {}
    fn by_mut_box_ref    (mut self: Box<&       Self>) {}
    fn by_mut_box_ref_mut(mut self: Box<&   mut Self>) {}

    fn by_rc             (    self: Rc <        Self>) {}
    fn by_rc_ref         (    self: Rc <&       Self>) {}
    fn by_rc_ref_mut     (    self: Rc <&   mut Self>) {}
    fn by_mut_rc         (mut self: Rc <        Self>) {}
    fn by_mut_rc_ref     (mut self: Rc <&       Self>) {}
    fn by_mut_rc_ref_mut (mut self: Rc <&   mut Self>) {}

    fn by_arc            (    self: Arc<        Self>) {}
    fn by_arc_ref        (    self: Arc<&       Self>) {}
    fn by_arc_ref_mut    (    self: Arc<&   mut Self>) {}
    fn by_mut_arc        (mut self: Arc<        Self>) {}
    fn by_mut_arc_ref    (mut self: Arc<&       Self>) {}
    fn by_mut_arc_ref_mut(mut self: Arc<&   mut Self>) {}

    fn by_pin_ref                  (    self: Pin<&       Self>) {}
    fn by_pin_ref_mut              (    self: Pin<&   mut Self>) {}
    fn by_mut_pin_ref              (mut self: Pin<&       Self>) {}
    fn by_mut_pin_ref_mut          (mut self: Pin<&   mut Self>) {}
    
    fn by_mut_ref_mut_box_mut_ref_mut_with_lifetimes<'a, 'b>(mut self: &'a mut Box<&'b mut Self>) {}
    fn by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes <'a, 'b>(mut self: &'a mut Rc <&'b mut Self>) {}
    fn by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes<'a, 'b>(mut self: &'a mut Arc<&'b mut Self>) {}
    fn by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes<'a, 'b>(mut self: &'a mut Pin<&'b mut Self>) {}

    fn nested<'a, 'b, 'c>(self: &'a Box<&mut Pin<&'c mut Rc<&'b mut Box<&'a& Arc<&mut &'c Pin<Rc<&'b mut&&mut Self>>>>>>>) {}

    fn return_by_value          (    self      ) -> i32 { BY_VALUE }
    fn return_by_value_colon    (    self: Self) -> i32 { BY_VALUE_COLON }
    fn return_by_mut_value      (mut self      ) -> i32 { BY_MUT_VALUE }
    fn return_by_mut_value_colon(mut self: Self) -> i32 { BY_MUT_VALUE_COLON }

    fn return_by_ref                            (&       self              ) -> i32 { BY_REF }
    fn return_by_ref_colon                      (        self: &       Self) -> i32 { BY_REF_COLON }
    fn return_by_ref_with_lifetime          <'a>(&'a     self              ) -> i32 { BY_REF_WITH_LIFETIME }
    fn return_by_ref_colon_with_lifetime    <'a>(        self: &'a     Self) -> i32 { BY_REF_COLON_WITH_LIFETIME }
    fn return_by_ref_mut                        (&   mut self              ) -> i32 { BY_REF_MUT }
    fn return_by_ref_mut_colon                  (        self: &   mut Self) -> i32 { BY_REF_MUT_COLON }
    fn return_by_ref_mut_with_lifetime      <'a>(&'a mut self              ) -> i32 { BY_REF_MUT_WITH_LIFETIME }
    fn return_by_ref_mut_colon_with_lifetime<'a>(        self: &'a mut Self) -> i32 { BY_REF_MUT_COLON_WITH_LIFETIME }

    fn return_by_box            (    self: Box<        Self>) -> i32 { BY_BOX }
    fn return_by_box_ref        (    self: Box<&       Self>) -> i32 { BY_BOX_REF }
    fn return_by_box_ref_mut    (    self: Box<&   mut Self>) -> i32 { BY_BOX_REF_MUT }
    fn return_by_mut_box        (mut self: Box<        Self>) -> i32 { BY_MUT_BOX }
    fn return_by_mut_box_ref    (mut self: Box<&       Self>) -> i32 { BY_MUT_BOX_REF }
    fn return_by_mut_box_ref_mut(mut self: Box<&   mut Self>) -> i32 { BY_MUT_BOX_REF_MUT }

    fn return_by_rc             (    self: Rc <        Self>) -> i32 { BY_RC }
    fn return_by_rc_ref         (    self: Rc <&       Self>) -> i32 { BY_RC_REF }
    fn return_by_rc_ref_mut     (    self: Rc <&   mut Self>) -> i32 { BY_RC_REF_MUT }
    fn return_by_mut_rc         (mut self: Rc <        Self>) -> i32 { BY_MUT_RC }
    fn return_by_mut_rc_ref     (mut self: Rc <&       Self>) -> i32 { BY_MUT_RC_REF }
    fn return_by_mut_rc_ref_mut (mut self: Rc <&   mut Self>) -> i32 { BY_MUT_RC_REF_MUT }

    fn return_by_arc            (    self: Arc<        Self>) -> i32 { BY_ARC }
    fn return_by_arc_ref        (    self: Arc<&       Self>) -> i32 { BY_ARC_REF }
    fn return_by_arc_ref_mut    (    self: Arc<&   mut Self>) -> i32 { BY_ARC_REF_MUT }
    fn return_by_mut_arc        (mut self: Arc<        Self>) -> i32 { BY_MUT_ARC }
    fn return_by_mut_arc_ref    (mut self: Arc<&       Self>) -> i32 { BY_MUT_ARC_REF }
    fn return_by_mut_arc_ref_mut(mut self: Arc<&   mut Self>) -> i32 { BY_MUT_ARC_REF_MUT }

    fn return_by_pin_ref        (    self: Pin<&       Self>) -> i32 { BY_PIN_REF }
    fn return_by_pin_ref_mut    (    self: Pin<&   mut Self>) -> i32 { BY_PIN_REF_MUT }
    fn return_by_mut_pin_ref    (mut self: Pin<&       Self>) -> i32 { BY_MUT_PIN_REF }
    fn return_by_mut_pin_ref_mut(mut self: Pin<&   mut Self>) -> i32 { BY_MUT_PIN_REF_MUT }
    
    fn return_by_mut_ref_mut_box_mut_ref_mut_with_lifetimes<'a, 'b>(mut self: &'a mut Box<&'b mut Self>) -> i32 { BY_MUT_REF_MUT_BOX_MUT_REF_MUT_WITH_LIFETIMES }
    fn return_by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes <'a, 'b>(mut self: &'a mut Rc <&'b mut Self>) -> i32 { BY_MUT_REF_MUT_RC_MUT_REF_MUT_WITH_LIFETIMES }
    fn return_by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes<'a, 'b>(mut self: &'a mut Arc<&'b mut Self>) -> i32 { BY_MUT_REF_MUT_ARC_MUT_REF_MUT_WITH_LIFETIMES }
    fn return_by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes<'a, 'b>(mut self: &'a mut Pin<&'b mut Self>) -> i32 { BY_MUT_REF_MUT_PIN_MUT_REF_MUT_WITH_LIFETIMES }
    
    fn return_nested<'a, 'b, 'c>(self: &'a Box<&mut Pin<&'c mut Rc<&'b mut Box<&'a& Arc<&mut &'c Pin<Rc<&'b mut&&mut Self>>>>>>>) -> i32 { NESTED }
}

mod tests {
    #![allow(non_snake_case)]
    #![allow(unused_imports)]

    use super::*;
    use automock::*;
    use common::*;

    mod basic_tests {
        use super::*;

        #[test]
        fn by_value_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_value()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            mock.by_value();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_value_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_value_colon()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            mock.by_value_colon();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_value_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_value()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            mock.by_mut_value();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_value_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_value_colon()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            mock.by_mut_value_colon();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref();
            mock.by_ref();

            // Assert
            assert_eq!(2, counter.get());
            mock.verify_calls();
        }

        #[test]
        fn by_ref_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_colon()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_colon();
            mock.by_ref_colon();

            // Assert
            assert_eq!(2, counter.get());
            mock.verify_calls();
        }

        #[test]
        fn by_ref_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_with_lifetime()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_with_lifetime();
            mock.by_ref_with_lifetime();

            // Assert
            assert_eq!(2, counter.get());
            mock.verify_calls();
        }

        #[test]
        fn by_ref_colon_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_colon_with_lifetime()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_colon_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_colon_with_lifetime();
            mock.by_ref_colon_with_lifetime();

            // Assert
            assert_eq!(2, counter.get());
            mock.verify_calls();
        }

        #[test]
        fn by_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_mut()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_mut();
            mock.by_ref_mut();

            // Assert
            assert_eq!(2, counter.get());
            mock.verify_calls();
        }

        #[test]
        fn by_ref_mut_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_mut_colon()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_mut_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_mut_colon();
            mock.by_ref_mut_colon();

            // Assert
            assert_eq!(2, counter.get());
            mock.verify_calls();
        }

        #[test]
        fn by_ref_mut_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_mut_with_lifetime()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_mut_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_mut_with_lifetime();
            mock.by_ref_mut_with_lifetime();

            // Assert
            assert_eq!(2, counter.get());
            mock.verify_calls();
        }

        #[test]
        fn by_ref_mut_colon_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_mut_colon_with_lifetime()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_mut_colon_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_mut_colon_with_lifetime();
            mock.by_ref_mut_colon_with_lifetime();

            // Assert
            assert_eq!(2, counter.get());
            mock.verify_calls();
        }

        #[test]
        fn by_box_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_box()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            Box::new(mock).by_box();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_box_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_box_ref()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_box_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Box::new(&mock).by_box_ref();
            Box::new(&mock).by_box_ref();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_box_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_box_ref_mut()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_box_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Box::new(&mut mock).by_box_ref_mut();
            Box::new(&mut mock).by_box_ref_mut();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_box_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_box()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            Box::new(mock).by_mut_box();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_box_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_box_ref()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_box_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Box::new(&mock).by_mut_box_ref();
            Box::new(&mock).by_mut_box_ref();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_box_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_box_ref_mut()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_box_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Box::new(&mut mock).by_mut_box_ref_mut();
            Box::new(&mut mock).by_mut_box_ref_mut();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_rc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_rc()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            Rc::new(mock).by_rc();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_rc_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_rc_ref()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_rc_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Rc::new(&mock).by_rc_ref();
            Rc::new(&mock).by_rc_ref();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_rc_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_rc_ref_mut()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_rc_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Rc::new(&mut mock).by_rc_ref_mut();
            Rc::new(&mut mock).by_rc_ref_mut();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_rc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_rc()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            Rc::new(mock).by_mut_rc();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_rc_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_rc_ref()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_rc_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Rc::new(&mock).by_mut_rc_ref();
            Rc::new(&mock).by_mut_rc_ref();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_rc_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_rc_ref_mut()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_rc_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Rc::new(&mut mock).by_mut_rc_ref_mut();
            Rc::new(&mut mock).by_mut_rc_ref_mut();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_arc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_arc()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            Arc::new(mock).by_arc();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_arc_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_arc_ref()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_arc_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Arc::new(&mock).by_arc_ref();
            Arc::new(&mock).by_arc_ref();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_arc_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_arc_ref_mut()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_arc_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Arc::new(&mut mock).by_arc_ref_mut();
            Arc::new(&mut mock).by_arc_ref_mut();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_arc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_arc()
                .expect_once()
                .and_does(move |_, _| counter.inc());

            // Act
            Arc::new(mock).by_mut_arc();

            // Assert
            assert_eq!(1, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_arc_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_arc_ref()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_arc_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Arc::new(&mock).by_mut_arc_ref();
            Arc::new(&mock).by_mut_arc_ref();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_arc_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_arc_ref_mut()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_arc_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Arc::new(&mut mock).by_mut_arc_ref_mut();
            Arc::new(&mut mock).by_mut_arc_ref_mut();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_pin_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_pin_ref()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_pin_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Pin::new(&mock).by_pin_ref();
            Pin::new(&mock).by_pin_ref();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_pin_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_pin_ref_mut()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_pin_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Pin::new(&mut mock).by_pin_ref_mut();
            Pin::new(&mut mock).by_pin_ref_mut();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_pin_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_pin_ref()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_pin_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Pin::new(&mock).by_mut_pin_ref();
            Pin::new(&mock).by_mut_pin_ref();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_pin_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_pin_ref_mut()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_pin_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Pin::new(&mut mock).by_mut_pin_ref_mut();
            Pin::new(&mut mock).by_mut_pin_ref_mut();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_ref_mut_box_mut_ref_mut_with_lifetimes_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_mut_ref_mut_box_mut_ref_mut_with_lifetimes()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_ref_mut_box_mut_ref_mut_with_lifetimes()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            #[allow(unused_allocation)]
            Box::new(&mut mock).by_mut_ref_mut_box_mut_ref_mut_with_lifetimes();
            #[allow(unused_allocation)]
            Box::new(&mut mock).by_mut_ref_mut_box_mut_ref_mut_with_lifetimes();

            // Assert
            assert_eq!(2, counter.get());
            mock.verify_calls();
        }

        #[test]
        fn by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Rc::new(&mut mock).by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes();
            Rc::new(&mut mock).by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Arc::new(&mut mock).by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes();
            Arc::new(&mut mock).by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            Pin::new(&mut mock).by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes();
            Pin::new(&mut mock).by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }

        #[test]
        fn nested_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .nested()
                .expect(2.times())
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .nested()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            let mut the_self = mock;
            let mut the_self = &&mut the_self;
            let the_self = Rc::new(&mut the_self);
            let the_self = &mut &Pin::new(the_self);
            let the_self = &&Arc::new(the_self);
            let the_self = &mut Box::new(the_self);
            let the_self = &mut Rc::new(the_self);
            let the_self = &mut Pin::new(the_self);
            let the_self = &Box::new(the_self);

            // Act
            the_self.nested();
            the_self.nested();

            // Assert
            assert_eq!(2, counter.get());
            mock_data.verify_calls();
        }
    }

    mod return_tests {
        use super::*;

        #[test]
        fn return_by_value_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_value()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = mock.return_by_value();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_value_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_value()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = mock.return_by_value();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_VALUE, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_value_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_value_colon()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = mock.return_by_value_colon();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_value_colon_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_value_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = mock.return_by_value_colon();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_VALUE_COLON, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_value_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_value()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = mock.return_by_mut_value();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_value_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_mut_value()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = mock.return_by_mut_value();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_MUT_VALUE, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_value_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_value_colon()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = mock.return_by_mut_value_colon();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_value_colon_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_mut_value_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = mock.return_by_mut_value_colon();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_MUT_VALUE_COLON, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref();
            let actual_second_value = mock.return_by_ref();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_ref_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_colon()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_colon();
            let actual_second_value = mock.return_by_ref_colon();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_COLON, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_ref_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_with_lifetime()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_with_lifetime();
            let actual_second_value = mock.return_by_ref_with_lifetime();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_WITH_LIFETIME, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_ref_colon_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_colon_with_lifetime()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_colon_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_colon_with_lifetime();
            let actual_second_value = mock.return_by_ref_colon_with_lifetime();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_COLON_WITH_LIFETIME, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_mut()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_mut();
            let actual_second_value = mock.return_by_ref_mut();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_MUT, actual_second_value);
            mock.verify_calls();
        }

        #[test]
        fn return_by_ref_mut_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_mut_colon()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_mut_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_mut_colon();
            let actual_second_value = mock.return_by_ref_mut_colon();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_MUT_COLON, actual_second_value);
            mock.verify_calls();
        }

        #[test]
        fn return_by_ref_mut_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_mut_with_lifetime()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_mut_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_mut_with_lifetime();
            let actual_second_value = mock.return_by_ref_mut_with_lifetime();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_MUT_WITH_LIFETIME, actual_second_value);
            mock.verify_calls();
        }

        #[test]
        fn return_by_ref_mut_colon_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_mut_colon_with_lifetime()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_mut_colon_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_mut_colon_with_lifetime();
            let actual_second_value = mock.return_by_ref_mut_colon_with_lifetime();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_MUT_COLON_WITH_LIFETIME, actual_second_value);
            mock.verify_calls();
        }

        #[test]
        fn return_by_box_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_box()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = Box::new(mock).return_by_box();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_box_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_box()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = Box::new(mock).return_by_box();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_BOX, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_box_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_box_ref()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_box_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Box::new(&mock).return_by_box_ref();
            let actual_second_value = Box::new(&mock).return_by_box_ref();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_BOX_REF, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_box_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_box_ref_mut()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_box_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Box::new(&mut mock).return_by_box_ref_mut();
            let actual_second_value = Box::new(&mut mock).return_by_box_ref_mut();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_BOX_REF_MUT, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_box_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_box()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = Box::new(mock).return_by_mut_box();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_box_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_mut_box()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = Box::new(mock).return_by_mut_box();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_MUT_BOX, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_box_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_box_ref()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_box_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Box::new(&mock).return_by_mut_box_ref();
            let actual_second_value = Box::new(&mock).return_by_mut_box_ref();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_BOX_REF, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_box_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_box_ref_mut()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_box_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Box::new(&mut mock).return_by_mut_box_ref_mut();
            let actual_second_value = Box::new(&mut mock).return_by_mut_box_ref_mut();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_BOX_REF_MUT, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_rc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_rc()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = Rc::new(mock).return_by_rc();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_rc_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_rc()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = Rc::new(mock).return_by_rc();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_RC, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_rc_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_rc_ref()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_rc_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Rc::new(&mock).return_by_rc_ref();
            let actual_second_value = Rc::new(&mock).return_by_rc_ref();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_RC_REF, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_rc_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_rc_ref_mut()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_rc_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Rc::new(&mut mock).return_by_rc_ref_mut();
            let actual_second_value = Rc::new(&mut mock).return_by_rc_ref_mut();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_RC_REF_MUT, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_rc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_rc()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = Rc::new(mock).return_by_mut_rc();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_rc_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_mut_rc()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = Rc::new(mock).return_by_mut_rc();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_MUT_RC, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_rc_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_rc_ref()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_rc_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Rc::new(&mock).return_by_mut_rc_ref();
            let actual_second_value = Rc::new(&mock).return_by_mut_rc_ref();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_RC_REF, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_rc_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_rc_ref_mut()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_rc_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Rc::new(&mut mock).return_by_mut_rc_ref_mut();
            let actual_second_value = Rc::new(&mut mock).return_by_mut_rc_ref_mut();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_RC_REF_MUT, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_arc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_arc()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = Arc::new(mock).return_by_arc();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_arc_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_arc()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = Arc::new(mock).return_by_arc();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_ARC, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_arc_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_arc_ref()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_arc_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Arc::new(&mock).return_by_arc_ref();
            let actual_second_value = Arc::new(&mock).return_by_arc_ref();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_ARC_REF, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_arc_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_arc_ref_mut()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_arc_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Arc::new(&mut mock).return_by_arc_ref_mut();
            let actual_second_value = Arc::new(&mut mock).return_by_arc_ref_mut();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_ARC_REF_MUT, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_arc_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_arc()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());

            // Act
            let actual_first_value = Arc::new(mock).return_by_mut_arc();

            // Assert
            assert_eq!(1, counter.get());
            assert_eq!(first_value, actual_first_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_arc_CallBase_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            mock.setup()
                .return_by_mut_arc()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_second_value = Arc::new(mock).return_by_mut_arc();

            // Assert
            assert_eq!(2, counter.get());
            assert_eq!(BY_MUT_ARC, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_arc_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_arc_ref()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_arc_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Arc::new(&mock).return_by_mut_arc_ref();
            let actual_second_value = Arc::new(&mock).return_by_mut_arc_ref();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_ARC_REF, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_arc_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_arc_ref_mut()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_arc_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Arc::new(&mut mock).return_by_mut_arc_ref_mut();
            let actual_second_value = Arc::new(&mut mock).return_by_mut_arc_ref_mut();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_ARC_REF_MUT, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_pin_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_pin_ref()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_pin_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Pin::new(&mock).return_by_pin_ref();
            let actual_second_value = Pin::new(&mock).return_by_pin_ref();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_PIN_REF, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_pin_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_pin_ref_mut()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_pin_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Pin::new(&mut mock).return_by_pin_ref_mut();
            let actual_second_value = Pin::new(&mut mock).return_by_pin_ref_mut();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_PIN_REF_MUT, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_pin_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_pin_ref()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_pin_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Pin::new(&mock).return_by_mut_pin_ref();
            let actual_second_value = Pin::new(&mock).return_by_mut_pin_ref();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_PIN_REF, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_pin_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_pin_ref_mut()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_pin_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = Pin::new(&mut mock).return_by_mut_pin_ref_mut();
            let actual_second_value = Pin::new(&mut mock).return_by_mut_pin_ref_mut();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_PIN_REF_MUT, actual_second_value);
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_ref_mut_box_mut_ref_mut_with_lifetimes_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_ref_mut_box_mut_ref_mut_with_lifetimes()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_ref_mut_box_mut_ref_mut_with_lifetimes()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            #[allow(unused_allocation)]
            let actual_first_value =
                Box::new(&mut mock).return_by_mut_ref_mut_box_mut_ref_mut_with_lifetimes();
            #[allow(unused_allocation)]
            let actual_second_value =
                Box::new(&mut mock).return_by_mut_ref_mut_box_mut_ref_mut_with_lifetimes();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(
                BY_MUT_REF_MUT_BOX_MUT_REF_MUT_WITH_LIFETIMES,
                actual_second_value
            );
            mock.verify_calls();
        }

        #[test]
        fn return_by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value =
                Rc::new(&mut mock).return_by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes();
            let actual_second_value =
                Rc::new(&mut mock).return_by_mut_ref_mut_rc_mut_ref_mut_with_lifetimes();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(
                BY_MUT_REF_MUT_RC_MUT_REF_MUT_WITH_LIFETIMES,
                actual_second_value
            );
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value =
                Arc::new(&mut mock).return_by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes();
            let actual_second_value =
                Arc::new(&mut mock).return_by_mut_ref_mut_arc_mut_ref_mut_with_lifetimes();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(
                BY_MUT_REF_MUT_ARC_MUT_REF_MUT_WITH_LIFETIMES,
                actual_second_value
            );
            mock_data.verify_calls();
        }

        #[test]
        fn return_by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value =
                Pin::new(&mut mock).return_by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes();
            let actual_second_value =
                Pin::new(&mut mock).return_by_mut_ref_mut_pin_mut_ref_mut_with_lifetimes();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(
                BY_MUT_REF_MUT_PIN_MUT_REF_MUT_WITH_LIFETIMES,
                actual_second_value
            );
            mock_data.verify_calls();
        }

        #[test]
        fn return_nested_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let mock_data = mock.mock_data();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_nested()
                .returns_once(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_nested()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            let mut the_self = mock;
            let mut the_self = &&mut the_self;
            let the_self = Rc::new(&mut the_self);
            let the_self = &mut &Pin::new(the_self);
            let the_self = &&Arc::new(the_self);
            let the_self = &mut Box::new(the_self);
            let the_self = &mut Rc::new(the_self);
            let the_self = &mut Pin::new(the_self);
            let the_self = &Box::new(the_self);

            // Act
            let actual_first_value = the_self.return_nested();
            let actual_second_value = the_self.return_nested();

            // Assert
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(NESTED, actual_second_value);
            mock_data.verify_calls();
        }
    }
}
