use automock::*;

#[mock]
trait Trait<'b> {
    fn work<'a: 'b>(&self, v: &'a i32) -> &'a i32;
}

mod tests {
    use super::*;

    #[test]
    fn my_test() {
        // Arrange
        let mut mock = TraitMock::new();
        let v1 = &10;
        let v2 = &20;
        let v3 = &-31;

        let r1 = &111;
        let r2 = &222;
        let r3 = &333;

        mock.setup()
            .work(v1)
            .returns(r1)
            .work(v2)
            .returns(r2)
            .and_does(|_, _| {})
            .work(Arg::is(|x: &&i32| **x < 0))
            .returns(r3);

        // Act
        let actual_r1 = mock.work(v1);
        let actual_r2 = mock.work(v2);
        let actual_r3 = mock.work(v3);

        // Assert
        assert_eq!(r1, actual_r1);
        assert_eq!(r2, actual_r2);
        assert_eq!(r3, actual_r3);

        mock.received()
            .work(v1, Times::Once)
            .work(v2, Times::Once)
            .work(v3, Times::Once)
            .work(Arg::Any, Times::Exactly(3))
            .no_other_calls();
    }
}
