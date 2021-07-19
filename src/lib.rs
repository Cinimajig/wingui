/// Module for helping `COM` and `Windows Runtime` initialization.
#[cfg(feature = "init")] pub mod init;

/// Module for working with unicode-strings
#[cfg(feature = "wstring")] pub mod wstring;

/// Module for helping with Win32 GUI
#[cfg(feature = "wstring")] pub mod window;

use ::std::{ffi::c_void, ptr};

/// A function for showing a `MessageBox`
///
/// For documenttation on `mb_type` values, look at the documentation at
/// [https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw]
pub fn msgbox(text: &str, title: Option<&str>, mb_type: u32) -> MBResult {
    let wtext = get_wide_string(text);

    unsafe {
        match title {
            Some(s) => {
                let wtitle = get_wide_string(s);
                MessageBoxW(ptr::null_mut(), wtext.as_ptr(), wtitle.as_ptr(), mb_type)
            },
            None => MessageBoxW(ptr::null_mut(), wtext.as_ptr(), ptr::null(), mb_type)
        }
    }
}

/// `MBResult` is the return type of the `msgbox` function
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum MBResult {
    Error = 0,
    Ok = 1,
    Cancel = 2,
    Abort = 3,
    Retry = 4,
    Ignore = 5,
    Yes = 6,
    No = 7,
    TryAgain = 10,
    Continue = 11
}

/// Converts a `&str` to a vector of UTF-16 bytes
fn get_wide_string(text: &str) -> Vec<u16> {
    use ::std::ffi::OsStr;
    use ::std::os::windows::ffi::OsStrExt;

    OsStr::new(text)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

#[link(name = "User32")]
extern "system" {
    fn MessageBoxW(hWnd: *mut c_void, lpText: *const u16, lpCaption: *const u16, uType: u32) -> MBResult;
}
