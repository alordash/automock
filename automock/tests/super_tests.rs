#[derive(Clone)]
struct Dependency;

mod inner {
    use automock::*;

    #[mock(base)]
    #[allow(unused)]
    fn work(_: super::Dependency) -> super::Dependency {
        use super::Dependency;
        let result = Dependency;
        return result;
    }

    #[mock]
    #[allow(unused)]
    fn work_impl_trait(_: impl AsRef<super::Dependency>) -> &'static super::Dependency {
        use super::Dependency;
        let result = Dependency;
        unreachable!()
    }
}

mod tests {
    #[test]
    fn compile() {}
}
