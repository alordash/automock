use rsubstitute::*;

#[mock]
#[allow(unused)]
fn f<const N: usize>(_: [&str; N]) {}

mod tests {
    #[test]
    fn compile() {}
}
