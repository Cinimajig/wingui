#![allow(dead_code)]

use ::std::fmt;
use crate::get_wide_string;

/// A struct for making working with unicode-strings easier. 
/// It implements the `Display` trait, so you can always get 
/// a normal String back from a WideString
///
/// ## Examples
/// ```
/// // Creating a WideString
/// let wstring = WideString::from("Hello world!");
///
/// ```
#[derive(Debug)]
pub struct WideString {
    pub bytes: Vec<u16>,
}

impl From<&str> for WideString {
    fn from(text: &str) -> Self {
        Self {
            bytes: get_wide_string(text),
        }
    }
}

impl Default for WideString {
    fn default() -> Self {
        Self { bytes: vec![0_u16] }
    }
}

impl WideString {
    /// Returns a raw pointer to the vector's buffer.
    ///
    /// The same as `WideString.bytes.as_ptr()`
    #[inline]
    pub fn ptr(&self) -> *const u16 {
        self.bytes.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    ///
    /// The same as `WideString.bytes.as_mut_ptr()`
    #[inline]
    pub fn mut_ptr(&mut self) -> *mut u16 {
        self.bytes.as_mut_ptr()
    }

    /// Creates a `WideString` with "size" amount of zeroes.
    ///
    /// If the given size is 0, then it returns from the `Default` constructor.
    pub fn with_size(size: usize) -> Self {
        if size == 0 {
            return Self::default();
        }

        let mut vec = Vec::new();
        vec.resize(size, 0);

        Self { bytes: vec }
    }

    /// Creates a `WideString` containing `text` and fills the remaining `size` with zeroes.
    pub fn from_str_with_size(text: &str, size: usize) -> Self {
        let mut vec = get_wide_string(text);
        vec.resize(size, 0);

        Self { bytes: vec }
    }

    /// Returns a `WideString` by reading the data at a raw pointer, until a 
    /// null-byte (zero) is encoutered and then takes ownership (copy).
    pub fn from_raw_ptr(ptr: *const u16) -> Self {
        unsafe {
            let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count() + 1;
            let slice = std::slice::from_raw_parts(ptr, len);

            Self {
                bytes: slice.to_owned(),
            }
        }
    }

    /// Adds Pushes another `WideString` to itself. 
    /// It removes the null-byte from `self` before pushing on the other one.
    ///
    //// If the underlying vector of "other" is empty, the function does nothing.
    pub fn push_wide(&mut self, other: &Self) {
        let len = other.bytes.len();
        if len > 0 {
            self.bytes.reserve(len);
            self.bytes.pop();
            self.bytes.extend(other.bytes.iter());
        }
    }

    /// Adds a `&str` to itself. 
    /// 
    /// If the text is empty, the function does nothing.
    pub fn push_str(&mut self, text: &str) {
        use ::std::ffi::OsStr;
        use ::std::os::windows::ffi::OsStrExt;

        if text.len() != 0 {
            let text_as_wide = OsStr::new(text).encode_wide().chain(std::iter::once(0));

            self.bytes.reserve_exact(text.chars().count());
            self.bytes.pop();
            self.bytes.extend(text_as_wide);
        }
    }
}

impl fmt::Display for WideString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = String::from_utf16_lossy(&self.bytes[..self.bytes.len() - 1]);
        write!(f, "{}", string)
    }
}
