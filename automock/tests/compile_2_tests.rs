#![allow(unused)]

use automock::*;

#[mock(base)]
trait Trait {
    fn by_box(self: Box<Self>) {}
}

#[mock]
trait Worker {
    fn work() {}
}

fn f<T: Worker>() {
    T::work();
}

#[mock]
struct Struct {
    pub v: i32,
}

#[mock(base)]
impl Struct {
    pub fn new(v: i32) -> Self {
        Self { v }
    }

    fn struct_refs(&self) {
        let s = Struct { v: 1 };
        let Struct { v: a } = s;

        let s = Struct { v: 1 };
        let Struct { v: b } = s;
    }

    pub fn f(&self) {}

    pub fn work() {}
}

#[test]
fn compile() {}
