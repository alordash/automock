use std::sync::*;

/// Crate level settings.
pub struct Settings {
    /// Controls how many invalid calls will be listed in case of an error.
    pub max_invalid_calls_listed_count: usize,
}

/// Default crate level settings.
pub const DEFAULT_SETTINGS: Settings = Settings {
    max_invalid_calls_listed_count: 10,
};

/// Crate level settings value.
pub static SETTINGS: LazyLock<RwLock<Settings>> = LazyLock::new(|| RwLock::new(DEFAULT_SETTINGS));

const LOCK_ERROR_MSG: &str = "Unable to lock `automock` settings.";

/// Reads crate level settings, returning read guard.
pub fn read_settings<'a>() -> RwLockReadGuard<'a, Settings> {
    SETTINGS.read().expect(LOCK_ERROR_MSG)
}

/// Writes crate level settings, returning write guard.
pub fn write_settings<'a>() -> RwLockWriteGuard<'a, Settings> {
    SETTINGS.write().expect(LOCK_ERROR_MSG)
}
