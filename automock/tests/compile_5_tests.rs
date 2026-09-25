use automock::*;
use std::fmt::Debug;

#[mock]
trait Trait<'rs, T1> {
    fn work<T2, T3, const B: bool, const N: usize>(&self, t1: T1, t2: &'rs T2) -> T3;
}

#[derive(Clone, Debug)]
struct Foo {
    amogus: f32,
}

mod tests {
    use super::*;

    #[test]
    fn my_test() {
        let mut mock = TraitMock::new();

        let v1 = 11;
        let v2 = 22;
        let v3 = 33;
        let v4 = [10; 5];
        let v5 = 'c';

        mock.setup()
            .work::<_, _, true, 2>(10, &"amogus")
            .returns(v1)
            .and_does(|_, (number, string)| {
                assert_eq!(10, *number);
                assert_eq!("amogus", **string);
            })
            .work::<_, _, true, 4>(10, &"amogus")
            .returns(v2)
            .and_does(|_, _| {})
            .work::<_, _, false, 2>(10, &"amogus")
            .returns(v3)
            .work::<_, _, false, 2>(10, &"amogus")
            .returns(v4)
            .work::<Foo, _, false, 2>(23, Arg::Any)
            .returns(v5);

        let av3 = mock.work::<_, i32, false, 2>(10, &"amogus");
        let av2 = mock.work::<_, i32, true, 4>(10, &"amogus");
        let av1 = mock.work::<_, i32, true, 2>(10, &"amogus");
        let av4 = mock.work::<_, [i32; 5], false, 2>(10, &"amogus");
        let av5 = mock.work::<_, char, false, 2>(23, &Foo { amogus: 53.2f32 });

        assert_eq!(v1, av1);
        assert_eq!(v2, av2);
        assert_eq!(v3, av3);
        assert_eq!(v4, av4);
        assert_eq!(v5, av5);

        mock.received()
            .work::<_, i32, true, 2>(10, &"amogus", Times::Once)
            .work::<_, i32, true, 4>(10, &"amogus", Times::Once)
            .work::<_, i32, false, 2>(10, &"amogus", Times::Once)
            .work::<_, [i32; 5], false, 2>(10, &"amogus", Times::Once)
            .work::<_, i32, true, 2>(10, &"quo vadis", Times::Never)
            .work::<_, i32, true, 4>(11, &"amogus", Times::Never)
            .work::<_, i32, false, 2>(10, &"quo vadis", Times::Never)
            .work::<_, i32, true, 2>(10, &true, Times::Never)
            .work::<Foo, char, false, 2>(
                23,
                Arg::is(|foo: &&Foo| foo.amogus == 53.2f32),
                Times::Once,
            )
            .no_other_calls();
    }
}
