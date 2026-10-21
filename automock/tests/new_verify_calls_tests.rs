use automock::*;

#[mock]
fn work(a: i32, b: i32) -> i32 {
    a + b + 1
}

#[mock]
fn gg<T>(t: T) -> T {t}

#[mock]
trait Trait {
    fn foo(&self, v: i32) -> i32;
    fn bar(&self, b: bool) -> bool;

    fn static_foo(v: i32) -> i32;
    fn static_bar(b: bool) -> bool;
}

#[mock]
struct Struct;

#[mock]
impl Struct {
    pub fn new() -> Self {
        Self
    }

    fn foo(&self, v: i32) -> i32 {
        v
    }
    fn bar(&self, b: bool) -> bool {
        b
    }

    fn static_foo(v: i32) -> i32 {
        v
    }
    fn static_bar(b: bool) -> bool {
        b
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn static_fn_Ok() {
        // Arrange
        let a = 10;
        let b = 15;
        let result = 20;
        work::setup(a, b)
            .returns_once(result)
            .setup(a + 1, b + 11)
            .returns_many([result + 1, 6, 8, 1])
            .setup(a + 2, Arg::is(|x| *x > 5))
            .returns_once(result + 2)
            .setup(a + 1, Arg::not_eq(b + 11))
            .returns_once(result + 1);

        // TODO - target API?
        // work::setup(1).returns_once(2)
        //      .setup(3).returns(4, 5.times());

        // Act
        let actual_result = work(a, b);

        // Assert
        assert_eq!(actual_result, result);

        // work::received(v, 1.time());
        work::verify_calls();
    }

    #[test]
    fn object_mock_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        let v = 10;
        let b = false;
        let foo_result = 20;
        let bar_result = true;
        let sequence = in_sequence(|| {
            mock.setup()
                .foo(v)
                .returns_once(foo_result)
                .foo(102)
                .returns_once(202);
            TraitMock::static_setup()
                .static_foo(44)
                .returns_once(11)
                .static_bar(true)
                .returns_once(false);
            mock.setup()
                .bar(b)
                .returns_many([true, false, true, false])
                .bar(b)
                .returns_once(bar_result);
        });

        // TODO - target API?
        // mock.setup()
        //     .work(1).returns_once(2)
        //     .work(3).returns(4, 5.times());

        // Act
        let actual_bar_result = mock.bar(b);
        let actual_foo_result = mock.foo(v);

        // Assert
        assert_eq!(actual_foo_result, foo_result);
        assert_eq!(actual_bar_result, bar_result);

        TraitMock::verify_static_calls();
        // mock.verify_calls();

        // sequence.verify();
    }

    #[test]
    fn struct_mock_Ok() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let sequence = in_sequence(|| {
            mock.setup().foo(554).returns_once(123).foo(23950).returns_once(3);
            Struct::static_setup()
                .static_foo(5)
                .returns_once(55)
                .static_bar(true)
                .returns_once(false);
            mock.setup()
                .bar(true)
                .returns_once(false)
                .bar(false)
                .returns_once(false);
        });

        // Act
        let first = mock.foo(554);
        let second = mock.bar(true);

        // Assert
        Struct::verify_static_calls();
    }
    
    #[test]
    fn gg_Ok() {
        // Arrange
        gg::setup(1).returns_once(2);
        
        // Act
        // assert_eq!(gg(1), 2);
        
        // Assert
        gg::verify_calls::<i32>();
    }
}

/*
Expected following calls:
     work((i32) == 11, fn(i32)), 1 time
     work(fn(i32), fn(i32)),     2 times
 */
