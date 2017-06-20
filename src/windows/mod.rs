//! # How-To
//! Hotkeys can be created by matching input within a capture loop.
//! 
//! The code below demonstrates how to create a rapidfire hotkey for videogames.
//! 
//! ```
//! extern crate inputbot;
//! 
//! use inputbot::*;
//! use Hotkey::*;
//! use KeybdHotkeyType::*;
//! use MouseHotkeyType::*;
//! use std::time::Duration;
//! use std::thread::sleep;
//! 
//! fn main() {
//!     register(MouseHotkey(PressRight), || {
//!         while get_logical_state(vk::RBUTTON) {
//!             mouse_press_left();
//!             sleep(Duration::from_millis(50));
//!             mouse_release_left();
//!         }
//!     });
//!     capture_input();
//! }
//! ```
//! 
//! Check out the [examples](https://github.com/obv-mikhail/InputBot/tree/master/examples) for more code samples, or read the documentation.

extern crate winapi;
extern crate user32;

use self::winapi::*;
use self::user32::*;
use Hotkey;
use std::mem::{transmute_copy, transmute, size_of, uninitialized};
use std::sync::{Arc, Mutex};
use std::collections::hash_map::HashMap;
use std::ops::{FnOnce, FnMut};
use std::marker::Send;

pub mod vk;

pub type VKCode = u8;

lazy_static! {
    static ref HOTKEYS: Arc<Mutex<HashMap<Hotkey, Box<FnMut() + Send + 'static>>>> = Arc::new(Mutex::new(HashMap::<Hotkey, Box<FnMut() + Send + 'static>>::new()));
}

pub fn register<F>(hotkey: Hotkey, callback: F) where for<'r> F: FnOnce() + 'static + Send + FnMut() {
    HOTKEYS.lock().unwrap().insert(hotkey, Box::new(callback));
}

pub fn unregister(hotkey: Hotkey) {
    HOTKEYS.lock().unwrap().remove(&hotkey);
}

static mut KEYBD_HHOOK: HHOOK = 0 as HHOOK;
static mut MOUSE_HHOOK: HHOOK = 0 as HHOOK;

unsafe extern "system" fn hhook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    PostMessageW(0 as HWND, 0, w_param, l_param);
    CallNextHookEx(KEYBD_HHOOK, code, w_param, l_param)
}

pub fn capture_input() {
    use Hotkey::*;
    use KeybdHotkeyType::*;
    use MouseHotkeyType::*;
    loop {
        let msg = unsafe {
            KEYBD_HHOOK = SetWindowsHookExW(13, Some(hhook_proc), 0 as HINSTANCE, 0);
            MOUSE_HHOOK = SetWindowsHookExW(14, Some(hhook_proc), 0 as HINSTANCE, 0);
            let mut msg: MSG = uninitialized();
            GetMessageW(&mut msg, 0 as HWND, 0, 0);
            UnhookWindowsHookEx(KEYBD_HHOOK);
            UnhookWindowsHookEx(MOUSE_HHOOK);
            KEYBD_HHOOK = 0 as HHOOK;
            MOUSE_HHOOK = 0 as HHOOK;
            msg
        };
        let hotkey = match msg.wParam {
            256 => KeybdHotkey(Press, unsafe{*(msg.lParam as *const KBDLLHOOKSTRUCT)}.vkCode as u8),
            257 => KeybdHotkey(Release, unsafe{*(msg.lParam as *const KBDLLHOOKSTRUCT)}.vkCode as u8),
            512...522 => {
                MouseHotkey(match msg.wParam {
                    512 => Move,
                    513 => PressLeft,
                    514 => ReleaseLeft,
                    516 => PressRight,
                    517 => ReleaseRight,
                    519 => PressMiddle,
                    520 => ReleaseMiddle,
                    522 => Scroll,
                    _ => Move
                })
            },
            _ => MouseHotkey(Move)
        };
        if let Some(func) = HOTKEYS.lock().unwrap().get_mut(&hotkey) {func()}
    }
}

fn send_mouse_input(flags: u32, data: u32, dx: i32, dy: i32) {
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe{transmute(MOUSEINPUT {
            dx: dx,
            dy: dy,
            mouseData: data,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0,
        })},
    };
    unsafe{SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int)};
}

fn send_keybd_input(flags: u32, vk: u8) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe{transmute_copy(&KEYBDINPUT {
            wVk: 0,
            wScan: MapVirtualKeyW(vk as u32, 0) as u16,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0,
        })}
    };
    unsafe{SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int)};
}

pub fn mouse_move(dx: i32, dy: i32) {
    send_mouse_input(MOUSEEVENTF_MOVE, 0, dx, dy);
}

pub fn mouse_move_to(x: i32, y: i32) {
    unsafe{send_mouse_input(
        MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, 
        0, 
        x*65335/GetSystemMetrics(78),
        y*65335/GetSystemMetrics(79)
    )};
}

pub fn mouse_press_left() {
    send_mouse_input(MOUSEEVENTF_LEFTDOWN, 0, 0, 0);
}

pub fn mouse_release_left() {
    send_mouse_input(MOUSEEVENTF_LEFTUP, 0, 0, 0);
}

pub fn mouse_press_right() {
    send_mouse_input(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0);
}

pub fn mouse_release_right() {
    send_mouse_input(MOUSEEVENTF_RIGHTUP, 0, 0, 0);
}

pub fn mouse_press_middle() {
    send_mouse_input(MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0);
}

pub fn mouse_release_middle() {
    send_mouse_input(MOUSEEVENTF_MIDDLEUP, 0, 0, 0);
}

pub fn mouse_scroll_hor(dwheel: i32) {
    send_mouse_input(MOUSEEVENTF_HWHEEL, unsafe{transmute(dwheel*120)}, 0, 0);
}

pub fn mouse_scroll_ver(dwheel: i32) {
    send_mouse_input(MOUSEEVENTF_WHEEL, unsafe{transmute(dwheel*120)}, 0, 0);
}

pub fn keybd_press(vk: VKCode) {
    send_keybd_input(KEYEVENTF_SCANCODE, vk);
}

pub fn keybd_release(vk: VKCode) {
    send_keybd_input(KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, vk);
}

/// Teturns the toggle state for NumLock, CapsLock, and ScrollLock.
pub fn get_toggle_state(vk_code: VKCode) -> bool {
    unsafe {GetKeyState(vk_code as i32) & 15 != 0}
}

/// Returns the logical state (pressed | not pressed).
pub fn get_logical_state(vk_code: VKCode) -> bool {
    match unsafe {GetAsyncKeyState(vk_code as i32)} {-32767 | -32768 => true, _ => false}
}