extern crate user32;
extern crate winapi;

use self::winapi::*;
use self::user32::*;
use std::mem::{size_of, transmute, transmute_copy, uninitialized};
use std::ptr::null_mut;
use ::*;

mod inputs;
pub use self::inputs::*;

unsafe extern "system" fn keybd_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if KEYBD_BINDS.lock().unwrap().is_empty() {
        unset_hook(&*KEYBD_HHOOK);
    } else if let Some(event) = match w_param as u32 {
        WM_KEYDOWN => Some(transmute(
            (*(l_param as *const KBDLLHOOKSTRUCT)).vkCode as u32,
        )),
        _ => None,
    } {
        if let Some(cb) = KEYBD_BINDS.lock().unwrap().get_mut(&event) {
            let cb = Arc::clone(cb);
            spawn(move || cb());
        }
    }
    CallNextHookEx(null_mut(), code, w_param, l_param)
}

unsafe extern "system" fn mouse_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if MOUSE_BINDS.lock().unwrap().is_empty() {
        unset_hook(&*MOUSE_HHOOK);
    } else if let Some(event) = match w_param as u32 {
        WM_LBUTTONDOWN => Some(MouseButton::LeftButton),
        WM_RBUTTONDOWN => Some(MouseButton::RightButton),
        WM_MBUTTONDOWN => Some(MouseButton::MiddleButton),
        _ => None,
    } {
        if let Some(cb) = MOUSE_BINDS.lock().unwrap().get_mut(&event) {
            let cb = Arc::clone(cb);
            spawn(move || cb());
        };
    }
    CallNextHookEx(null_mut(), code, w_param, l_param)
}

lazy_static! {
    static ref KEYBD_HHOOK: AtomicPtr<HHOOK__> = AtomicPtr::default();
    static ref MOUSE_HHOOK: AtomicPtr<HHOOK__> = AtomicPtr::default();
}

fn set_hook(
    hook_id: i32,
    hook_ptr: &AtomicPtr<HHOOK__>,
    hook_proc: unsafe extern "system" fn(i32, u64, i64) -> i64,
) {
    hook_ptr.store(
        unsafe { SetWindowsHookExW(hook_id, Some(hook_proc), 0 as HINSTANCE, 0) },
        Ordering::Relaxed,
    );
}

fn unset_hook(hook_ptr: &AtomicPtr<HHOOK__>) {
    if !hook_ptr.load(Ordering::Relaxed).is_null() {
        unsafe { UnhookWindowsHookEx(hook_ptr.load(Ordering::Relaxed)) };
        hook_ptr.store(null_mut(), Ordering::Relaxed);
    }
}

impl KeybdKey {
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
    pub fn is_pressed(self) -> bool {
        (unsafe { GetAsyncKeyState(u32::from(self) as i32) } >> 15) != 0
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

pub fn handle_input_events() {
    if !MOUSE_BINDS.lock().unwrap().is_empty() {
        set_hook(WH_MOUSE_LL, &*MOUSE_HHOOK, mouse_proc);
    };
    if !KEYBD_BINDS.lock().unwrap().is_empty() {
        set_hook(WH_KEYBOARD_LL, &*KEYBD_HHOOK, keybd_proc);
    };
    let mut msg: MSG = unsafe { uninitialized() };
    unsafe { GetMessageW(&mut msg, 0 as HWND, 0, 0) };
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
