use automock::*;

#[allow(unused)]
trait Trait<T> {
    fn work(&self, t: T) -> T;

    fn static_work(t: T) -> T;
}

#[mock]
struct Struct;

#[mock(base)]
impl Struct {}

#[mock(base)]
impl<T: Clone> Trait<T> for Struct {
    fn work(&self, t: T) -> T {
        t
    }

    fn static_work(t: T) -> T {
        t
    }
}

mod tests {
    #[test]
    fn compile() {}
}
