use automock::*;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub number: Vec<i32>,
}

#[mock]
struct Struct;

#[mock(base)]
impl Struct {
    fn new() -> Self {
        Self
    }

    #[allow(unused)]
    fn foo(
        &mut self,
        Foo { mut number }: Foo,
        #[allow(unused_variables)] mut qq: &mut &mut &&&&mut i32,
    ) {
    }
}

#[mock(base)]
impl From<i32> for Struct {
    fn from(_: i32) -> Self {
        Self::new()
    }
}

impl From<f32> for Struct {
    fn from(_: f32) -> Self {
        Self::new()
    }
}

#[mock]
struct Selfish;

#[mock(base)]
impl Selfish {
    #[allow(unused)]
    fn work(&self, _: &Self) {}
}

mod tests {
    #[test]
    fn compile() {}
}
