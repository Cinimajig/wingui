#![allow(dead_code)]

use ::std::{ffi::c_void, ptr};

// W.I.P.
pub type HWND = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type PWSTR = *const u16;

#[derive(Debug)]
pub struct Window<T> {
    h_wnd: HWND,
    h_instance: HINSTANCE,
    cls: PWSTR,
    title: PWSTR,
    child: T,
}

impl<T> Window<T> {
    pub fn new(data: T) -> Self {
        Self {
            h_wnd: ptr::null_mut(),
            h_instance: ptr::null_mut(),
            cls: ptr::null(),
            title: ptr::null(),
            child: data,
        }
    }

    pub fn data(&self) -> &T {
        &self.child
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.child
    }

    fn hwnd(&self) -> HWND {
        self.h_wnd
    }

    fn hinstance(&self) -> HINSTANCE {
        self.h_instance
    }

    fn title(&self) -> String {
        unsafe {
            let len = (0..).take_while(|&i| *self.title.offset(i) != 0).count() + 1;
            let slice = std::slice::from_raw_parts(self.title, len);

            String::from_utf16_lossy(slice)
        }
    }

    fn class(&self) -> String {
        unsafe {
            let len = (0..).take_while(|&i| *self.cls.offset(i) != 0).count() + 1;
            let slice = std::slice::from_raw_parts(self.cls, len);

            String::from_utf16_lossy(slice)
        }
    }
}

pub trait Windowing {
    fn on_create(&mut self) {}
    fn on_command(&mut self) {}
    fn on_draw(&mut self) {}
    fn on_close(&mut self) {}
    fn on_destroy(&mut self) {}
    fn on_hotkey(&mut self) {}
    fn on_notify(&mut self) {}
}
// impl<T> Windowing for Window<T> {}
