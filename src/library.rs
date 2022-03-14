use ::std::{ffi::{c_void, CString}, io};
use crate::get_wide_string;

type FARPROC = Option<unsafe extern "system" fn() -> isize>;

/// Struct for helping with loading external Libraries (dll).
/// The Library is automaticly unloaded when dropped, Unlees a static lib is loaded 
/// (can check with the [`lib_type`](`Self::lib_type`)).
/// 
/// If you need to manually unload the dll, you can 
/// call the `free_lib` function. This does nothing if it's a static library.
#[derive(Debug)]
pub struct Library {
    handle: *mut c_void,
    lib_type: LibType,
}

/// Library types used by [`Library`]. Static libraries will not be unloaded on [`Drop`].
#[derive(Debug, Clone, Copy)]
pub enum LibType {
    Static,
    Dynamic,
}

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

            Ok(Self {
                handle,
                lib_type: LibType::Dynamic
            })
        }
    }

    /// Returns a [`Library`] from a raw handle. You should wheater not, it's a static 
    /// library or dynamic.
    pub fn from_handle(handle: *mut c_void, dynamic: bool) -> io::Result<Self> {
        if handle.is_null() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a valid handle."));
        }

        Ok(Self {
            handle,
            lib_type: if dynamic { LibType::Dynamic } else { LibType::Static }
        })
    }

    /// Returns a [`Library`], that is staticly linked to the program.
    /// Nothing happens, when this gets dropped.
    pub fn get_static_lib(path: &str) -> io::Result<Self> {
        unsafe {
            let w_path = get_wide_string(path);
            let handle = GetModuleHandleW(w_path.as_ptr());

            if handle.is_null() || path.len() == 0 {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a lib name."));
            }

            Ok(Self {
                handle,
                lib_type: LibType::Static
            })
        }
    }

    /// Returns the [`LibType`] from `&self`.
    pub fn lib_type(&self) -> LibType {
        self.lib_type
    }

    /// Returns a Library struct, without loading any file.
    /// This can be used in a static way.
    /// ```
    /// static mut LIB: Library = Library::empty();
    /// ```
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            handle: 0 as *mut c_void,
            lib_type: LibType::Static,
        }
    }

    /// Returns the raw handle of the library.
    #[inline(always)]
    pub fn handle(&self) -> *mut c_void {
        self.handle
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
            FreeLibrary(self.handle) != 0
        }
    }

    /// Retrieves the address of a given function name and wraps it in a [`FnWrapper`] struct.
    /// To use the function, you must [`unwrap`] / [`match`] it before using it.
    pub fn load_func<F: Sized>(&self, name: &str) -> FnWrapper<F> {
        unsafe {
            match CString::new(name) {
                Ok(cname) => {
                    let proc = GetProcAddress(self.handle, cname.as_bytes_with_nul().as_ptr());
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
        let proc = GetProcAddress(self.handle, cname.as_bytes_with_nul().as_ptr());
        let ref_proc: *const FARPROC = &proc;
        ref_proc.cast::<Option<F>>().read().unwrap()
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            match self.lib_type {
                LibType::Dynamic => {
                    FreeLibrary(self.handle);
                },
                _ => (),
            }
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
    fn GetModuleHandleW(lpModuleName: *const u16) -> *mut c_void;
}