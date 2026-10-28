#![allow(non_snake_case)]

use std::ffi::c_void;
use std::mem::zeroed;
use std::ptr::null_mut;

use crate::state::State;

type LPCWSTR = *const u16;
type HWND = *mut std::ffi::c_void;
type HDC = *mut c_void;
type HINSTANCE = *mut c_void;
type HMENU = *mut c_void;
type HICON = *mut c_void;
type HCURSOR = *mut c_void;
type HBRUSH = *mut c_void;
type BOOL = i32;
type UINT = u32;
type DWORD = u32;
type WPARAM = usize;
type LPARAM = isize;
type LRESULT = isize;

#[repr(C)]
struct WNDCLASSW {
    style: UINT,
    lpfnWndProc: Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>,
    cbClsExtra: i32,
    cbWndExtra: i32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
}

#[repr(C)]
struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt_x: i32,
    pt_y: i32,
}

#[repr(C)]
struct BITMAPINFOHEADER {
    biSize: DWORD,
    biWidth: i32,
    biHeight: i32,
    biPlanes: u16,
    biBitCount: u16,
    biCompression: DWORD,
    biSizeImage: DWORD,
    biXPelsPerMeter: i32,
    biYPelsPerMeter: i32,
    biClrUsed: DWORD,
    biClrImportant: DWORD,
}

#[repr(C)]
struct CREATESTRUCTW {
    pub lpCreateParams: *mut c_void,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: LPCWSTR,
    pub lpszClass: LPCWSTR,
    pub dwExStyle: DWORD,
}

#[repr(C)]
struct RGBQUAD {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8,
}

#[repr(C)]
struct BITMAPINFO {
    bmiHeader: BITMAPINFOHEADER,
    bmiColors: [RGBQUAD; 1],
}

#[link(name = "user32")]
unsafe extern "system" {
    fn RegisterClassW(wc: *const WNDCLASSW) -> u16;
    fn CreateWindowExW(
        ex: DWORD,
        class: LPCWSTR,
        title: LPCWSTR,
        style: DWORD,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        parent: HWND,
        menu: HMENU,
        instance: HINSTANCE,
        param: *mut c_void,
    ) -> HWND;
    fn DefWindowProcW(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT;
    fn ShowWindow(hwnd: HWND, cmd: i32) -> BOOL;
    fn PeekMessageW(msg: *mut MSG, hwnd: HWND, min: UINT, max: UINT, remove: UINT) -> BOOL;
    fn SetWindowLongPtrW(hwnd: HWND, index: i32, new_long: isize) -> isize;
    fn GetWindowLongPtrW(hwnd: HWND, index: i32) -> isize;
    fn TranslateMessage(msg: *const MSG) -> BOOL;
    fn DispatchMessageW(msg: *const MSG) -> LRESULT;
    fn PostQuitMessage(code: i32);
    fn GetModuleHandleW(name: LPCWSTR) -> HINSTANCE;
    fn GetDC(hwnd: HWND) -> HDC;
    fn ReleaseDC(hwnd: HWND, dc: HDC) -> i32;
    fn DestroyWindow(hwnd: HWND) -> BOOL;
    fn GetKeyState(nVirtKey: i32) -> i16;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    fn StretchDIBits(
        dc: HDC,
        x: i32,
        y: i32,
        dw: i32,
        dh: i32,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
        bits: *const c_void,
        info: *const BITMAPINFO,
        usage: UINT,
        rop: DWORD,
    ) -> i32;
}

const WM_CREATE: UINT = 0x0001;
const WM_DESTROY: UINT = 0x0002;
const WM_CLOSE: UINT = 0x0010;
const WM_KEYDOWN: UINT = 0x0100;
const WM_LBUTTONDOWN: UINT = 0x0201;
const WM_RBUTTONDOWN: UINT = 0x0204;
const WM_MOUSEWHEEL: UINT = 0x20A;
const WS_OVERLAPPEDWINDOW: DWORD = 0x00CF0000;
const SW_SHOW: i32 = 5;
const PM_REMOVE: UINT = 1;
const BI_RGB: DWORD = 0;
const DIB_RGB_COLORS: UINT = 0;
const SRCCOPY: DWORD = 0x00CC0020;
const GWLP_USERDATA: i32 = -21;

// Key codes

const VK_SHIFT: i32 = 0x10;

const KEY_0: WPARAM = 0x30;
const KEY_1: WPARAM = 0x31;
const KEY_2: WPARAM = 0x32;
const KEY_3: WPARAM = 0x33;
const KEY_4: WPARAM = 0x34;
const KEY_5: WPARAM = 0x35;
const KEY_6: WPARAM = 0x36;
const KEY_7: WPARAM = 0x37;
const KEY_8: WPARAM = 0x38;
const KEY_9: WPARAM = 0x39;

const KEY_R: WPARAM = 0x52;
const KEY_G: WPARAM = 0x47;
const KEY_B: WPARAM = 0x42;

unsafe extern "system" fn wndproc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => unsafe {
            let create_struct = lp as *const CREATESTRUCTW;
            if !create_struct.is_null() {
                let state_ptr = (*create_struct).lpCreateParams;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, state_ptr as isize);
            }
            0
        },
        WM_LBUTTONDOWN => unsafe {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
            if !ptr.is_null() {
                let state = &mut *ptr;
                state.reverse_spin_direction();
            }
            0
        },
        WM_RBUTTONDOWN => unsafe {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
            if !ptr.is_null() {
                let state = &mut *ptr;
                state.switch_model();
            }
            0
        },
        WM_MOUSEWHEEL => unsafe {
            let delta = ((wp >> 16) & 0xFFFF) as u16 as i16;
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
            if !ptr.is_null() {
                let state = &mut *ptr;
                if delta > 0 {
                    state.move_towards_screen();
                } else if delta < 0 {
                    state.move_away_from_screen();
                }
            }
            0
        },
        WM_KEYDOWN => unsafe {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
            if !ptr.is_null() {
                let state = &mut *ptr;
                let shift_held = GetKeyState(VK_SHIFT) < 0;
                match wp {
                    KEY_R => {
                        if shift_held {
                            state.decrease_red();
                        } else {
                            state.increase_red();
                        }
                    }
                    KEY_G => {
                        if shift_held {
                            state.decrease_green();
                        } else {
                            state.increase_green();
                        }
                    }
                    KEY_B => {
                        if shift_held {
                            state.decrease_blue();
                        } else {
                            state.increase_blue();
                        }
                    }
                    KEY_0 => {
                        state.toggle_color_rotation();
                    }
                    KEY_1 => {
                        state.set_line_stroke_radius(1);
                    }
                    KEY_2 => {
                        state.set_line_stroke_radius(2);
                    }
                    KEY_3 => {
                        state.set_line_stroke_radius(3);
                    }
                    KEY_4 => {
                        state.set_line_stroke_radius(4);
                    }
                    KEY_5 => {
                        state.set_line_stroke_radius(5);
                    }
                    KEY_6 => {
                        state.set_line_stroke_radius(6);
                    }
                    KEY_7 => {
                        state.set_line_stroke_radius(7);
                    }
                    KEY_8 => {
                        state.set_line_stroke_radius(8);
                    }
                    KEY_9 => {
                        state.set_line_stroke_radius(9);
                    }
                    _ => {}
                }
            }
            0
        },
        WM_CLOSE => unsafe {
            DestroyWindow(hwnd);
            0
        },
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wp, lp) },
    }
}

