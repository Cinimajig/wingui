/// Module for helping `COM` and `Windows Runtime` initialization.
pub mod init;

/// Module for working with unicode-strings
pub mod wstring;

use ::std::ffi::c_void;
use ::std::ptr;
use wstring::WideString;

/// A function for showing a `MessageBox`
///
/// For documenttation on `mb_type` values, look at the documentation at
/// [https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw]
pub fn msgbox(text: &str, title: Option<&str>, mb_type: u32) -> MBResult {
    let wtext = WideString::from(text);

    unsafe {
        match title {
            Some(s) => {
                let wtitle = WideString::from(s);
                MessageBoxW(ptr::null_mut(), wtext.ptr(), wtitle.ptr(), mb_type)
            },
            None => MessageBoxW(ptr::null_mut(), wtext.ptr(), ptr::null(), mb_type)
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

#[link(name = "User32")]
extern "system" {
    fn MessageBoxW(hWnd: *mut c_void, lpText: *const u16, lpCaption: *const u16, uType: u32) -> MBResult;
}
