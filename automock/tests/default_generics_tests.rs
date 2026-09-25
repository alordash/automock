use automock::*;

#[mock(base)]
#[allow(unused)]
trait Trait<TT = i32, const NT: usize = 1> {
    fn work<QT, const MT: usize>(&self);
    fn work_base<QT, const MT: usize>(&self) {}
    fn static_work<QT, const MT: usize>();
    fn static_work_base<QT, const MT: usize>() {}
}

#[mock]
#[allow(unused)]
struct Struct<TS = i32, const NS: usize = 2> {
    payload: [TS; NS],
}

#[mock(base)]
impl<TS, const NS: usize> Struct<TS, NS> {
    fn work<QS, const MS: usize>(&self) {}
    fn static_work<QS, const MS: usize>() {}
}

#[mock(base)]
impl<TT, const NT: usize, TS, const NS: usize> Trait<TT, NT> for Struct<TS, NS> {
    fn work<QT, const MT: usize>(&self) {}
    fn work_base<QT, const MT: usize>(&self) {}
    fn static_work<QT, const MT: usize>() {}
    fn static_work_base<QT, const MT: usize>() {}
}

mod tests {
    #[test]
    fn compile() {}
}
