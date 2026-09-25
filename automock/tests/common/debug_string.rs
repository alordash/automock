#[allow(unused)]
pub fn debug_string(
    #[cfg_attr(not(feature = "debug_naming"), allow(unused))] t: impl AsRef<str>,
) -> String {
    #[cfg(feature = "debug_naming")]
    return t.as_ref().to_owned();
    #[cfg(not(feature = "debug_naming"))]
    return "?".to_owned();
}
