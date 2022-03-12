use ::std::{ffi::{c_void, CString}, io};
use crate::get_wide_string;

type FARPROC = Option<unsafe extern "system" fn() -> isize>;

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
    #[inline(always)]
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
    /// fn init() {
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

    /// Retrieves the address of a given function name and wraps it in a [`FnWrapper`] struct.
    /// To use the function, you must [`unwrap`] / [`match`] it before using it.
    pub fn load_func<F: Sized>(&self, name: &str) -> FnWrapper<F> {
        unsafe {
            match CString::new(name) {
                Ok(cname) => {
                    let proc = GetProcAddress(self.0, cname.as_bytes_with_nul().as_ptr());
                    let ref_proc: *const FARPROC = &proc;

                    FnWrapper(ref_proc.cast::<Option<F>>().read())
                },
                Err(_) => {
                    FnWrapper(None)
                },
            }
        }
    }

    /// A faster and unsafe version [`load_func`]. This function will panic if the 
    /// function name is invalid or doesn't exist.
    pub unsafe fn unsafe_func<F: Sized>(&self, name: &str) -> F {
        let cname = CString::new(name).unwrap_or_default();
        let proc = GetProcAddress(self.0, cname.as_bytes_with_nul().as_ptr());
        let ref_proc: *const FARPROC = &proc;
        ref_proc.cast::<Option<F>>().read().unwrap()
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            FreeLibrary(self.0);
        }
    }
}

/// Helper struct for dealing with function pointers with dynamic loaded libraries.
/// The point is to uwrap the underlying function pointer, after validating everything went well.
/// 
/// It uses a gernic to figure out the function signature.
/// # Example
/// ```
/// use std::{ptr, ffi::c_void};
/// use winutils::{
///     utils::*,
///     wstring::WideString,
/// };
/// 
/// type MsgBoxProc = extern "system" fn(*const c_void, *const u16, *const u16, i32);
/// 
/// let user32 = Library::load("User32.dll").unwrap();
/// let func: FnWrapper<MsgBoxProc> = user32.load_func("MessageBoxW");
/// 
/// if func.is_valid() {
///     let msgbox = func.unwrap();
///     let msg = WideString::from("Hello from a dynamic library loader!");
///
///     msgbox(ptr::null(), msg.ptr(), ptr::null(), 0);
/// }
/// ```
#[repr(transparent)]
#[derive(Default)]
pub struct FnWrapper<F>(pub Option<F>);

impl<F> std::fmt::Display for FnWrapper<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.0 as *const _)
    }
}

impl<F> FnWrapper<F> {
    pub fn is_valid(&self) -> bool {
        self.0.is_some()
    }

    /// Consumes the `FnWrapper` and unwraps the function-pointer underneath.
    /// 
    /// This is the same as `Option::unwrap`.
    pub fn unwrap(self) -> F {
        self.0.unwrap()
    }
}

#[link(name = "Kernel32")]
extern "system" {
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void;
    fn FreeLibrary(hLibModule: *mut c_void) -> i32;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u8) -> FARPROC;
}