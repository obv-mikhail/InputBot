extern crate winapi;
extern crate user32;

use self::winapi::*;
use self::user32::*;
use *;
use Event::*;
use std::mem::{transmute_copy, transmute, size_of, uninitialized};

pub mod codes;

static mut KEYBD_HHOOK: HHOOK = 0 as HHOOK;
static mut MOUSE_HHOOK: HHOOK = 0 as HHOOK;

unsafe extern "system" fn hhook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    PostMessageW(0 as HWND, 0, w_param, l_param);
    CallNextHookEx(KEYBD_HHOOK, code, w_param, l_param)
}

pub fn start_capture() {
    *CAPTURE_HOTKEYS.lock().unwrap() = true;
    while *CAPTURE_HOTKEYS.lock().unwrap() {
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
        if let Some(hotkey) = match msg.wParam as u32 {
            WM_KEYDOWN => Some(KeybdPress(unsafe{*(msg.lParam as *const KBDLLHOOKSTRUCT)}.vkCode as u8)),
            WM_KEYUP => Some(KeybdRelease(unsafe{*(msg.lParam as *const KBDLLHOOKSTRUCT)}.vkCode as u8)),
            WM_MOUSEMOVE => Some(MouseMove),
            WM_LBUTTONDOWN => Some(MousePressLeft),
            WM_LBUTTONUP => Some(MouseReleaseLeft),
            WM_RBUTTONDOWN => Some(MousePressRight),
            WM_RBUTTONUP => Some(MouseReleaseRight),
            WM_MBUTTONDOWN => Some(MousePressMiddle),
            WM_MBUTTONUP => Some(MouseReleaseMiddle),
            WM_MOUSEWHEEL => Some(MouseScroll),
            _ => None
        } {if let Some(cb) = HOTKEYS.lock().unwrap().get_mut(&hotkey) {cb()}}
    }
}

pub fn stop_capture() {
    *CAPTURE_HOTKEYS.lock().unwrap() = false;
}

fn send_mouse_input(flags: u32, data: u32, dx: i32, dy: i32) {
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe{transmute_copy(&MOUSEINPUT {
            dx: dx,
            dy: dy,
            mouseData: data,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0,
        })}
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

pub fn keybd_press(vk: Code) {
    send_keybd_input(KEYEVENTF_SCANCODE, vk);
}

pub fn keybd_release(vk: Code) {
    send_keybd_input(KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, vk);
}

pub fn is_toggled(vk_code: Code) -> bool {
    unsafe {GetKeyState(vk_code as i32) & 15 != 0}
}

pub fn is_pressed(vk_code: Code) -> bool {
    match unsafe {GetAsyncKeyState(vk_code as i32)} {-32767 | -32768 => true, _ => false}
}