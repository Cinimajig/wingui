use ::std::{ffi::c_void, io};
use crate::get_wide_string;

/// Struct for helping with loading external Libraries (dll).
/// The Library is automaticly unloaded when dropped.
/// 
/// If you need to manually unload the dll, you can 
/// call the `free_lib` function.
pub struct Library(*mut c_void);

impl Library {
    /// Loads a dll file, from the system defined in `path`.
    /// It returns an [`std::io::Result`], based on if it worked.
    pub fn load(path: &str) -> io::Result<Self> {
        unsafe {
            let w_path = get_wide_string(path);
            let handle = LoadLibraryW(w_path.as_ptr());

            if handle.is_null() {
                return Err(io::Error::last_os_error());
            }

            Ok(Self(handle))
        }
    }

    /// Returns a Library struct, without loading any file.
    /// This can be used in a static way.
    /// ```
    /// static mut LIB: Library = Library::empty();
    /// ```
    pub const fn empty() -> Self {
        Self(0 as *mut c_void)
    }

    /// Returns the raw handle of the library.
    #[inline(always)]
    pub fn handle(&self) -> *mut c_void {
        self.0
    }

    /// Unloads the library without dropping the struct.
    /// Only use this, if your variable does not go out 
    /// of scope.
    /// # Example
    /// ```
    /// static mut LIB: Library = Library::empty();
    /// 
    /// unsafe fn init() {
    ///     LIB = Library::load("User32.dll").unwrap();
    ///     // Do stuff here...
    /// 
    ///     LIB.free_lib();
    /// }
    /// ```
    pub fn free_lib(&self) -> bool {
        unsafe {
            FreeLibrary(self.0) != 0
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            FreeLibrary(self.0);
        }
    }
}

#[link(name = "Kernel32")]
extern "system" {
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void;
    fn FreeLibrary(hLibModule: *mut c_void) -> i32;
}