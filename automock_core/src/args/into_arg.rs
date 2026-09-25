use crate::args::*;

#[doc(hidden)]
pub trait IntoArg<T> {
    fn into_arg(self, format_debug_string: impl Fn(&T) -> String) -> Arg<T>;
}

impl<T: PartialEq> IntoArg<T> for T {
    fn into_arg(self, format_debug_string: impl Fn(&T) -> String) -> Arg<T> {
        let print_arg = format_debug_string(&self);
        let arg_cmp = ArgCmp::new_eq(self, print_arg);
        return Arg::Eq(arg_cmp, Internal);
    }
}

impl<T> IntoArg<T> for Arg<T> {
    fn into_arg(mut self, format_debug_string: impl Fn(&T) -> String) -> Arg<T> {
        match &mut self {
            Arg::Eq(arg_cmp, _) | Arg::NotEq(arg_cmp, _) => {
                let print_arg = format_debug_string(arg_cmp.value());
                arg_cmp.set_print_arg(print_arg);
            }
            _ => (),
        }
        self
    }
}
