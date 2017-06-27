extern crate x11;

use self::x11::xlib::*;
use self::x11::xtest::*;
use Event::*;
use std::mem::uninitialized;
use std::sync::{Arc, Mutex};
use *;

pub mod codes;

lazy_static! {
    static ref STATE: Arc<Mutex<(u64, u64)>> = {unsafe{
        let display = XOpenDisplay(std::ptr::null());
        Arc::new(Mutex::new((display as u64, XDefaultRootWindow(display))))
    }};
}

fn get_display() -> *mut Display {
    STATE.lock().unwrap().0 as *mut Display
}

fn get_window() -> u64 {
    STATE.lock().unwrap().1
}

pub fn start_capture() {
    *CAPTURE_HOTKEYS.lock().unwrap() = true;
    while *CAPTURE_HOTKEYS.lock().unwrap() {
        unsafe{XGrabPointer(get_display(), get_window(), true as _,
        (ButtonPressMask |
        ButtonReleaseMask |
        PointerMotionMask) as u32,
        GrabModeAsync,
        GrabModeAsync, 0, 0, 0)};
        for hotkey in HOTKEYS.lock().unwrap().keys() {match hotkey {
            &KeybdPress(scan_code) | &KeybdRelease(scan_code) => {
                unsafe{XGrabKey
                (get_display(), scan_code as i32, 0, get_window(), false as i32, GrabModeAsync, GrabModeAsync)};
            },
            _ => {} 
        }};
        let mut ev = unsafe{uninitialized()};
        unsafe{XNextEvent(get_display(), &mut ev)};
        for hotkey in HOTKEYS.lock().unwrap().keys() {match hotkey {
            &KeybdPress(scan_code) | &KeybdRelease(scan_code) => {
                unsafe{XUngrabKey(get_display(), scan_code as i32, 0, get_window())};
            },
            _ => {} 
        }};
        if let Some(hotkey) = match ev.get_type() {
            KeyPress => Some(KeybdPress((ev.as_ref() as &XKeyEvent).keycode as u8)),
            KeyRelease => Some(KeybdRelease((ev.as_ref() as &XKeyEvent).keycode as u8)),
            ButtonPress => match (ev.as_ref() as &XKeyEvent).keycode {
                1 => Some(MousePressLeft),
                2 => Some(MousePressMiddle),
                3 => Some(MousePressRight),
                _ => None
            },
            ButtonRelease => match (ev.as_ref() as &XKeyEvent).keycode {
                1 => Some(MouseReleaseLeft),
                2 => Some(MouseReleaseMiddle),
                3 => Some(MouseReleaseRight),
                _ => None
            },
            _ => None
        } {if let Some(cb) = HOTKEYS.lock().unwrap().get_mut(&hotkey) {cb()}}
    }
}

pub fn mouse_move_to(x: i32, y: i32) {
    unsafe {
        XWarpPointer(get_display(), 0, 0, 0, 0, 0, 0, x, y);
        XFlush(get_display());
    };
}

pub fn mouse_move(x: i32, y: i32) {
    unsafe {
        XWarpPointer(get_display(), 0, 0, 0, 0, 0, 0, x, y);
        XFlush(get_display());
    }
}

fn send_mouse_input(button: u32, is_press: i32) {
    unsafe {
        XTestFakeButtonEvent(get_display(), button, is_press, 0);
        XFlush(get_display());
    }
}


fn send_keybd_input(scan_code: u8, is_press: i32) {
    unsafe {
        XTestFakeKeyEvent(get_display(), scan_code as u32, is_press, 0);
        XFlush(get_display());
    }
}

pub fn mouse_press_left() {
    send_mouse_input(1, 1);
}

pub fn mouse_release_left() {
    send_mouse_input(1, 0);
}

pub fn mouse_press_right() {
    send_mouse_input(3, 1);
}

pub fn mouse_release_right() {
    send_mouse_input(3, 0);
}

pub fn mouse_press_middle() {
    send_mouse_input(2, 1);
}

pub fn mouse_release_middle() {
    send_mouse_input(2, 0);
}

pub fn mouse_scroll_hor(dwheel: i32) {
}

pub fn mouse_scroll_ver(dwheel: i32) {
}

pub fn keybd_press(vk: Code) {
    send_keybd_input(vk, 1);
}

pub fn keybd_release(vk: Code) {
    send_keybd_input(vk, 0);
}

/// Don't know how to impliment.
pub fn is_toggled(vk_code: Code) -> bool {
    false
}

/// Don't know how to impliment.
pub fn is_pressed(vk_code: Code) -> bool {
    false
}