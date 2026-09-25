#![allow(clippy::extra_unused_lifetimes)]
use automock::*;

#[mock]
fn accept_ref<'a>(r: &'a i32) -> &'a i32 {
    unreachable!()
}

#[mock]
#[allow(unused)]
fn return_mut_ref<'a>() -> &'a mut i32 {
    unreachable!()
}

#[mock]
#[allow(unused)]
fn accept_mut_ref<'a>(mut r: i32) -> i32 {
    r += 1;
    return r;
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub number: Vec<i32>,
}

#[mock]
#[allow(unused)]
fn accept_foo(Foo { mut number }: Foo) {
    let q = Foo { number };
}

#[mock]
fn accept_many_ref<'a, 'b>(mut r: &'a &'b &'a &i32, _em: &()) -> &'a &'b &'a &'b i32 {
    unreachable!()
}

#[mock]
#[allow(unused)]
fn accept_impl_trait(_: impl AsRef<str>) {}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn compile() {}

    #[test]
    fn accept_ref_test() {
        let v1 = 1;
        let v2 = 2;
        let r1 = &v1;
        {
            let r2 = &v2;

            accept_ref::setup(r1).returns(r2);

            accept_ref(r1);

            accept_ref::received(r1, Times::Once).no_other_calls();
        }
    }

    #[test]
    fn accept_many_ref_test() {
        let v1 = 11;
        let r1 = &&&&v1;
        {
            let v2 = 23;
            let r2 = &&&&v2;
            accept_many_ref::setup(r2, Arg::Any).returns(r1);

            let r = accept_many_ref(r2, &());

            assert_eq!(r1, r);

            accept_many_ref::received(r2, Arg::Any, Times::Once).no_other_calls();
        }
    }
}
