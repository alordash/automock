#![allow(clippy::needless_maybe_sized)]
use automock::*;

#[mock(base)]
#[allow(unused)]
fn accept_unsized<T: ?Sized + Clone>(t: &T) {}

#[mock]
struct Struct<T: ?Sized + Clone> {
    #[allow(unused)]
    t: Box<T>,
}

#[mock(base)]
impl<T: ?Sized + Clone> Struct<T> {
    #[allow(unused)]
    fn accept<F: ?Sized + Clone>(&self, _: Box<T>, _: &F) {}
}

#[mock(base)]
trait Trait<U: ?Sized + Clone> {
    #[allow(unused)]
    fn accept<V: ?Sized + Clone>(&self, _: Box<U>, _: &V) {}
}

#[mock(base)]
impl<T: ?Sized + Clone, U: ?Sized + Clone> Trait<U> for Struct<T> {
    fn accept<V: ?Sized + Clone>(&self, _: Box<U>, _: &V) {}
}

mod tests {
    #[test]
    fn compile() {}
}
