#![allow(dead_code)]

use std::mem;
use ::std::{ffi::c_void, io, ptr};
use crate::get_wide_string;

/// Retrieves information about the current user.
/// The function fails, if you retrieve information, which is not available.
///
/// All posible values for `name_format` is defined as constants starting with `NAME_`
/// 
/// See more at https://docs.microsoft.com/en-us/windows/win32/api/secext/ne-secext-extended_name_format
///
/// to learn about them
pub fn get_user_info(name_format: u32) -> io::Result<String> {
    let mut buffer = [0_u16; 260];
    let mut size = 260;

    unsafe {
        if GetUserNameExW(name_format, buffer.as_mut_ptr(), &mut size) == 0 {
            return Err(io::Error::last_os_error());
        }
    }
    
    Ok(String::from_utf16_lossy(&buffer[..size as usize]))
}

/// Retrieves information about the computer.
/// The function fails, if you retrieve information, which is not available.
///
/// All posible values for `computer_format` is defined as constants starting with `COMPUTER_`
/// 
/// See more at https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ne-sysinfoapi-computer_name_format
///
/// to learn about them
pub fn get_computer_info(computer_format: u32) -> io::Result<String> {
    let mut buffer = [0_u16; 260];
    let mut size = 260;

    #[cfg(not(feature = "winapi-crate"))]
    unsafe {
        if GetComputerNameExW(computer_format, buffer.as_mut_ptr(), &mut size) == 0 {
            return Err(io::Error::last_os_error());
        }
    }

    #[cfg(feature = "winapi-crate")]
    unsafe {
        if winapi::um::sysinfoapi::GetComputerNameExW(computer_format, buffer.as_mut_ptr(), &mut size) == 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(String::from_utf16_lossy(&buffer[..size as usize]))
}

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

                #[cfg(not(feature = "winapi-crate"))]
                return MessageBoxW(ptr::null_mut(), wtext.as_ptr(), wtitle.as_ptr(), mb_type);

                #[cfg(feature = "winapi-crate")]
                return mem::transmute(winapi::um::winuser::MessageBoxW(ptr::null_mut(), wtext.as_ptr(), wtitle.as_ptr(), mb_type));
            },
            None => {
                #[cfg(not(feature = "winapi-crate"))]
                return MessageBoxW(ptr::null_mut(), wtext.as_ptr(), ptr::null(), mb_type);

                #[cfg(feature = "winapi-crate")]
                return mem::transmute(winapi::um::winuser::MessageBoxW(ptr::null_mut(), wtext.as_ptr(), ptr::null(), mb_type));
            }
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

pub const NAME_UNKNOWN: u32 = 0;
pub const NAME_FULLY_QUALIFIED_DN: u32 = 1;
pub const NAME_SAM_COMPATIBLE: u32 = 2;
pub const NAME_DISPLAY: u32 = 3;
pub const NAME_UNIQUE_ID: u32 = 6;
pub const NAME_CANONICAL: u32 = 7;
pub const NAME_USER_PRINCIPAL: u32 = 8;
pub const NAME_CANONICAL_EX: u32 = 9;
pub const NAME_SERVICE_PRINCIPAL: u32 = 10;
pub const NAME_DNS_DOMAIN: u32 = 12;
pub const NAME_GIVEN_NAME: u32 = 13;
pub const NAME_SURNAME: u32 = 14;

pub const COMPUTER_NAME_NET_BIOS: u32 = 0;
pub const COMPUTER_NAME_DNS_HOSTNAME: u32 = 1;
pub const COMPUTER_NAME_DNS_DOMAIN: u32 = 2;
pub const COMPUTER_NAME_DNS_FULLY_QUALIFIED: u32 = 3;
pub const COMPUTER_NAME_PHYSICAL_NET_BIOS: u32 = 4;
pub const COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME: u32 = 5;
pub const COMPUTER_NAME_PHYSICAL_DNS_DOMAIN: u32 = 6;
pub const COMPUTER_NAME_PHYSICAL_DNS_FULLY_QUALIFIED: u32 = 7;
pub const COMPUTER_NAME_MAX: u32 = 8;

#[cfg(not(feature = "winapi-crate"))]
#[link(name = "User32")]
extern "system" {
    fn MessageBoxW(hWnd: *mut c_void, lpText: *const u16, lpCaption: *const u16, uType: u32) -> MBResult;
}

#[cfg(not(feature = "winapi-crate"))]
#[link(name = "Kernel32")]
extern "system" {
    /* https://docs.microsoft.com/da-dk/windows/win32/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw */
    fn GetComputerNameExW(NameType: u32, lpBuffer: *const u16, nSize: *mut u32) -> i32;
}

#[link(name = "Secur32")]
extern "system" {
    /* https://docs.microsoft.com/en-us/windows/win32/api/secext/nf-secext-getusernameexW */
    fn GetUserNameExW(NameFormat: u32, lpNameBuffer: *const u16, nSize: *mut u32) -> i32;
}
