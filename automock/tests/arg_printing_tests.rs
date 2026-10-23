use automock::*;
use std::marker::PhantomData;

mod common;

#[mock]
fn accept_ref<'b>(r: &&&'b i32) -> i32 {
    unreachable!()
}

#[mock]
fn accept_ref_ptr<'b>(r: &&*const &&'b i32) -> i32 {
    unreachable!()
}

#[mock]
fn generic<T1, T2>(t1: T1) -> T2 {
    unreachable!()
}

#[mock]
trait Trait<'a, T0> {
    fn accept_ref<'b>(&self, r: &'a &&'b i32) -> i32;

    fn accept_ref_ptr<'b>(&self, r: &'a &*const &&'b i32) -> i32;

    fn generic<T1, T2>(&self, t1: T1) -> T2;
}

#[mock]
#[derive(Clone)]
struct Struct<'s, TS> {
    phantom: PhantomData<&'s TS>,
}

#[mock]
impl<'s, TS> Struct<'s, TS> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[mock]
impl<'s, TS> Struct<'s, TS> {
    fn accept_ref<'b>(&self, _r: &'s &&'b i32) -> i32 {
        unreachable!()
    }

    fn accept_ref_ptr<'b>(&self, _r: &'s &*const &&'b i32) -> i32 {
        unreachable!()
    }

    fn generic<T1, T2>(&self, _t1: T1) -> T2 {
        unreachable!()
    }
}

