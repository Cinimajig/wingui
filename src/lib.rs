#![allow(dead_code)]

/// Module for helping `COM` and `Windows Runtime` initialization.
#[cfg(feature = "init")] pub mod init;

/// Module for working with unicode-strings
#[cfg(feature = "wstring")] pub mod wstring;

/// Module for helping with Win32 GUI
#[cfg(feature = "window")] pub mod window;

/// Module with some utility functions
#[cfg(feature = "utils")] pub mod utils;

/// Converts a `&str` to a vector of UTF-16 bytes
#[cfg(any(
    feature = "wstring", 
    feature = "window", 
    feature = "utils")
)]
fn get_wide_string(text: &str) -> Vec<u16> {
    use ::std::ffi::OsStr;
    use ::std::os::windows::ffi::OsStrExt;

    OsStr::new(text)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
