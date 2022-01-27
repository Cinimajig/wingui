#![allow(dead_code, non_snake_case)]

use ::std::{ffi::c_void, ptr, mem};

// W.I.P.
pub type HWND = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type HICON = *mut c_void;
pub type HCURSOR = *mut c_void;
pub type HBRUSH = *mut c_void;
pub type HMENU = *mut c_void;
pub type PWSTR = *const u16;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;

type _WNDPROC = unsafe extern "system" fn(h_wnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
type WNDPROC = Option<_WNDPROC>;

const IDI_APPLICATION: PWSTR = 0x7f00 as PWSTR;
const IDC_ARROW: PWSTR = 0x7f00 as PWSTR;

const CS_VREDRAW: u32 = 1u32;
const CS_HREDRAW: u32 = 2u32;

const WM_NULL: u32 = 0;
const WM_CREATE: u32 = 1;
const WM_DESTROY: u32 = 2;
const WM_PAINT: u32 = 15;
const WM_CLOSE: u32 = 16;
const WM_QUIT: u32 = 18;
const WM_NOTIFY: u32 = 78;
const WM_COMMAND: u32 = 273;
const WM_WTSSESSION_CHANGE: u32 = 689;
const WM_HOTKEY: u32 = 786;

const WS_OVERLAPPEDWINDOW: u32 = 13565952;

const SW_SHOW: i32 = 5;
const SW_HIDE: i32 = 0;


#[repr(C)]
struct WNDCLASSEXW {
    cbSize: u32,
    style: u32,
    lpfnWndProc: WNDPROC,
    cbCksExtra: i32,
    cbWndExtra: i32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: PWSTR,
    lpszClassName: PWSTR,
    hInconSm: HICON,
}

#[repr(C)]
#[derive(Default)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

impl Default for MSG {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Default for WNDCLASSEXW {
    fn default() -> Self {
        unsafe {
            mem::zeroed()
        }
    }
}

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
            h_instance: unsafe { GetModuleHandleW(ptr::null_mut()) },
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

    // pub fn hwnd(&self) -> HWND {
    //     self.h_wnd
    // }

    pub fn hinstance(&self) -> HINSTANCE {
        unsafe { GetModuleHandleW(ptr::null_mut()) }
    }

    pub fn title(&self) -> String {
        unsafe {
            let len = (0..).take_while(|&i| *self.title.offset(i) != 0).count() + 1;
            let slice = ::std::slice::from_raw_parts(self.title, len);

            String::from_utf16_lossy(slice)
        }
    }

    pub fn class(&self) -> String {
        unsafe {
            let len = (0..).take_while(|&i| *self.cls.offset(i) != 0).count() + 1;
            let slice = ::std::slice::from_raw_parts(self.cls, len);

            String::from_utf16_lossy(slice)
        }
    }

}

pub fn show(h_wnd: HWND) {
    unsafe {
        ShowWindow(h_wnd, SW_SHOW);
    }
}

pub fn hide(h_wnd: HWND) {
    unsafe {
        ShowWindow(h_wnd, SW_HIDE);
    }
}

#[allow(unused_variables)]
pub trait Windowing {
    fn on_create(&mut self, w_param: WPARAM, l_param: LPARAM) {}
    fn on_command(&mut self, w_param: WPARAM, l_param: LPARAM) {}
    fn on_draw(&mut self, w_param: WPARAM, l_param: LPARAM) {}
    fn on_close(&mut self, w_param: WPARAM, l_param: LPARAM) {}
    fn on_destroy(&mut self, w_param: WPARAM, l_param: LPARAM) {}
    fn on_hotkey(&mut self, w_param: WPARAM, l_param: LPARAM) {}
    fn on_notify(&mut self, w_param: WPARAM, l_param: LPARAM) {}
    fn on_session_change(&mut self, w_param: WPARAM, l_param: LPARAM) {}

    fn run(&mut self) -> WPARAM {
        unsafe {
            let mut msg = MSG::default();

            while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
                if msg.message == WM_QUIT {
                    break;
                }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            msg.wParam
        }
    }

    fn hinstance(&self) -> HINSTANCE {
        unsafe { GetModuleHandleW(ptr::null_mut()) }
    }

    fn register(&mut self, class_name: &str) {
        unsafe {
            let cls = crate::get_wide_string(class_name);

            let wc = WNDCLASSEXW {
                cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_VREDRAW | CS_HREDRAW,
                hIcon: LoadIconW(ptr::null_mut(), IDI_APPLICATION),
                hInconSm: LoadIconW(ptr::null_mut(), IDI_APPLICATION),
                hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
                hInstance: self.hinstance(),
                lpszClassName: cls.as_ptr(),
                lpfnWndProc: Some(DefWindowProcW),
                ..Default::default()
            };

            // if RegisterClassExW(&wc) != 0 {
            //     self.cls = wc.lpszClassName;
            // }
        }
    }

    fn create_window(&mut self, class: &str, title: &str, width: i32, height: i32) -> HWND {
        let wtitle = crate::get_wide_string(title);
        let cls = crate::get_wide_string(class);

        unsafe {
            let (x, y) = {
                let mut rect = RECT::default();
                let h_dsk = GetDesktopWindow();
                GetClientRect(h_dsk, &mut rect);
        
                ((rect.right - width) / 2, (rect.bottom - height) / 2)
            };

            CreateWindowExW(
                0,
                cls.as_ptr(),
                wtitle.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                x,
                y,
                width,
                height,
                ptr::null_mut(),
                ptr::null_mut(),
                self.hinstance(),
                ptr::null()
            )
        }
    }

    fn wnd_proc(&mut self, h_wnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        unsafe {
            let mut result = w_param as LRESULT;

            match msg {
                WM_CREATE => self.on_create(w_param, l_param),
                WM_COMMAND => self.on_command(w_param, l_param),
                WM_PAINT => self.on_draw(w_param, l_param),
                WM_CLOSE => self.on_close(w_param, l_param),
                WM_DESTROY => self.on_destroy(w_param, l_param),
                WM_HOTKEY => self.on_hotkey(w_param, l_param),
                WM_NOTIFY => self.on_notify(w_param, l_param),
                WM_WTSSESSION_CHANGE => self.on_session_change(w_param, l_param),
                _ => result = DefWindowProcW(h_wnd, msg, w_param, l_param),
            };

            result
        }
    }
}

// impl<T> Windowing for Window<T> {
//     fn run(&mut self) {
//         unsafe {
//             let func = Self::wnd_proc;
//             SetWindowLongPtrW(self.h_wnd, -4i32 /* GWL_WNDPROC */, func as isize);

//             let mut msg = MSG::default();
//             while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
//                 if msg.message == WM_QUIT {
//                     break;
//                 }

//                 TranslateMessage(&msg);
//                 DispatchMessageW(&msg);
//             }
//         }
//     }
// }

#[link(name = "User32")]
extern "system" {
    fn RegisterClassExW(lpclassex: *const WNDCLASSEXW) -> u16;
    fn DefWindowProcW(h_wnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
    fn LoadIconW(hinstance: HINSTANCE, lpiconname: PWSTR) -> HICON;
    fn LoadCursorW(hinstance: HINSTANCE, lpcursorname: PWSTR) -> HCURSOR;
    fn GetMessageW(lpmsg: *mut MSG, hwnd: HWND, wmsgfiltermin: u32, wmsgfiltermax: u32) -> i32;
    fn TranslateMessage(lpmsg: *const MSG) -> i32;
    fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    fn GetDesktopWindow() -> HWND;
    fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> i32;
    fn SetWindowLongPtrW(
        hwnd: HWND,
        nindex: i32,
        dwnewlong: isize
    ) -> isize;    
    fn GetClientRect(
        hwnd: HWND, 
        lprect: *mut RECT
    ) -> i32;    
    fn CreateWindowExW(
        dwexstyle: u32, 
        lpclassname: PWSTR, 
        lpwindowname: PWSTR, 
        dwstyle: u32, 
        x: i32, 
        y: i32, 
        nwidth: i32, 
        nheight: i32, 
        hwndparent: HWND, 
        hmenu: HMENU, 
        hinstance: HINSTANCE, 
        lpparam: *const c_void
    ) -> HWND;
    
}

#[link(name = "Kernel32")]
extern "system" {
    fn GetModuleHandleW(lpModuleName: PWSTR) -> HINSTANCE;
}