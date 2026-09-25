use std::fmt::Debug;

const UNKNOWN_ARG_STRING: &str = "?";

// Inspired by mockall's way of printing arguments values:
// https://github.com/asomers/mockall/blob/4401e5ac4aa7b05227c157f569d1147d732944b0/mockall/src/lib.rs#L1496
#[doc(hidden)]
pub struct ArgPrinter<'a, T: ?Sized>(pub &'a T);

#[doc(hidden)]
pub mod arg_printing {
    use super::*;

    cfg_if::cfg_if! {
        if #[cfg(feature = "debug_naming")] {
            pub use specialization_printing::*;
            mod specialization_printing {
                use super::*;

                pub trait IDebugPrinter {
                    fn debug_string(&self) -> String;
                }

                impl<'a, T: ?Sized> IDebugPrinter for ArgPrinter<'a, T> {
                    #[allow(unstable_features)]
                    #[allow(incomplete_features)]
                    default fn debug_string(&self) -> String {
                        UNKNOWN_ARG_STRING.to_owned()
                    }
                }

                impl<'a, T: Debug + ?Sized> IDebugPrinter for ArgPrinter<'a, T> {
                    fn debug_string(&self) -> String {
                        format!("{:?}", self.0)
                    }
                }
            }
        } else {
            pub use default_printing::*;
            mod default_printing {
                use super::*;

                pub trait IDebugArgPrinter {
                    fn debug_string(&self) -> String;
                }

                pub trait IUnknownArgPrinter {
                    fn debug_string(&self) -> String;
                }

                impl<'a, T: Debug + ?Sized> IDebugArgPrinter for ArgPrinter<'a, T> {
                    fn debug_string(&self) -> String {
                        format!("{:?}", self.0)
                    }
                }

                impl<'a, T: ?Sized> IUnknownArgPrinter for &ArgPrinter<'a, T> {
                    fn debug_string(&self) -> String {
                        UNKNOWN_ARG_STRING.to_owned()
                    }
                }
            }
        }
    }
}
