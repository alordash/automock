use crate::args::*;

#[doc(hidden)]
pub trait IntoArg<T> {
    fn into_arg(self, format_debug_string: impl Fn(&T) -> String) -> Arg<T>;
}

impl<T: PartialEq> IntoArg<T> for T {
    fn into_arg(self, format_debug_string: impl Fn(&T) -> String) -> Arg<T> {
        let arg_cmp = ArgCmp {
            print_arg: format_debug_string(&self),
            value: Box::new(self),
            comparator: PartialEq::eq,
            maybe_deref_info: None,
        };
        return Arg::PrivateEq(arg_cmp, Internal);
    }
}

impl<T> IntoArg<T> for Arg<T> {
    fn into_arg(mut self, format_debug_string: impl Fn(&T) -> String) -> Arg<T> {
        match &mut self {
            Arg::PrivateEq(arg_cmp, _) | Arg::PrivateNotEq(arg_cmp, _) => {
                let print_arg = format_debug_string(arg_cmp.value.as_ref());
                arg_cmp.set_print_arg(print_arg);
            }
            _ => (),
        }
        self
    }
}
