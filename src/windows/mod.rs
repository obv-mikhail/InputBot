use crate::{common::*, public::*};
use once_cell::sync::Lazy;
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

mod inputs;

static KEYBD_HHOOK: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);
static MOUSE_HHOOK: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

impl KeybdKey {
    /// Returns true if a given `KeybdKey` is currently pressed (in the down position).
    pub fn is_pressed(self) -> bool {
        (unsafe { GetAsyncKeyState(u64::from(self) as i32) } >> 15) != 0
    }

    /// Presses a given `KeybdKey`. Note: this means the key will remain in the down
    /// position. You must manually call release to create a full 'press'.
    pub fn press(self) {
        send_keybd_input(KEYEVENTF_SCANCODE, self);
    }

    /// Releases a given `KeybdKey`. This means the key would be in the up position.
    pub fn release(self) {
        send_keybd_input(KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, self);
    }

    /// Returns true if a keyboard key which supports toggling (ScrollLock, NumLock,
    /// CapsLock) is on.
    pub fn is_toggled(self) -> bool {
        unsafe { GetKeyState(u64::from(self) as i32) & 15 != 0 }
    }
}

impl MouseButton {
    /// Returns true if a given `MouseButton` is currently pressed (in the down position).
    pub fn is_pressed(self) -> bool {
        (unsafe { GetAsyncKeyState(u32::from(self) as i32) } >> 15) != 0
    }

    /// Presses a given `MouseButton`. Note: this means the button will remain in the down
    /// position. You must manually call release to create a full 'click'.
    pub fn press(self) {
        match self {
            MouseButton::LeftButton => send_mouse_input(MOUSEEVENTF_LEFTDOWN, 0, 0, 0),
            MouseButton::RightButton => send_mouse_input(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0),
            MouseButton::MiddleButton => send_mouse_input(MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0),
            _ => {}
        }
    }

    /// Releases a given `MouseButton`. This means the button would be in the up position.
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
        
            let mut point = MaybeUninit::uninit();
            unsafe { GetCursorPos(point.as_mut_ptr()) };
            let point = unsafe { point.assume_init() };
            (point.x, point.y)
        
    }

    /// Moves the mouse relative to its current position by a given amount of pixels.
    pub fn move_rel(dx: i32, dy: i32) {
        let (x, y) = Self::pos();
        Self::move_abs(x + dx, y + dy);
    }

    /// Moves the mouse to a given position based on absolute coordinates. The top left
    /// corner of the screen is (0, 0).
    pub fn move_abs(x: i32, y: i32) {
        unsafe {
            SetCursorPos(x, y);
        }
    }
}

impl MouseWheel {
    /// Scrolls the mouse wheel vertically by a given amount.
    pub fn scroll_ver(dwheel: i32) {
        send_mouse_input(MOUSEEVENTF_WHEEL, (dwheel * 120) as u32, 0, 0);
    }

    /// Scrolls the mouse wheel horizontally by a given amount.
    pub fn scroll_hor(dwheel: i32) {
        send_mouse_input(MOUSEEVENTF_HWHEEL, (dwheel * 120) as u32, 0, 0);
    }
}

/// Starts listening for bound input events.
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
                Bind::BlockBind(cb) => {
                    let cb = Arc::clone(cb);
                    spawn(move || cb());
                    return 1;
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
        }
        _ => None,
    } {
        if let Some(bind) = MOUSE_BINDS.lock().unwrap().get_mut(&event) {
            match bind {
                Bind::NormalBind(cb) => {
                    let cb = Arc::clone(cb);
                    spawn(move || cb());
                }
                Bind::BlockBind(cb) => {
                    let cb = Arc::clone(cb);
                    spawn(move || cb());
                    return 1;
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
