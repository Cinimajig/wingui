#![allow(dead_code)]

use ::std::{ffi::c_void, io, ptr};
type HRESULT = i32;

/// Struct for helping with COM initialization. this struct automaticly calls
///  `CoUninitialize` when the variable is dropped.
pub struct ComInit;

/// Struct for helping with the Windows Runtime initialization. this struct automaticly calls
///  `RoUninitialize` when the variable is dropped.
pub struct RoInit;

impl ComInit {
    /// Initializes the COM library as single-threaded.
    /// This function fails, if it's already initialized for the current thread
    ///
    /// ## Example
    /// ```
    /// use winutils::init::ComInit;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let _com = ComInit::init_sta()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn init_sta() -> io::Result<Self> {
        #[cfg(feature = "winapi-crate")]
        unsafe {
            let result = winapi::um::combaseapi::CoInitializeEx(ptr::null_mut(), 2);
            if result != 0 {
                return Err(io::Error::from_raw_os_error(result as i32));
            }
        }

        #[cfg(not(feature = "winapi-crate"))]
        unsafe {
            let result = CoInitializeEx(ptr::null_mut(), 2);
            if result != 0 {
                return Err(io::Error::from_raw_os_error(result as i32));
            }
        }

        Ok(Self)
    }

    /// Initializes the COM library as multi-threaded.
    /// This function fails, if it's already initialized for the current thread
    ///
    /// ## Example
    /// ```
    /// use winutils::init::ComInit;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let _com = ComInit::init_mta()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn init_mta() -> io::Result<Self> {
        #[cfg(feature = "winapi-crate")]
        unsafe {
            let result = winapi::um::combaseapi::CoInitializeEx(ptr::null_mut(), 0);
            if result != 0 {
                return Err(io::Error::from_raw_os_error(result as i32));
            }
        }

        #[cfg(not(feature = "winapi-crate"))]
        unsafe {
            let result = CoInitializeEx(ptr::null_mut(), 0);
            if result != 0 {
                return Err(io::Error::from_raw_os_error(result as i32));
            }
        }

        Ok(Self)
    }
}

impl RoInit {
    /// Initializes the Windows Runtime as single-threaded.
    /// This function fails, if it's already initialized for the current thread
    ///
    /// ## Example
    /// ```
    /// use winutils::init::RoInit;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let _runtime = RoInit::init_sta()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn init_sta() -> io::Result<Self> {
        
        #[cfg(target_env = "msvc")]
        unsafe {
            let result = RoInitialize(0);
            if result != 0 {
                return Err(io::Error::from_raw_os_error(result as i32));
            }

            Ok(Self)
        }

        #[cfg(target_env = "gnu")]
        unimplemented!("Linking problems with runtimeobject.lib with gcc.")
    }

    /// Initializes the Windows Runtime as multi-threaded.
    /// This function fails, if it's already initialized for the current thread
    ///
    /// ## Example
    /// ```
    /// use winutils::init::RoInit;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let _runtime = RoInit::init_mta()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn init_mta() -> io::Result<Self> {
        #[cfg(target_env = "msvc")]
        unsafe {
            let result = RoInitialize(1);
            if result != 0 {
                return Err(io::Error::from_raw_os_error(result as i32));
            }

            Ok(Self)
        }


        #[cfg(target_env = "gnu")]
        unimplemented!("Linking problems with runtimeobject.lib with gcc.")
    }
}

impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe {
            #[cfg(feature = "winapi-crate")]
            winapi::um::combaseapi::CoUninitialize();

            #[cfg(not(feature = "winapi-crate"))]
            CoUninitialize();
        }
    }
}

#[cfg(target_env = "msvc")]
impl Drop for RoInit {
    fn drop(&mut self) {
        unsafe {
            RoUninitialize();
        }
    }
}

#[cfg(not(feature = "winapi-crate"))]
#[link(name = "Ole32")]
extern "system" {
    fn CoInitializeEx(pvreserved: *mut c_void, dwcoinit: u32) -> HRESULT;
    fn CoUninitialize();
}

#[cfg(target_env = "msvc")]
#[link(name = "runtimeobject")]
extern "system" {
    fn RoUninitialize();
    fn RoInitialize(initType: i32) -> HRESULT;
}
