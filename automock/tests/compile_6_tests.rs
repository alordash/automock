use automock::*;

#[mock]
trait Trait<'am> {
    fn work<'a: 'am>(&self, v: &'a i32) -> &'a i32;
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
            .returns_once(r1)
            .work(v2)
            .returns_once(r2)
            .and_does(|_, (_,)| {})
            .work(Arg::is(|x: &&i32| **x < 0))
            .returns_once(r3);

        // Act
        let actual_r1 = mock.work(v1);
        let actual_r2 = mock.work(v2);
        let actual_r3 = mock.work(v3);

        // Assert
        assert_eq!(r1, actual_r1);
        assert_eq!(r2, actual_r2);
        assert_eq!(r3, actual_r3);
        mock.verify_calls();
    }
}
