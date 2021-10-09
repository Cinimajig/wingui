use proc_wstring::wstr;
use winutils::wstring::WideString;

struct TESTSTRUCTW {
    font_name: [u16; 32],
}

fn main() {
    let w_struct = TESTSTRUCTW {
        font_name: wstr!("Hello world!", 32),
    };

    let w_string = WideString::from_str_with_size("Hello world!", 32);

    println!("{:?}", &w_struct.font_name);
    println!("{:?}", &w_string.bytes);
}