#[mock]
impl<'s, 'a, TS> Trait<'a, String> for Struct<'s, TS> {
    fn accept_ref<'b>(&self, r: &'a &&'b i32) -> i32 {
        Struct::<'s, TS>::accept_ref(self, transmute_lifetime!(r))
    }

    fn accept_ref_ptr<'b>(&self, r: &'a &*const &&'b i32) -> i32 {
        Struct::<'s, TS>::accept_ref_ptr(self, transmute_lifetime!(r))
    }

    fn generic<T1, T2>(&self, t1: T1) -> T2 {
        Struct::<'s, TS>::generic(self, t1)
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::record_panic;

    type T0 = [u8; 3];

    mod r#fn {
        use super::*;

        #[test]
        fn accept_ref_NoConfig_Ok() {
            // Arrange
            let r = &&&5;

            // Act
            let panic_msg = record_panic(|| accept_ref(r));

            // Assert
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	accept_ref({r})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_DidNotReceive_Ok() {
            // Arrange
            let r = &&&5;
            let return_value = 175;
            let unexpected_r = &&&14;

            accept_ref::setup(r)
                .returns_once(return_value)
                .setup(unexpected_r)
                .returns_once(return_value);

            // Act
            let actual_return_value = accept_ref(r);
            let panic_msg = record_panic(accept_ref::verify_calls);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected following calls:
	accept_ref(&&&i32 == {unexpected_r}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_UnexpectedCall_Ok() {
            // Arrange
            let r = &&&5;
            let return_value = 175;
            let unexpected_r = &&&14;

            accept_ref::setup(r).returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| accept_ref(unexpected_r));
            let actual_return_value = accept_ref(r);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	accept_ref({unexpected_r})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref(*{unexpected_r}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            accept_ref::verify_calls();
        }

        #[test]
        fn accept_ref_ptr_NoConfig_Ok() {
            // Arrange
            let r = &&(&&&5 as *const &&i32);

            // Act
            let panic_msg = record_panic(|| accept_ref_ptr(r));

            // Assert
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	accept_ref_ptr({r:?})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_DidNotReceive_Ok() {
            // Arrange
            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);

            accept_ref_ptr::setup(r)
                .returns_once(return_value)
                .setup(unexpected_r)
                .returns_once(return_value);

            // Act
            let actual_return_value = accept_ref_ptr(r);
            let panic_msg = record_panic(accept_ref_ptr::verify_calls);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected following calls:
	accept_ref_ptr(&&*const &&i32 == {unexpected_r:?}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_UnexpectedCall_Ok() {
            // Arrange
            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);

            accept_ref_ptr::setup(r).returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| accept_ref_ptr(unexpected_r));
            let actual_return_value = accept_ref_ptr(r);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	accept_ref_ptr({unexpected_r:?})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref_ptr(*{unexpected_r:?}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            accept_ref_ptr::verify_calls();
        }

        #[test]
        fn generic_NoConfig_Ok() {
            // Arrange
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;

            // Act
            let panic_msg = record_panic(|| generic::<T1, T2>(t1));

            // Assert
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	generic<{t1_name}, {t2_name}>({t1_debug_string})",
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveSameGenerics_Ok() {
            // Arrange
            type T1 = i32;
            type T2 = f64;
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            generic::setup(t1)
                .returns_once(return_value)
                .setup(unexpected_t1)
                .returns_once(return_value);

            // Act
            let actual_return_value: T2 = generic(t1);
            let panic_msg = record_panic(generic::verify_calls::<T1, T2>);

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string = common::debug_string(unexpected_t1.to_string());
            let expected_panic_msg = format!(
                "Expected following calls:
	generic({arg_debug_string}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveDifferentGenerics_Ok() {
            // Arrange
            type T1 = i32;
            type T2 = f64;
            type T3 = usize;
            type T4 = String;
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t3: T3 = 11;
            let unexpected_return_value: T4 = "unexpected".to_owned();

            generic::setup(t1).returns_once(return_value);
            generic::setup::<T3, T4>(unexpected_t3).returns_once(unexpected_return_value);

            // Act
            let actual_return_value: T2 = generic(t1);
            let panic_msg = record_panic(generic::verify_calls::<T3, T4>);

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string = common::debug_string(unexpected_t3.to_string());
            let expected_panic_msg = format!(
                "Expected following calls:
	generic({arg_debug_string}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            generic::verify_calls::<T1, T2>();
        }

        #[test]
        fn generic_ref_UnexpectedCall_Ok() {
            // Arrange
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            generic::setup(t1).returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| generic::<T1, T2>(unexpected_t1));
            let actual_return_value = generic(t1);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let unexpected_t1_debug_string = common::debug_string(unexpected_t1.to_string());
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	generic<{t1_name}, {t2_name}>({unexpected_t1_debug_string})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: generic(*{unexpected_t1_debug_string}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            generic::verify_calls::<T1, T2>();
        }
    }

    mod r#trait {
        use super::*;

        #[test]
        fn accept_ref_NoConfig_Ok() {
            // Arrange
            let mock = TraitMock::<T0>::new();

            let r = &&&5;

            // Act
            let panic_msg = record_panic(|| mock.accept_ref(r));

            // Assert
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Trait::accept_ref({r})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_DidNotReceive_Ok() {
            // Arrange
            let mut mock = TraitMock::<T0>::new();

            let r = &&&5;
            let return_value = 175;
            let unexpected_r = &&&14;

            mock.setup().accept_ref(r).returns_once(return_value);
            mock.setup()
                .accept_ref(unexpected_r)
                .returns_once(return_value);

            // Act
            let actual_return_value = mock.accept_ref(r);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected following calls:
	Trait::accept_ref(&&&i32 == {unexpected_r}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = TraitMock::<T0>::new();

            let r = &&&5;
            let return_value = 175;
            let unexpected_r = &&&14;

            mock.setup().accept_ref(r).returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| mock.accept_ref(unexpected_r));
            let actual_return_value = mock.accept_ref(r);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Trait::accept_ref({unexpected_r})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref(*{unexpected_r}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            mock.verify_calls();
        }

        #[test]
        fn accept_ref_ptr_NoConfig_Ok() {
            // Arrange
            let mock = TraitMock::<T0>::new();

            let r = &&(&&&5 as *const &&i32);

            // Act
            let panic_msg = record_panic(|| mock.accept_ref_ptr(r));

            // Assert
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Trait::accept_ref_ptr({r:?})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_DidNotReceive_Ok() {
            // Arrange
            let mut mock = TraitMock::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);

            mock.setup().accept_ref_ptr(r).returns_once(return_value);
            mock.setup()
                .accept_ref_ptr(unexpected_r)
                .returns_once(return_value);

            // Act
            let actual_return_value = mock.accept_ref_ptr(r);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected following calls:
	Trait::accept_ref_ptr(&&*const &&i32 == {unexpected_r:?}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = TraitMock::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);

            mock.setup().accept_ref_ptr(r).returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| mock.accept_ref_ptr(unexpected_r));
            let actual_return_value = mock.accept_ref_ptr(r);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Trait::accept_ref_ptr({unexpected_r:?})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref_ptr(*{unexpected_r:?}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            mock.verify_calls();
        }

        #[test]
        fn generic_NoConfig_Ok() {
            // Arrange
            let mock = TraitMock::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;

            // Act
            let panic_msg = record_panic(|| mock.generic::<T1, T2>(t1));

            // Assert
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Trait::generic<{t1_name}, {t2_name}>({t1_debug_string})",
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveSameGenerics_Ok() {
            // Arrange
            let mut mock = TraitMock::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            mock.setup().generic(t1).returns_once(return_value);
            mock.setup()
                .generic(unexpected_t1)
                .returns_once(return_value);

            // Act
            let actual_return_value: T2 = mock.generic(t1);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string = common::debug_string(unexpected_t1.to_string());
            let expected_panic_msg = format!(
                "Expected following calls:
	Trait::generic({arg_debug_string}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveDifferentGenerics_Ok() {
            // Arrange
            let mut mock = TraitMock::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            type T3 = usize;
            type T4 = String;
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t3: T3 = 11;
            let unexpected_return_value: T4 = "unexpected".to_owned();

            mock.setup().generic(t1).returns_once(return_value);
            mock.setup()
                .generic::<T3, T4>(unexpected_t3)
                .returns_once(unexpected_return_value);

            // Act
            let actual_return_value: T2 = mock.generic(t1);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string = common::debug_string(unexpected_t3.to_string());
            let expected_panic_msg = format!(
                "Expected following calls:
	Trait::generic({arg_debug_string}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_ref_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = TraitMock::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            mock.setup().generic(t1).returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| mock.generic::<T1, T2>(unexpected_t1));
            let actual_return_value = mock.generic(t1);

            // Assert
            assert_eq!(return_value, actual_return_value);
            let unexpected_t1_debug_string = common::debug_string(unexpected_t1.to_string());
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Trait::generic<{t1_name}, {t2_name}>({unexpected_t1_debug_string})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: generic(*{unexpected_t1_debug_string}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            mock.verify_calls();
        }
    }

    mod r#struct {
        use super::*;

        #[test]
        fn accept_ref_NoConfig_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mock = Struct::<T0>::new();

            let r = &&&5;

            // Act
            let panic_msg = record_panic(|| mock.accept_ref(r));

            // Assert
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Struct::accept_ref({r})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_DidNotReceive_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();

            let r = &&&5;
            let return_value = 175;
            let unexpected_r = &&&14;

            mock.setup().accept_ref(r).returns_once(return_value);
            mock.setup()
                .accept_ref(unexpected_r)
                .returns_once(return_value);

            // Act
            let actual_return_value = mock.accept_ref(r);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected following calls:
	Struct::accept_ref(&&&i32 == {unexpected_r}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_UnexpectedCall_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();

            let r = &&&5;
            let return_value = 175;
            let unexpected_r = &&&14;

            mock.setup().accept_ref(r).returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| mock.accept_ref(unexpected_r));
            let actual_return_value = mock.accept_ref(r);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Struct::accept_ref({unexpected_r})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref(*{unexpected_r}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            mock.verify_calls();
        }

        #[test]
        fn accept_ref_ptr_NoConfig_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);

            // Act
            let panic_msg = record_panic(|| mock.accept_ref_ptr(r));

            // Assert
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Struct::accept_ref_ptr({r:?})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_DidNotReceive_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);

            mock.setup().accept_ref_ptr(r).returns_once(return_value);
            mock.setup()
                .accept_ref_ptr(unexpected_r)
                .returns_once(return_value);

            // Act
            let actual_return_value = mock.accept_ref_ptr(r);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected following calls:
	Struct::accept_ref_ptr(&&*const &&i32 == {unexpected_r:?}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_UnexpectedCall_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);

            mock.setup().accept_ref_ptr(r).returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| mock.accept_ref_ptr(unexpected_r));
            let actual_return_value = mock.accept_ref_ptr(r);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Struct::accept_ref_ptr({unexpected_r:?})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref_ptr(*{unexpected_r:?}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            mock.verify_calls();
        }

        #[test]
        fn generic_NoConfig_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;

            // Act
            let panic_msg = record_panic(|| mock.generic::<T1, T2>(t1));

            // Assert
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Struct::generic<{t1_name}, {t2_name}>({t1_debug_string})",
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveSameGenerics_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            mock.setup().generic(t1).returns_once(return_value);
            mock.setup()
                .generic(unexpected_t1)
                .returns_once(return_value);

            // Act
            let actual_return_value: T2 = mock.generic(t1);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string = common::debug_string(unexpected_t1.to_string());
            let expected_panic_msg = format!(
                "Expected following calls:
	Struct::generic({arg_debug_string}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveDifferentGenerics_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            type T3 = usize;
            type T4 = String;
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t3: T3 = 11;
            let unexpected_return_value: T4 = "unexpected".to_owned();

            mock.setup().generic(t1).returns_once(return_value);
            mock.setup()
                .generic::<T3, T4>(unexpected_t3)
                .returns_once(unexpected_return_value);

            // Act
            let actual_return_value: T2 = mock.generic(t1);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string = common::debug_string(unexpected_t3.to_string());
            let expected_panic_msg = format!(
                "Expected following calls:
	Struct::generic({arg_debug_string}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_ref_UnexpectedCall_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            mock.setup().generic(t1).returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| mock.generic::<T1, T2>(unexpected_t1));
            let actual_return_value = mock.generic(t1);

            // Assert
            assert_eq!(return_value, actual_return_value);
            let unexpected_t1_debug_string = common::debug_string(unexpected_t1.to_string());
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	Struct::generic<{t1_name}, {t2_name}>({unexpected_t1_debug_string})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: generic(*{unexpected_t1_debug_string}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            mock.verify_calls();
        }
    }

    mod struct_as_trait {
        use super::*;

        #[test]
        fn accept_ref_NoConfig_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mock = Struct::<T0>::new();

            let r = &&&5;

            // Act
            let panic_msg = record_panic(|| Trait::accept_ref(&mock, r));

            // Assert
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	<Struct as Trait>::accept_ref({r})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_DidNotReceive_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();

            let r = &&&5;
            let return_value = 175;
            let unexpected_r = &&&14;

            mock.setup()
                .as_Trait()
                .accept_ref(r)
                .returns_once(return_value);
            mock.setup()
                .as_Trait()
                .accept_ref(unexpected_r)
                .returns_once(return_value);

            // Act
            let actual_return_value = Trait::accept_ref(&mock, r);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected following calls:
	<Struct as Trait>::accept_ref(&&&i32 == {unexpected_r}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_UnexpectedCall_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();

            let r = &&&5;
            let return_value = 175;
            let unexpected_r = &&&14;

            mock.setup()
                .as_Trait()
                .accept_ref(r)
                .returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| Trait::accept_ref(&mock, unexpected_r));
            let actual_return_value = Trait::accept_ref(&mock, r);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	<Struct as Trait>::accept_ref({unexpected_r})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref(*{unexpected_r}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            mock.verify_calls();
        }

        #[test]
        fn accept_ref_ptr_NoConfig_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);

            // Act
            let panic_msg = record_panic(|| Trait::accept_ref_ptr(&mock, r));

            // Assert
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	<Struct as Trait>::accept_ref_ptr({r:?})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_DidNotReceive_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);

            mock.setup()
                .as_Trait()
                .accept_ref_ptr(r)
                .returns_once(return_value);
            mock.setup()
                .as_Trait()
                .accept_ref_ptr(unexpected_r)
                .returns_once(return_value);

            // Act
            let actual_return_value = Trait::accept_ref_ptr(&mock, r);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected following calls:
	<Struct as Trait>::accept_ref_ptr(&&*const &&i32 == {unexpected_r:?}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_UnexpectedCall_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);

            mock.setup()
                .as_Trait()
                .accept_ref_ptr(r)
                .returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| Trait::accept_ref_ptr(&mock, unexpected_r));
            let actual_return_value = Trait::accept_ref_ptr(&mock, r);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	<Struct as Trait>::accept_ref_ptr({unexpected_r:?})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: accept_ref_ptr(*{unexpected_r:?}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            mock.verify_calls();
        }

        #[test]
        fn generic_NoConfig_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;

            // Act
            let panic_msg = record_panic(|| Trait::generic::<T1, T2>(&mock, t1));

            // Assert
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	<Struct as Trait>::generic<{t1_name}, {t2_name}>({t1_debug_string})",
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveSameGenerics_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            mock.setup()
                .as_Trait()
                .generic(t1)
                .returns_once(return_value);
            mock.setup()
                .as_Trait()
                .generic(unexpected_t1)
                .returns_once(return_value);

            // Act
            let actual_return_value: T2 = Trait::generic(&mock, t1);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string = common::debug_string(unexpected_t1.to_string());
            let expected_panic_msg = format!(
                "Expected following calls:
	<Struct as Trait>::generic({arg_debug_string}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveDifferentGenerics_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            type T3 = usize;
            type T4 = String;
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t3: T3 = 11;
            let unexpected_return_value: T4 = "unexpected".to_owned();

            mock.setup()
                .as_Trait()
                .generic(t1)
                .returns_once(return_value);
            mock.setup()
                .as_Trait()
                .generic::<T3, T4>(unexpected_t3)
                .returns_once(unexpected_return_value);

            // Act
            let actual_return_value: T2 = Trait::generic(&mock, t1);
            let panic_msg = record_panic(|| mock.verify_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string = common::debug_string(unexpected_t3.to_string());
            let expected_panic_msg = format!(
                "Expected following calls:
	<Struct as Trait>::generic({arg_debug_string}) 1 time"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_ref_UnexpectedCall_Ok() {
            // Arrange
            Struct::<T0>::static_setup().new().call_base();
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            mock.setup()
                .as_Trait()
                .generic(t1)
                .returns_once(return_value);

            // Act
            let panic_msg = record_panic(|| Trait::generic::<T1, T2>(&mock, unexpected_t1));
            let actual_return_value = Trait::generic(&mock, t1);

            // Assert
            assert_eq!(return_value, actual_return_value);
            let unexpected_t1_debug_string = common::debug_string(unexpected_t1.to_string());
            let expected_panic_msg = format!(
                "Mock wasn't configured to handle following call:
	<Struct as Trait>::generic<{t1_name}, {t2_name}>({unexpected_t1_debug_string})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: generic(*{unexpected_t1_debug_string}*)"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
            mock.verify_calls();
        }
    }
}
