use automock::*;

#[mock(base)]
#[allow(unused)]
trait Trait<TA, const TB: usize, TC, const TD: usize = 3> {}

#[mock]
struct S<SA, const SB: usize, SC, const SD: usize = 2> {
    #[allow(unused)]
    first: [SA; SB],
    #[allow(unused)]
    second: [SC; SD],
}

#[mock(base)]
impl<TA, const TB: usize, TC, SA, const SB: usize, SC> Trait<TA, TB, TC> for S<SA, SB, SC> {}

mod tests {
    #[test]
    fn compile() {}
}
