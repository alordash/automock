use automock::*;
use std::marker::PhantomData;

#[mock]
struct Struct<S: ?Sized> {
    s: PhantomData<S>,
}

#[mock]
impl<S: ?Sized> Struct<S> {
    pub fn new() -> Self {
        Self { s: PhantomData }
    }
}

trait Trait<T> {
    fn work(&self, t: T) -> T;
}

#[mock]
impl<T, S: ?Sized> Trait<T> for Struct<S> {
    fn work(&self, t: T) -> T {
        t
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;

    type S = [u8];
    type T1 = i32;
    type T2<'a> = &'a str;

    #[test]
    fn Trait_work_DifferentTraitGenerics_Ok() {
        Struct::<S>::static_setup().new().call_base();
        let mut mock = Struct::<S>::new();
        let t1 = 10;
        let t1_result = 20;
        let t2 = "quo vadis";
        let t2_result = "veridis quo";
        mock.setup()
            .as_Trait::<T1>()
            .work(t1)
            .returns_once(t1_result);
        mock.setup()
            .as_Trait::<T2>()
            .work(t2)
            .returns_once(t2_result);

        // Act
        let actual_t1_result = mock.work(t1);
        let actual_t2_result = mock.work(t2);

        // Assert
        assert_eq!(actual_t1_result, t1_result);
        assert_eq!(actual_t2_result, t2_result);
        mock.verify_calls();
    }

    #[test]
    fn Trait_work_TooManyCalls_Panics() {
        // Arrange
        Struct::<S>::static_setup().new().call_base();
        let mut mock = Struct::<S>::new();
        let t1 = 10;
        let t2 = "quo vadis";
        mock.setup()
            .as_Trait::<T1>()
            .work(t1)
            .returns(20, 2.times());
        mock.setup()
            .as_Trait::<T2>()
            .work(t2)
            .returns_once("veridis quo");

        // Act
        mock.work(t1);
        mock.work(t2);
        mock.work(t1);

        // Assert
        assert_panics(
            || mock.work(t1),
            r#"Mock wasn't configured to handle following call:
	<Struct<[u8]> as Trait<i32>>::work(?)"#,
        );
        mock.verify_calls();
    }

    #[test]
    fn Trait_work_NotEnoughCalls_Panics() {
        // Arrange
        Struct::<S>::static_setup().new().call_base();
        let mut mock = Struct::<S>::new();
        let t1 = 10;
        let t2 = "quo vadis";
        mock.setup()
            .as_Trait::<T1>()
            .work(t1)
            .returns(20, 4.times());
        mock.setup()
            .as_Trait::<T2>()
            .work(t2)
            .returns_once("veridis quo");

        // Act
        mock.work(t1);
        mock.work(t2);
        mock.work(t1);
        mock.work(t1);

        // Assert
        assert_panics(
            || mock.verify_calls(),
            r#"Expected following calls:
	<Struct<[u8]> as Trait<i32>>::work(?) 1 time"#,
        );
    }
}
