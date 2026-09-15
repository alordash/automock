use rsubstitute::*;
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

#[mock(base)]
impl<'s, TS> Struct<'s, TS> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[mock]
impl<'s, TS> Struct<'s, TS> {
    fn accept_ref<'b>(&self, r: &'s &&'b i32) -> i32 {
        unreachable!()
    }

    fn accept_ref_ptr<'b>(&self, r: &'s &*const &&'b i32) -> i32 {
        unreachable!()
    }

    fn generic<T1, T2>(&self, t1: T1) -> T2 {
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
            let r_ptr = core::ptr::from_ref(r);
            let return_value = 175;
            let unexpected_r = &&&14;
            let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

            accept_ref::setup(r).returns(return_value);

            // Act
            let actual_return_value = accept_ref(r);
            let panic_msg =
                record_panic(|| accept_ref::received(Arg::ref_eq(unexpected_r), Times::Once));

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	accept_ref((&&&i32): equal to {unexpected_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref(*{r}*)
	1. r (&&&i32):
		Expected (ptr: {unexpected_r_ptr:?}): {unexpected_r}
		Actual   (ptr: {r_ptr:?}): {r}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_UnexpectedCall_Ok() {
            // Arrange
            let r = &&&5;
            let return_value = 175;

            accept_ref::setup(r).returns(return_value);

            // Act
            let actual_return_value = accept_ref(r);
            let panic_msg = record_panic(accept_ref::received_nothing);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. accept_ref({r})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
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
            let r_ptr = core::ptr::from_ref(r);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);
            let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

            accept_ref_ptr::setup(r).returns(return_value);

            // Act
            let actual_return_value = accept_ref_ptr(r);
            let panic_msg =
                record_panic(|| accept_ref_ptr::received(Arg::ref_eq(unexpected_r), Times::Once));

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	accept_ref_ptr((&&*const &&i32): equal to {unexpected_r:?})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref_ptr(*{r:?}*)
	1. r (&&*const &&i32):
		Expected (ptr: {unexpected_r_ptr:?}): {unexpected_r:?}
		Actual   (ptr: {r_ptr:?}): {r:?}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_UnexpectedCall_Ok() {
            // Arrange
            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;

            accept_ref_ptr::setup(r).returns(return_value);

            // Act
            let actual_return_value = accept_ref_ptr(r);
            let panic_msg = record_panic(accept_ref_ptr::received_nothing);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. accept_ref_ptr({r:?})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
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
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            generic::setup(t1).returns(return_value);

            // Act
            let actual_return_value: T2 = generic(t1);
            let panic_msg =
                record_panic(|| generic::received::<T1, T2>(unexpected_t1, Times::Once));

            // Assert
            assert_eq!(return_value, actual_return_value);
            let unexpected_t1_debug_string = common::debug_string(unexpected_t1.to_string());
            let arg_debug_string =
                common::debug_string(format!("({t1_name}): equal to {unexpected_t1}"));
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	generic<{t1_name}, {t2_name}>({arg_debug_string})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
generic(*{t1_debug_string}*)
	1. t1 ({t1_name}):
		Expected: {unexpected_t1_debug_string}
		Actual:   {t1_debug_string}"
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
            let t3_name = core::any::type_name::<T3>();
            let t4_name = core::any::type_name::<T4>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t3: T3 = 11;

            generic::setup(t1).returns(return_value);

            // Act
            let actual_return_value: T2 = generic(t1);
            let panic_msg =
                record_panic(|| generic::received::<T3, T4>(unexpected_t3, Times::Once));

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string =
                common::debug_string(format!("({t3_name}): equal to {unexpected_t3}"));
            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	generic<{t3_name}, {t4_name}>({arg_debug_string})
Actually received no matching calls
Received no non-matching calls"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
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

            generic::setup(t1).returns(return_value);

            // Act
            let actual_return_value = generic(t1);
            let panic_msg = record_panic(generic::received_nothing::<T1, T2>);

            // Assert
            assert_eq!(return_value, actual_return_value);

            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. generic<{t1_name}, {t2_name}>({t1_debug_string})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
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
            let r_ptr = core::ptr::from_ref(r);
            let return_value = 175;
            let unexpected_r = &&&14;
            let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

            mock.setup().accept_ref(r).returns(return_value);

            // Act
            let actual_return_value = mock.accept_ref(r);
            let panic_msg = record_panic(|| {
                mock.received()
                    .accept_ref(Arg::ref_eq(unexpected_r), Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	Trait::accept_ref((&&&i32): equal to {unexpected_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref(*{r}*)
	1. r (&&&i32):
		Expected (ptr: {unexpected_r_ptr:?}): {unexpected_r}
		Actual   (ptr: {r_ptr:?}): {r}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = TraitMock::<T0>::new();

            let r = &&&5;
            let return_value = 175;

            mock.setup().accept_ref(r).returns(return_value);

            // Act
            let actual_return_value = mock.accept_ref(r);
            let panic_msg = record_panic(|| mock.received().no_other_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. Trait::accept_ref({r})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
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
            let r_ptr = core::ptr::from_ref(r);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);
            let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

            mock.setup().accept_ref_ptr(r).returns(return_value);

            // Act
            let actual_return_value = mock.accept_ref_ptr(r);
            let panic_msg = record_panic(|| {
                mock.received()
                    .accept_ref_ptr(Arg::ref_eq(unexpected_r), Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	Trait::accept_ref_ptr((&&*const &&i32): equal to {unexpected_r:?})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref_ptr(*{r:?}*)
	1. r (&&*const &&i32):
		Expected (ptr: {unexpected_r_ptr:?}): {unexpected_r:?}
		Actual   (ptr: {r_ptr:?}): {r:?}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = TraitMock::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;

            mock.setup().accept_ref_ptr(r).returns(return_value);

            // Act
            let actual_return_value = mock.accept_ref_ptr(r);
            let panic_msg = record_panic(|| mock.received().no_other_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. Trait::accept_ref_ptr({r:?})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
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
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            mock.setup().generic(t1).returns(return_value);

            // Act
            let actual_return_value: T2 = mock.generic(t1);
            let panic_msg = record_panic(|| {
                mock.received()
                    .generic::<T1, T2>(unexpected_t1, Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);
            let unexpected_t1_debug_string = common::debug_string(unexpected_t1.to_string());
            let arg_debug_string =
                common::debug_string(format!("({t1_name}): equal to {unexpected_t1}"));
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	Trait::generic<{t1_name}, {t2_name}>({arg_debug_string})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
generic(*{t1_debug_string}*)
	1. t1 ({t1_name}):
		Expected: {unexpected_t1_debug_string}
		Actual:   {t1_debug_string}"
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
            let t3_name = core::any::type_name::<T3>();
            let t4_name = core::any::type_name::<T4>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t3: T3 = 11;

            mock.setup().generic(t1).returns(return_value);

            // Act
            let actual_return_value: T2 = mock.generic(t1);
            let panic_msg = record_panic(|| {
                mock.received()
                    .generic::<T3, T4>(unexpected_t3, Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string =
                common::debug_string(format!("({t3_name}): equal to {unexpected_t3}"));
            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	Trait::generic<{t3_name}, {t4_name}>({arg_debug_string})
Actually received no matching calls
Received no non-matching calls"
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

            mock.setup().generic(t1).returns(return_value);

            // Act
            let actual_return_value = mock.generic(t1);
            let panic_msg = record_panic(|| mock.received().no_other_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. Trait::generic<{t1_name}, {t2_name}>({t1_debug_string})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }
    }

    mod r#struct {
        use super::*;

        #[test]
        fn accept_ref_NoConfig_Ok() {
            // Arrange
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
            let mut mock = Struct::<T0>::new();

            let r = &&&5;
            let r_ptr = core::ptr::from_ref(r);
            let return_value = 175;
            let unexpected_r = &&&14;
            let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

            mock.setup().accept_ref(r).returns(return_value);

            // Act
            let actual_return_value = mock.accept_ref(r);
            let panic_msg = record_panic(|| {
                mock.received()
                    .accept_ref(Arg::ref_eq(unexpected_r), Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	Struct::accept_ref((&&&i32): equal to {unexpected_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref(*{r}*)
	1. r (&&&i32):
		Expected (ptr: {unexpected_r_ptr:?}): {unexpected_r}
		Actual   (ptr: {r_ptr:?}): {r}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = Struct::<T0>::new();

            let r = &&&5;
            let return_value = 175;

            mock.setup().accept_ref(r).returns(return_value);

            // Act
            let actual_return_value = mock.accept_ref(r);
            let panic_msg = record_panic(|| mock.received().no_other_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. Struct::accept_ref({r})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_NoConfig_Ok() {
            // Arrange
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
            let mut mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let r_ptr = core::ptr::from_ref(r);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);
            let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

            mock.setup().accept_ref_ptr(r).returns(return_value);

            // Act
            let actual_return_value = mock.accept_ref_ptr(r);
            let panic_msg = record_panic(|| {
                mock.received()
                    .accept_ref_ptr(Arg::ref_eq(unexpected_r), Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	Struct::accept_ref_ptr((&&*const &&i32): equal to {unexpected_r:?})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref_ptr(*{r:?}*)
	1. r (&&*const &&i32):
		Expected (ptr: {unexpected_r_ptr:?}): {unexpected_r:?}
		Actual   (ptr: {r_ptr:?}): {r:?}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;

            mock.setup().accept_ref_ptr(r).returns(return_value);

            // Act
            let actual_return_value = mock.accept_ref_ptr(r);
            let panic_msg = record_panic(|| mock.received().no_other_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. Struct::accept_ref_ptr({r:?})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_NoConfig_Ok() {
            // Arrange
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
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            mock.setup().generic(t1).returns(return_value);

            // Act
            let actual_return_value: T2 = mock.generic(t1);
            let panic_msg = record_panic(|| {
                mock.received()
                    .generic::<T1, T2>(unexpected_t1, Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);
            let unexpected_t1_debug_string = common::debug_string(unexpected_t1.to_string());
            let arg_debug_string =
                common::debug_string(format!("({t1_name}): equal to {unexpected_t1}"));
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	Struct::generic<{t1_name}, {t2_name}>({arg_debug_string})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
generic(*{t1_debug_string}*)
	1. t1 ({t1_name}):
		Expected: {unexpected_t1_debug_string}
		Actual:   {t1_debug_string}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveDifferentGenerics_Ok() {
            // Arrange
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            type T3 = usize;
            type T4 = String;
            let t3_name = core::any::type_name::<T3>();
            let t4_name = core::any::type_name::<T4>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t3: T3 = 11;

            mock.setup().generic(t1).returns(return_value);

            // Act
            let actual_return_value: T2 = mock.generic(t1);
            let panic_msg = record_panic(|| {
                mock.received()
                    .generic::<T3, T4>(unexpected_t3, Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string =
                common::debug_string(format!("({t3_name}): equal to {unexpected_t3}"));
            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	Struct::generic<{t3_name}, {t4_name}>({arg_debug_string})
Actually received no matching calls
Received no non-matching calls"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_ref_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;

            mock.setup().generic(t1).returns(return_value);

            // Act
            let actual_return_value = mock.generic(t1);
            let panic_msg = record_panic(|| mock.received().no_other_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. Struct::generic<{t1_name}, {t2_name}>({t1_debug_string})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }
    }

    mod struct_as_trait {
        use super::*;

        #[test]
        fn accept_ref_NoConfig_Ok() {
            // Arrange
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
            let mut mock = Struct::<T0>::new();

            let r = &&&5;
            let r_ptr = core::ptr::from_ref(r);
            let return_value = 175;
            let unexpected_r = &&&14;
            let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

            mock.setup().as_Trait().accept_ref(r).returns(return_value);

            // Act
            let actual_return_value = Trait::accept_ref(&mock, r);
            let panic_msg = record_panic(|| {
                mock.received()
                    .as_Trait()
                    .accept_ref(Arg::ref_eq(unexpected_r), Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	<Struct as Trait>::accept_ref((&&&i32): equal to {unexpected_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref(*{r}*)
	1. r (&&&i32):
		Expected (ptr: {unexpected_r_ptr:?}): {unexpected_r}
		Actual   (ptr: {r_ptr:?}): {r}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = Struct::<T0>::new();

            let r = &&&5;
            let return_value = 175;

            mock.setup().as_Trait().accept_ref(r).returns(return_value);

            // Act
            let actual_return_value = Trait::accept_ref(&mock, r);
            let panic_msg = record_panic(|| mock.received().as_Trait().no_other_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. <Struct as Trait>::accept_ref({r})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_NoConfig_Ok() {
            // Arrange
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
            let mut mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let r_ptr = core::ptr::from_ref(r);
            let return_value = 175;
            let unexpected_r = &&(&&&14 as *const &&i32);
            let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

            mock.setup()
                .as_Trait()
                .accept_ref_ptr(r)
                .returns(return_value);

            // Act
            let actual_return_value = Trait::accept_ref_ptr(&mock, r);
            let panic_msg = record_panic(|| {
                mock.received()
                    .as_Trait()
                    .accept_ref_ptr(Arg::ref_eq(unexpected_r), Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	<Struct as Trait>::accept_ref_ptr((&&*const &&i32): equal to {unexpected_r:?})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref_ptr(*{r:?}*)
	1. r (&&*const &&i32):
		Expected (ptr: {unexpected_r_ptr:?}): {unexpected_r:?}
		Actual   (ptr: {r_ptr:?}): {r:?}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn accept_ref_ptr_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = Struct::<T0>::new();

            let r = &&(&&&5 as *const &&i32);
            let return_value = 175;

            mock.setup()
                .as_Trait()
                .accept_ref_ptr(r)
                .returns(return_value);

            // Act
            let actual_return_value = Trait::accept_ref_ptr(&mock, r);
            let panic_msg = record_panic(|| mock.received().as_Trait().no_other_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);

            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. <Struct as Trait>::accept_ref_ptr({r:?})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_NoConfig_Ok() {
            // Arrange
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
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t1: T1 = 235;

            mock.setup().as_Trait().generic(t1).returns(return_value);

            // Act
            let actual_return_value: T2 = Trait::generic(&mock, t1);
            let panic_msg = record_panic(|| {
                mock.received()
                    .as_Trait()
                    .generic::<T1, T2>(unexpected_t1, Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);
            let unexpected_t1_debug_string = common::debug_string(unexpected_t1.to_string());
            let arg_debug_string =
                common::debug_string(format!("({t1_name}): equal to {unexpected_t1}"));
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	<Struct as Trait>::generic<{t1_name}, {t2_name}>({arg_debug_string})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
generic(*{t1_debug_string}*)
	1. t1 ({t1_name}):
		Expected: {unexpected_t1_debug_string}
		Actual:   {t1_debug_string}"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_DidNotReceiveDifferentGenerics_Ok() {
            // Arrange
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            type T3 = usize;
            type T4 = String;
            let t3_name = core::any::type_name::<T3>();
            let t4_name = core::any::type_name::<T4>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;
            let unexpected_t3: T3 = 11;

            mock.setup().as_Trait().generic(t1).returns(return_value);

            // Act
            let actual_return_value: T2 = Trait::generic(&mock, t1);
            let panic_msg = record_panic(|| {
                mock.received()
                    .as_Trait()
                    .generic::<T3, T4>(unexpected_t3, Times::Once)
            });

            // Assert
            assert_eq!(return_value, actual_return_value);
            let arg_debug_string =
                common::debug_string(format!("({t3_name}): equal to {unexpected_t3}"));
            let expected_panic_msg = format!(
                "Expected to receive a call exactly once matching:
	<Struct as Trait>::generic<{t3_name}, {t4_name}>({arg_debug_string})
Actually received no matching calls
Received no non-matching calls"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }

        #[test]
        fn generic_ref_UnexpectedCall_Ok() {
            // Arrange
            let mut mock = Struct::<T0>::new();
            type T1 = i32;
            type T2 = f64;
            let t1_name = core::any::type_name::<T1>();
            let t2_name = core::any::type_name::<T2>();
            let t1: T1 = 5;
            let return_value: T2 = 64.0f64;

            mock.setup().as_Trait().generic(t1).returns(return_value);

            // Act
            let actual_return_value = Trait::generic(&mock, t1);
            let panic_msg = record_panic(|| mock.received().as_Trait().no_other_calls());

            // Assert
            assert_eq!(return_value, actual_return_value);
            let t1_debug_string = common::debug_string(t1.to_string());
            let expected_panic_msg = format!(
                "Did not expect to receive any other calls. Received 1 unexpected call:
1. <Struct as Trait>::generic<{t1_name}, {t2_name}>({t1_debug_string})"
            );
            assert_eq!(Some(expected_panic_msg), panic_msg);
        }
    }
}
