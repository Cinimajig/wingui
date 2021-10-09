#![allow(dead_code)]

extern crate proc_macro;
use ::std::{ffi::OsStr, iter, os::windows::prelude::OsStrExt};

use proc_macro::TokenStream;

/// A macro to help with creating static 16-bit Unicode byte arrays.
/// It's meant to be used with struct-declarations with the Windows API.
///
/// The first argument is a string-slice and is always required.
///
/// The second argument is optional and is the size that the array should be.
///
/// # Example
/// ```
/// #[macro_use]
/// use proc_wstring::wstr;
///
/// struct TESTSTRUCTW { font_name: [u16; 32] }
///
/// let font = TESTSTRUCTW {
///     font_name: wstr!("Segoe UI", 32),
/// };
/// ```
#[proc_macro]
pub fn wstr(item: TokenStream) -> TokenStream {
    let input: String = item.to_string();
    let args: Vec<&str> = input.split(",").collect();

    let mut wstr: Vec<u16> = OsStr::new(&args[0][1..args[0].len() - 1])
        .encode_wide()
        .chain(iter::once(0))
        .collect();

    if let Some(size) = args.get(1) {
        let val: usize = size.trim().parse().unwrap();
        wstr.resize(val, 0_u16);
    }

    format!("{:?}", &wstr).parse().unwrap()
}
