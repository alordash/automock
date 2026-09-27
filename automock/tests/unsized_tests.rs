#![allow(clippy::needless_maybe_sized)]
use automock::*;

#[mock]
#[allow(unused)]
fn accept_unsized<T: ?Sized + Clone>(t: &T) {}

#[mock]
struct Struct<T: ?Sized + Clone> {
    #[allow(unused)]
    t: Box<T>,
}

#[mock]
impl<T: ?Sized + Clone> Struct<T> {
    #[allow(unused)]
    fn accept<F: ?Sized + Clone>(&self, _: Box<T>, _: &F) {}
}

#[mock]
trait Trait<U: ?Sized + Clone> {
    #[allow(unused)]
    fn accept<V: ?Sized + Clone>(&self, _: Box<U>, _: &V) {}
}

#[mock]
impl<T: ?Sized + Clone, U: ?Sized + Clone> Trait<U> for Struct<T> {
    fn accept<V: ?Sized + Clone>(&self, _: Box<U>, _: &V) {}
}

mod tests {
    #[test]
    fn compile() {}
}
