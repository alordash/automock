use automock::*;

#[mock]
trait Trait<'am> {
    fn work(&self, v: i32) -> i32;
}

mod tests {
    use super::*;

    #[test]
    fn my_test() {
        // Arrange
        let mut mock = TraitMock::new();
        let v1 = 10;
        let v2 = 20;
        let v3 = -31;
        let v4 = 1919;
        let v5 = 2022;

        let r1 = 111;
        let r2 = 222;
        let r3 = 333;
        let r45 = 50505;

        mock.setup()
            .work(v1)
            .returns_once(r1)
            .work(v2)
            .returns_once(r2)
            .work(Arg::is(|x| *x < 0))
            .returns_once_with(move |(args,)| r3 + *args)
            .and_does(|_, (_,)| {})
            .work(v4)
            .returns_once(r45)
            .work(v5)
            .returns_once(r45);

        // Act
        let actual_r1 = mock.work(v1);
        let actual_r2 = mock.work(v2);
        let actual_r3 = mock.work(v3);
        let actual_r4 = mock.work(v4);
        let actual_r5 = mock.work(v5);

        // Assert
        assert_eq!(r1, actual_r1);
        assert_eq!(r2, actual_r2);
        assert_eq!(r3 + v3, actual_r3);
        assert_eq!(r45, actual_r4);
        assert_eq!(r45, actual_r5);
        mock.verify_calls();
    }
}
