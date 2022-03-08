use std::{ffi::c_void, ptr};

use crate::wstring::WideString;

use super::*;

#[test]
fn wide_str() {
    let wide = wstring::WideString::from("Hello world!");
    let wstr = wstring::WideStr::from(wide.ptr());

    println!("'{}'", wstr.to_string());
    println!("{:?}", wstr);
}

#[test]
fn lib() {
    type MsgBoxProc = extern "system" fn(*const c_void, *const u16, *const u16, i32);

    let user32 = utils::Library::load("User32.dll").unwrap();
    let func: utils::FnWrapper<MsgBoxProc> = user32.load_func("MessageBoxW");

    assert!(func.is_valid());
    println!("MsgBox address: {}", &func);

    let msgbox = func.0.unwrap();
    let msg = WideString::from("Hello from a dynamic library loader!");

    msgbox(ptr::null(), msg.ptr(), 0 as _, 0);
}

