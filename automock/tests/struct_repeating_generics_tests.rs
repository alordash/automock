use automock::*;
use std::marker::PhantomData;

macro_rules! define_marker_traits {
    ($($names:ident),*) => { $(#[allow(unused)] trait $names {} impl<T> $names for T {})* };
}

define_marker_traits!(M1, M2, M3, M4);

#[allow(unused)]
trait ILifetime<'x: 'c, 'a, 'b, 'c, 'd>
where
    'x: 'd,
{
}

#[allow(unused)]
trait IType<T: M3>
where
    T: M4,
{
}

#[allow(unused)]
trait IConst<const C: usize> {}

#[mock]
pub struct Lifetime<'x: 'a, 'a, 'b, 'c, 'd>
where
    'x: 'b,
{
    phantom: PhantomData<&'x &'a &'b &'c &'d ()>,
}

#[mock(base)]
impl<'x: 'a + 'c, 'a, 'b, 'c, 'd> Lifetime<'x, 'a, 'b, 'c, 'd> where 'x: 'b + 'd {}

#[mock(base)]
impl<'x: 'a + 'c, 'a, 'b, 'c, 'd> ILifetime<'x, 'a, 'b, 'c, 'd> for Lifetime<'x, 'a, 'b, 'c, 'd> where
    'x: 'b + 'd
{
}

#[allow(unused)]
struct Type<T: M1>(PhantomData<T>)
where
    T: M2;

#[allow(unused)]
struct Const<const C: usize>;

mod tests {
    #[test]
    fn compile() {}
}
