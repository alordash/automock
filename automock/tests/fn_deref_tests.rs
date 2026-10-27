mod common;

use automock::*;
use common::*;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

#[mock]
fn ref_test(r: &i32) {}

#[mock]
fn rc_test(rc: Rc<i32>) {}

#[mock]
fn arc_test(rc: Arc<i32>) {}

#[derive(Debug, Clone, PartialEq)]
struct Custom(Rc<i32>);
impl Custom {
    pub fn new(v: i32) -> Self {
        Self(Rc::new(v))
    }
}
impl Deref for Custom {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mock]
fn custom_test(custom: Custom) {}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    mod ref_tests {
        use super::*;

        #[test]
        fn ref_test_Raw_ComparesByValue() {
            // Arrange
            let v = [1, 1, 1];
            let [r1, r2, r3] = &v;
            let counter = Counter::new();

            ref_test::setup(r1)
                .expect(3.times())
                .and_does(move |_| counter.inc());

            // Act 1
            ref_test(r2);
            ref_test(r3);

            // Assert 1
            assert_eq!(2, counter.get());

            // Act 2
            ref_test(r1);

            // Assert 2
            assert_eq!(3, counter.get());
            ref_test::verify_calls();
        }

        #[test]
        fn ref_test_RefEq_ComparesByReference() {
            // Arrange
            let v = [1, 1, 1];
            let [r1, r2, r3] = &v;
            let counter = Counter::new();

            ref_test::setup(Arg::ref_eq(r1))
                .expect_once()
                .and_does(move |_| counter.inc())
                .setup(Arg::ref_eq(r2))
                .expect_once()
                .setup(Arg::ref_eq(r3))
                .expect_once();

            // Act 1
            ref_test(r2);
            ref_test(r3);

            // Assert 1
            assert_eq!(0, counter.get());

            // Act 2
            ref_test(r1);

            // Assert 2
            assert_eq!(1, counter.get());
            ref_test::verify_calls();
        }
    }

    mod rc_tests {
        use super::*;

        #[test]
        fn rc_test_Raw_ComparesByValue() {
            // Arrange
            let v = [Rc::new(1), Rc::new(1), Rc::new(1)];
            let [r1, r2, r3] = v;
            let counter = Counter::new();

            rc_test::setup(r1.clone())
                .expect(3.times())
                .and_does(move |_| counter.inc());

            // Act 1
            rc_test(r2.clone());
            rc_test(r3.clone());

            // Assert 1
            assert_eq!(2, counter.get());

            // Act 2
            rc_test(r1.clone());

            // Assert 2
            assert_eq!(3, counter.get());
            rc_test::verify_calls();
        }

        #[test]
        fn rc_test_RefEq_ComparesByReference() {
            // Arrange
            let v = [Rc::new(1), Rc::new(1), Rc::new(1)];
            let [r1, r2, r3] = v;
            let counter = Counter::new();

            rc_test::setup(Arg::ref_eq(r1.clone()))
                .expect_once()
                .and_does(move |_| counter.inc())
                .setup(Arg::ref_eq(r2.clone()))
                .expect_once()
                .setup(Arg::ref_eq(r3.clone()))
                .expect_once();

            // Act 1
            rc_test(r2.clone());
            rc_test(r3.clone());

            // Assert 1
            assert_eq!(0, counter.get());

            // Act 2
            rc_test(r1.clone());

            // Assert 2
            assert_eq!(1, counter.get());
            rc_test::verify_calls();
        }
    }

    mod arc_tests {
        use super::*;

        #[test]
        fn arc_test_Raw_ComparesByValue() {
            // Arrange
            let v = [Arc::new(1), Arc::new(1), Arc::new(1)];
            let [r1, r2, r3] = v;
            let counter = Counter::new();

            arc_test::setup(r1.clone())
                .expect(3.times())
                .and_does(move |_| counter.inc());

            // Act 1
            arc_test(r2.clone());
            arc_test(r3.clone());

            // Assert 1
            assert_eq!(2, counter.get());

            // Act 2
            arc_test(r1.clone());

            // Assert 2
            assert_eq!(3, counter.get());
            arc_test::verify_calls();
        }

        #[test]
        fn arc_test_RefEq_ComparesByReference() {
            // Arrange
            let v = [Arc::new(1), Arc::new(1), Arc::new(1)];
            let [r1, r2, r3] = v;
            let counter = Counter::new();

            arc_test::setup(Arg::ref_eq(r1.clone()))
                .expect_once()
                .and_does(move |_| counter.inc())
                .setup(Arg::ref_eq(r2.clone()))
                .expect_once()
                .setup(Arg::ref_eq(r3.clone()))
                .expect_once();

            // Act 1
            arc_test(r2.clone());
            arc_test(r3.clone());

            // Assert 1
            assert_eq!(0, counter.get());

            // Act 2
            arc_test(r1.clone());

            // Assert 2
            assert_eq!(1, counter.get());
            arc_test::verify_calls();
        }
    }

    mod custom_tests {
        use super::*;

        #[test]
        fn custom_test_Raw_ComparesByValue() {
            // Arrange
            let v = [Custom::new(1), Custom::new(1), Custom::new(1)];
            let [r1, r2, r3] = v;
            let counter = Counter::new();

            custom_test::setup(r1.clone())
                .expect(3.times())
                .and_does(move |_| counter.inc());

            // Act 1
            custom_test(r2.clone());
            custom_test(r3.clone());

            // Assert 1
            assert_eq!(2, counter.get());

            // Act 2
            custom_test(r1.clone());

            // Assert 2
            assert_eq!(3, counter.get());
            custom_test::verify_calls();
        }

        #[test]
        fn custom_test_RefEq_ComparesByReference() {
            // Arrange
            let v = [Custom::new(1), Custom::new(1), Custom::new(1)];
            let [r1, r2, r3] = v;
            let counter = Counter::new();

            custom_test::setup(Arg::ref_eq(r1.clone()))
                .expect_once()
                .and_does(move |_| counter.inc())
                .setup(Arg::ref_eq(r2.clone()))
                .expect_once()
                .setup(Arg::ref_eq(r3.clone()))
                .expect_once();

            // Act 1
            custom_test(r2.clone());
            custom_test(r3.clone());

            // Assert 1
            assert_eq!(0, counter.get());

            // Act 2
            custom_test(r1.clone());

            // Assert 2
            assert_eq!(1, counter.get());
            custom_test::verify_calls();
        }
    }
}
