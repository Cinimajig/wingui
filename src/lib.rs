#![allow(dead_code)]

/// Module for helping `COM` and `Windows Runtime` initialization.
///
/// `Windows Runtime` initialization is only available with the
/// winapi-crate feature enabled (linking issue)
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

#[cfg(test)]
mod test {
    use crate::window::Window;

    use super::*;
    use wstring::WideString;
    use window::Windowing;

    #[derive(Default)]
    struct Data {
        age: i32,
    }

    #[test]
    fn test_wstring() {
        let mut a = WideString::from("Hello ");
        a.push_str("world");
        a.push_wide(&WideString::from("!"));

        assert!(a.to_string().len() < a.bytes.len());
    }

    #[test]
    fn window() {
        let mut wnd = window::Window::new(());
        wnd.register("class_name");
        wnd.create_window("TEST", 360, 120);
        wnd.show();

        wnd.run();
    }
}
