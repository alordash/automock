use automock::*;
use std::marker::PhantomData;

mod common;

#[mock]
struct Struct<S: ?Sized, const CS: usize> {
    s: PhantomData<S>,
}

#[mock]
impl<S: ?Sized, const CS: usize> Struct<S, CS> {
    pub fn new() -> Self {
        Self { s: PhantomData }
    }
}

trait Trait<T, const CT: usize> {
    fn work(&self, t: T) -> T;
}

#[mock]
impl<T, S: ?Sized, const CS: usize, const CT: usize> Trait<T, CT> for Struct<S, CS> {
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
    const CS: usize = 3;
    const CT: usize = 5;

    #[test]
    fn Trait_work_DifferentTraitGenerics_Ok() {
        Struct::<S, CS>::static_setup().new().call_base();
        let mut mock = Struct::<S, CS>::new();
        let t1 = 10;
        let t1_result = 20;
        let t2 = "quo vadis";
        let t2_result = "veridis quo";
        mock.setup()
            .as_Trait::<T1, CT>()
            .work(t1)
            .returns_once(t1_result);
        mock.setup()
            .as_Trait::<T2, CT>()
            .work(t2)
            .returns_once(t2_result);

        // Act
        let actual_t1_result = Trait::<_, CT>::work(&mock, t1);
        let actual_t2_result = Trait::<_, CT>::work(&mock, t2);

        // Assert
        assert_eq!(actual_t1_result, t1_result);
        assert_eq!(actual_t2_result, t2_result);
        mock.verify_calls();
    }

    #[test]
    fn Trait_work_TooManyCalls_Panics() {
        // Arrange
        Struct::<S, CS>::static_setup().new().call_base();
        let mut mock = Struct::<S, CS>::new();
        let t1 = 10;
        let t2 = "quo vadis";
        mock.setup()
            .as_Trait::<T1, CT>()
            .work(t1)
            .returns(20, 2.times());
        mock.setup()
            .as_Trait::<T2, CT>()
            .work(t2)
            .returns_once("veridis quo");

        // Act
        Trait::<_, CT>::work(&mock, t1);
        Trait::<_, CT>::work(&mock, t2);
        Trait::<_, CT>::work(&mock, t1);

        // Assert
        let t1_arg_string = common::debug_string(format!("{t1:?}"));
        assert_panics(
            || Trait::<_, CT>::work(&mock, t1),
            format!(
                r#"Mock wasn't configured to handle following call:
	<Struct<[u8], 3> as Trait<i32, 5>>::work({t1_arg_string})"#
            ),
        );
        mock.verify_calls();
    }

    #[test]
    fn Trait_work_NotEnoughCalls_Panics() {
        // Arrange
        Struct::<S, CS>::static_setup().new().call_base();
        let mut mock = Struct::<S, CS>::new();
        let t1 = 10;
        let t2 = "quo vadis";
        mock.setup()
            .as_Trait::<T1, CT>()
            .work(t1)
            .returns(20, 4.times());
        mock.setup()
            .as_Trait::<T2, CT>()
            .work(t2)
            .returns_once("veridis quo");

        // Act
        Trait::<_, CT>::work(&mock, t1);
        Trait::<_, CT>::work(&mock, t2);
        Trait::<_, CT>::work(&mock, t1);
        Trait::<_, CT>::work(&mock, t1);

        // Assert
        let t1_arg_string = common::debug_string(format!("{t1:?}"));
        assert_panics(
            || mock.verify_calls(),
            format!(
                r#"Expected following calls:
	<Struct<[u8], 3> as Trait<i32, 5>>::work({t1_arg_string}) 1 time"#
            ),
        );
    }
}
