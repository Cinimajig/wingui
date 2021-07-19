use ::std::{ffi::c_void, io, ptr};

/// Struct for helping with COM initialization. this struct automaticly calls
///  `CoUninitialize` when the variable is dropped
pub struct ComInit;

/// Struct for helping with the Windows Runtime initialization. this struct automaticly calls
///  `RoUninitialize` when the variable is dropped
pub struct RoInit;

impl ComInit {
    /// Initializes the COM library as single-threaded
    pub fn init_sta() -> io::Result<Self> {
        unsafe {
            let result = CoInitializeEx(ptr::null_mut(), 2);
            if result != 0 {
                return Err(io::Error::from_raw_os_error(result as i32));
            }
        }

        Ok(Self)
    }

    /// Initializes the COM library as multi-threaded
    pub fn init_mta() -> io::Result<Self> {
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
    /// Initializes the Windows Runtime as single-threaded
    pub fn init_sta() -> io::Result<Self> {
        unsafe {
            let result = RoInitialize(0);
            if result != 0 {
                return Err(io::Error::from_raw_os_error(result as i32));
            }
        }

        Ok(Self)
    }

    /// Initializes the Windows Runtime as multi-threaded
    pub fn init_mta() -> io::Result<Self> {
        unsafe {
            let result = RoInitialize(1);
            if result != 0 {
                return Err(io::Error::from_raw_os_error(result as i32));
            }
        }

        Ok(Self)
    }
}

impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

impl Drop for RoInit {
    fn drop(&mut self) {
        unsafe {
            RoUninitialize();
        }
    }
}

extern "system" {
    fn RoInitialize(initType: i32) -> u32;
    fn RoUninitialize();
}

#[link(name = "Ole32")]
extern "system" {
    fn CoInitializeEx(pvreserved: *mut c_void, dwcoinit: u32) -> u32;
    fn CoUninitialize();
}