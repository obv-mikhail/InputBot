use crate::{common::*, public::*};
use std::{
    mem::{size_of, transmute_copy, MaybeUninit},
    ptr::null_mut,
    sync::atomic::AtomicPtr,
};
use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::*},
    um::winuser::*,
};
use once_cell::sync::Lazy;

mod inputs;

static KEYBD_HHOOK: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);
static MOUSE_HHOOK: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

impl KeybdKey {
    pub fn is_pressed(self) -> bool {
        (unsafe { GetAsyncKeyState(u64::from(self) as i32) } >> 15) != 0
    }

    pub fn is_toggled(self) -> bool {
        unsafe { GetKeyState(u64::from(self) as i32) & 15 != 0 }
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

impl MouseCursor {
    pub fn pos() -> (i32, i32) {
        unsafe {
            let mut point = MaybeUninit::uninit();
            GetCursorPos(point.as_mut_ptr());
            let point = point.assume_init();
            (point.x, point.y)
        }
    }

    pub fn move_rel(dx: i32, dy: i32) {
        let (x, y) = Self::pos();
        Self::move_abs(x + dx, y + dy);
    }

    pub fn move_abs(x: i32, y: i32) {
        unsafe {
            SetCursorPos(x, y);
        }
    }
}

impl MouseWheel {
    pub fn scroll_ver(dwheel: i32) {
        send_mouse_input(MOUSEEVENTF_WHEEL, (dwheel * 120) as u32, 0, 0);
    }

    pub fn scroll_hor(dwheel: i32) {
        send_mouse_input(MOUSEEVENTF_HWHEEL, (dwheel * 120) as u32, 0, 0);
    }
}

pub fn handle_input_events() {
    if !MOUSE_BINDS.lock().unwrap().is_empty() {
        set_hook(WH_MOUSE_LL, &*MOUSE_HHOOK, mouse_proc);
    };
    if !KEYBD_BINDS.lock().unwrap().is_empty() {
        set_hook(WH_KEYBOARD_LL, &*KEYBD_HHOOK, keybd_proc);
    };
    let mut msg: MSG = unsafe { MaybeUninit::zeroed().assume_init() };
    unsafe { GetMessageW(&mut msg, 0 as HWND, 0, 0) };
}

unsafe extern "system" fn keybd_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if KEYBD_BINDS.lock().unwrap().is_empty() {
        unset_hook(&*KEYBD_HHOOK);
    } else if w_param as u32 == WM_KEYDOWN {
        if let Some(bind) = KEYBD_BINDS
            .lock()
            .unwrap()
            .get_mut(&KeybdKey::from(u64::from(
                (*(l_param as *const KBDLLHOOKSTRUCT)).vkCode,
            )))
        {
            match bind {
                Bind::NormalBind(cb) => {
                    let cb = Arc::clone(cb);
                    spawn(move || cb());
                }
                Bind::BlockableBind(cb) => {
                    if let BlockInput::Block = cb() {
                        return 1;
                    }
                }
            }
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
        WM_XBUTTONDOWN => {
            let llhs = &*(l_param as *const MSLLHOOKSTRUCT);

            match HIWORD(llhs.mouseData) {
                XBUTTON1 => Some(MouseButton::X1Button),
                XBUTTON2 => Some(MouseButton::X2Button),
                _ => None,
            }
        },
        _ => None,
    } {
        if let Some(bind) = MOUSE_BINDS.lock().unwrap().get_mut(&event) {
            match bind {
                Bind::NormalBind(cb) => {
                    let cb = Arc::clone(cb);
                    spawn(move || cb());
                }
                Bind::BlockableBind(cb) => {
                    if let BlockInput::Block = cb() {
                        return 1;
                    }
                }
            }
        };
    }
    CallNextHookEx(null_mut(), code, w_param, l_param)
}

fn set_hook(
    hook_id: i32,
    hook_ptr: &AtomicPtr<HHOOK__>,
    hook_proc: unsafe extern "system" fn(c_int, WPARAM, LPARAM) -> LRESULT,
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

fn send_mouse_input(flags: u32, data: u32, dx: i32, dy: i32) {
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe {
            transmute_copy(&MOUSEINPUT {
                dx,
                dy,
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
                wScan: MapVirtualKeyW(u64::from(key_code) as u32, 0) as u16,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            })
        },
    };
    unsafe { SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int) };
}