pub struct Window {
    hwnd: HWND,
    width: usize,
    height: usize,
    info: BITMAPINFO,
    running: bool,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize, state: *mut State) -> Self {
        unsafe {
            let instance = GetModuleHandleW(null_mut());

            let class: Vec<u16> = "RustPixelWindow\0".encode_utf16().collect();
            let title: Vec<u16> = format!("{title}\0").encode_utf16().collect();

            let wc = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: instance,
                hIcon: null_mut(),
                hCursor: null_mut(),
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
                lpszClassName: class.as_ptr(),
            };

            assert_ne!(RegisterClassW(&wc), 0, "RegisterClassW failed");

            let hwnd = CreateWindowExW(
                0,
                class.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                100,
                100,
                width as i32 + 16,
                height as i32 + 39,
                null_mut(),
                null_mut(),
                instance,
                state as *mut c_void,
            );

            assert!(!hwnd.is_null(), "CreateWindowExW failed");

            ShowWindow(hwnd, SW_SHOW);

            Self {
                hwnd,
                width,
                height,
                info: BITMAPINFO {
                    bmiHeader: BITMAPINFOHEADER {
                        biSize: size_of::<BITMAPINFOHEADER>() as u32,
                        biWidth: width as i32,
                        biHeight: -(height as i32),
                        biPlanes: 1,
                        biBitCount: 32,
                        biCompression: BI_RGB,
                        biSizeImage: 0,
                        biXPelsPerMeter: 0,
                        biYPelsPerMeter: 0,
                        biClrUsed: 0,
                        biClrImportant: 0,
                    },
                    bmiColors: [RGBQUAD {
                        blue: 0,
                        green: 0,
                        red: 0,
                        reserved: 0,
                    }],
                },
                running: true,
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.running
    }

    pub fn update_with_buffer(&mut self, pixels: &[u32]) {
        assert_eq!(pixels.len(), self.width * self.height);

        unsafe {
            let mut msg: MSG = zeroed();

            while PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == 0x0012 {
                    self.running = false;
                    return;
                }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let dc = GetDC(self.hwnd);

            StretchDIBits(
                dc,
                0,
                0,
                self.width as i32,
                self.height as i32,
                0,
                0,
                self.width as i32,
                self.height as i32,
                pixels.as_ptr() as *const c_void,
                &self.info,
                DIB_RGB_COLORS,
                SRCCOPY,
            );

            ReleaseDC(self.hwnd, dc);
        }
    }
}
