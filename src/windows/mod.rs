extern crate user32;
extern crate winapi;

use self::winapi::*;
use self::user32::*;
use ::*;
use std::mem::{size_of, transmute, transmute_copy, uninitialized};
use std::cell::RefCell;
use std::thread::spawn;

pub mod inputs;

unsafe extern "system" fn keybd_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if KEYBD_BINDS.lock().unwrap().len() == 0 {
        KEYBD_HHOOK.with(|hhook| {
            if let Some(hhook) = *hhook.as_ptr() {
                UnhookWindowsHookEx(hhook);
            }
        });
    };
    if let Some(event) = match w_param as u32 {
        WM_KEYDOWN => Some(transmute(
            (*(l_param as *const KBDLLHOOKSTRUCT)).vkCode as u32,
        )),
        _ => None,
    } {
        if let Some(cb) = KEYBD_BINDS.lock().unwrap().get_mut(&event) {
            let cb = cb.clone();
            spawn(move || cb());
        };
    }
    CallNextHookEx(0 as _, code, w_param, l_param)
}

unsafe extern "system" fn mouse_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if MOUSE_BINDS.lock().unwrap().len() == 0 {
        MOUSE_HHOOK.with(|hhook| {
            if let Some(hhook) = *hhook.as_ptr() {
                UnhookWindowsHookEx(hhook);
            }
        });
    };
    if let Some(event) = match w_param as u32 {
        WM_LBUTTONDOWN => Some(MouseButton::LeftButton),
        WM_RBUTTONDOWN => Some(MouseButton::RightButton),
        WM_MBUTTONDOWN => Some(MouseButton::MiddleButton),
        _ => None,
    } {
        if let Some(cb) = MOUSE_BINDS.lock().unwrap().get_mut(&event) {
            let cb = cb.clone();
            spawn(move || cb());
        };
    }
    CallNextHookEx(0 as _, code, w_param, l_param)
}

lazy_static! {
    static ref KEYBD_BINDS: Mutex<HashMap<KeybdKey, Arc<Fn() + Send + Sync + 'static>>> = Mutex::new(HashMap::<KeybdKey, Arc<Fn() + Send + Sync + 'static>>::new());
    static ref MOUSE_BINDS: Mutex<HashMap<MouseButton, Arc<Fn() + Send + Sync + 'static>>> = Mutex::new(HashMap::<MouseButton, Arc<Fn() + Send + Sync + 'static>>::new());
}

thread_local! {
    static KEYBD_HHOOK: RefCell<Option<HHOOK>> = RefCell::new(None);
    static MOUSE_HHOOK: RefCell<Option<HHOOK>> = RefCell::new(None);
}

impl KeybdKey {
    pub fn bind<F>(self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        KEYBD_BINDS.lock().unwrap().insert(self, Arc::new(callback));
        if KEYBD_BINDS.lock().unwrap().len() != 1 {
            return;
        };
        spawn(move || unsafe {
            KEYBD_HHOOK.with(|hhook| {
                *hhook.as_ptr() = Some(SetWindowsHookExW(
                    WH_KEYBOARD_LL,
                    Some(keybd_proc),
                    0 as HINSTANCE,
                    0,
                ))
            });
            let mut msg: MSG = uninitialized();
            GetMessageW(&mut msg, 0 as HWND, 0, 0);
        });
    }

    pub fn unbind(self) {
        KEYBD_BINDS.lock().unwrap().remove(&self);
    }

    pub fn is_pressed(self) -> bool {
        (unsafe { GetAsyncKeyState(self as i32) } >> 15) != 0
    }

    pub fn press(self) {
        send_keybd_input(KEYEVENTF_SCANCODE, self);
    }

    pub fn release(self) {
        send_keybd_input(KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, self);
    }
}

impl MouseButton {
    pub fn bind<F>(self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        MOUSE_BINDS.lock().unwrap().insert(self, Arc::new(callback));
        if MOUSE_BINDS.lock().unwrap().len() != 1 {
            return;
        };
        spawn(move || unsafe {
            MOUSE_HHOOK.with(|hhook| {
                *hhook.as_ptr() = Some(SetWindowsHookExW(
                    WH_MOUSE_LL,
                    Some(mouse_proc),
                    0 as HINSTANCE,
                    0,
                ))
            });
            let mut msg: MSG = uninitialized();
            GetMessageW(&mut msg, 0 as HWND, 0, 0);
        });
    }

    pub fn unbind(self) {
        MOUSE_BINDS.lock().unwrap().remove(&self);
    }

    pub fn is_pressed(self) -> bool {
        (unsafe { GetAsyncKeyState(self as i32) } >> 15) != 0
    }

    pub fn press(self) {
        match self {
            MouseButton::LeftButton => send_mouse_input(MOUSEEVENTF_LEFTDOWN, 0, 0, 0),
            MouseButton::RightButton => send_mouse_input(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0),
            MouseButton::MiddleButton => send_mouse_input(MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0),
            _ => {}
        }
    }

    pub fn release(self) {
        match self {
            MouseButton::LeftButton => send_mouse_input(MOUSEEVENTF_LEFTUP, 0, 0, 0),
            MouseButton::RightButton => send_mouse_input(MOUSEEVENTF_RIGHTUP, 0, 0, 0),
            MouseButton::MiddleButton => send_mouse_input(MOUSEEVENTF_MIDDLEUP, 0, 0, 0),
            _ => {}
        }
    }
}

fn send_mouse_input(flags: u32, data: u32, dx: i32, dy: i32) {
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe {
            transmute_copy(&MOUSEINPUT {
                dx: dx,
                dy: dy,
                mouseData: data,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            })
        },
    };
    unsafe { SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int) };
}

fn send_keybd_input(flags: u32, key_code: KeybdKey) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe {
            transmute_copy(&KEYBDINPUT {
                wVk: 0,
                wScan: MapVirtualKeyW(key_code as u32, 0) as u16,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            })
        },
    };
    unsafe { SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int) };
}

pub fn mouse_move(dx: i32, dy: i32) {
    send_mouse_input(MOUSEEVENTF_MOVE, 0, dx, dy);
}

pub fn mouse_move_to(x: i32, y: i32) {
    unsafe {
        send_mouse_input(
            MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
            0,
            x * 65_335 / GetSystemMetrics(78),
            y * 65_335 / GetSystemMetrics(79),
        )
    };
}

pub fn wheel_scroll_hor(dwheel: i32) {
    send_mouse_input(MOUSEEVENTF_HWHEEL, unsafe { transmute(dwheel * 120) }, 0, 0);
}

pub fn wheel_scroll_ver(dwheel: i32) {
    send_mouse_input(MOUSEEVENTF_WHEEL, unsafe { transmute(dwheel * 120) }, 0, 0);
}

pub fn num_lock_is_toggled() -> bool {
    unsafe { GetKeyState(0x90 as i32) & 15 != 0 }
}

pub fn caps_lock_is_toggled() -> bool {
    unsafe { GetKeyState(0x14 as i32) & 15 != 0 }
}
