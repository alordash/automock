/// This is marker macro used to transmute ONLY lifetimes.
/// `Src` and `Dst` passed to [`core::mem::transmute`] from this macro must always have the same
/// type differing only in lifetime, e.g. `Src: 'a, Dst: 'b`.
///
/// This macro is used to verify that transmutations in this crate are only used to convert lifetimes.
///
/// # Safety:
/// Mock objects store arguments passed to them for their entire lifetime. rsubstitute follows
/// assumption that mock object should not outlive arguments passed to it. This requires treating
/// all types and references as having a special lifetime [`rsubstitute_proc_macro::constants::DEFAULT_ARG_LIFETIME`]
/// that corresponds to mock object lifetime.
///
/// In order to keep source lifetime constraints as is in mocked traits/structs/functions and to use
/// [`DEFAULT_ARG_LIFETIME`] internally, lifetimes transmutation is used. Without it a trait
/// `trait Trait { fn_info work<'a>(&'a self) -> &'a i32; }` will have in its mock extra generic
/// constraint: `trait Trait<'rs> { fn_info work<'a: 'rs>(&'a self) -> &'a i32; }`
///
/// For user it means that you just should keep your arguments alive for the duration of mock object.
/// Not doing so will result in Undefined behavior as mock object will try to check whether given
/// argument is suitable for some configuration. If this argument was a reference to a value that
/// was dropped, checker function will get a dangling reference.
///
/// For example, this leads to UB:
/// ```ignore
/// let mock = Mock::new();
/// {
///     let v = 1;
///     let r = &v;
///     mock.work(r);
/// }
/// // `Arg::is` will receive a dangling reference.
/// mock.received.work(Arg::is(|v| *v > 0, Times::once);
/// ```
/// To fix it either move `v` to the same or higher scope where `mock` is defined:
/// ```ignore
/// let mock = Mock::new();
/// let v = 1;
/// let r = &v;
/// mock.work(r);
/// ```
/// or call `mock.received` only when all arguments are alive:
/// ```ignore
/// let mock = Mock::new();
/// {
///     let v = 1;
///     let r = &v;
///     mock.work(r);
///     // `Arg::is` will receive a valid reference.
///     mock.received.work(Arg::is(|v| *v > 0, Times::once);
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! transmute_lifetime {
    ($expr:expr) => {
        unsafe {
            #[allow(clippy::missing_transmute_annotations)]
            let result = core::mem::transmute_copy(&core::mem::ManuallyDrop::new($expr));
            result
        }
    };
    // If `$ty` is used as generic param in `core::mem::transmute` then source type is highlighted
    // as being part of unsafe block. Using `let` binding to circumvent that.
    ($expr:expr, $ty:ty) => {{
        unsafe {
            #[allow(clippy::missing_transmute_annotations)]
            let result: $ty = core::mem::transmute_copy(&core::mem::ManuallyDrop::new($expr));
            result
        }
    }};
}
