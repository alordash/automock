#![allow(clippy::missing_safety_doc)]
use automock::*;

#[mock]
#[allow(unused)]
unsafe trait UnsafeTrait {
    fn work();
}

#[mock]
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

#[mock]
unsafe impl UnsafeTraitBase for Struct {
    fn work() {}
}

mod tests {
    #[test]
    fn compile() {}
}
