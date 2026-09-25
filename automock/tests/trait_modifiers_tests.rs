#![allow(clippy::missing_safety_doc)]
use automock::*;

#[mock]
#[allow(unused)]
unsafe trait UnsafeTrait {
    fn work();
}

#[mock(base)]
#[allow(unused)]
unsafe trait UnsafeTraitBase {
    fn work() {}
}

#[mock]
struct Struct;

#[mock]
unsafe impl UnsafeTrait for Struct {
    fn work() {}
}

#[mock(base)]
unsafe impl UnsafeTraitBase for Struct {
    fn work() {}
}

mod tests {
    #[test]
    fn compile() {}
}
