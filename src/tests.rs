use super::*;

#[test]
fn wide_str() {
    let wide = wstring::WideString::from("Hello world!");
    let wstr = wstring::WideStr::from(wide.ptr());

    println!("'{}'", wstr.to_string());
    println!("{:?}", wstr);
}

