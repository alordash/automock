#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use automock::*;

#[mock(base)]
trait Trait<T1: Clone> {
    fn ok(&self) -> i32;
    fn ok_static() -> i32;
    fn f(&self, v: i32) {
        let ok_v = self.ok();
    }
    fn f_static(v: i32) {
        let ok_v = Self::ok_static();
    }

    fn gg<T2>(&self, t1: T1, t2: T2);

    fn gg_static<T2>(t1: T1, t2: T2);
}

#[test]
fn compile() {}
