extern crate x11;

use self::x11::*;
use self::x11::xlib::*;
use *;
use Event::*;
use std::mem::uninitialized;
use std::collections::hash_map::HashMap;

pub mod codes;


#[link(name = "Xtst")]
extern "C" {
    fn XTestFakeButtonEvent(display: Display, keycode: KeyCode, state: Bool, delay: u64);
}

lazy_static! {
    static ref STATE: Arc<Mutex<(u64, u64)>> = {unsafe{
        let display = xlib::XOpenDisplay(std::ptr::null());
        Arc::new(Mutex::new((display as u64, xlib::XDefaultRootWindow(display))))
    }};
}

fn get_display() -> *mut Display {
    STATE.lock().unwrap().0 as *mut Display
}

fn get_window() -> u64 {
    STATE.lock().unwrap().1
}

pub fn start_capture() {
    loop {
        for hotkey in HOTKEYS.lock().unwrap().keys() {match hotkey {
            &KeybdPress(scan_code) | &KeybdRelease(scan_code)  => {
                unsafe{xlib::XGrabKey
                (get_display(), scan_code as i32, 0, get_window(), false as i32, GrabModeAsync, GrabModeAsync)};
            },
            _ => {} 
        }};
        let mut ev = unsafe{uninitialized()};
        unsafe{xlib::XNextEvent(get_display(), &mut ev)};
        for hotkey in HOTKEYS.lock().unwrap().keys() {match hotkey {
            &KeybdPress(scan_code) | &KeybdRelease(scan_code)  => {
                unsafe{xlib::XUngrabKey
                (get_display(), scan_code as i32, 0, get_window())};
            },
            _ => {} 
        }};
        if let Some(hotkey) = match ev.get_type() {
            KeyPress => {
                let ev: &XKeyEvent = ev.as_ref();
                Some(KeybdPress(ev.keycode as u8))
            },
            KeyRelease => {
                let ev: &XKeyEvent = ev.as_ref();
                Some(KeybdRelease(ev.keycode as u8))
            },
            _ => None
        } {if let Some(cb) = HOTKEYS.lock().unwrap().get_mut(&hotkey) {cb()}}
    }
}

pub fn mouse_move_to(x: i32, y: i32) {
    unsafe {
        xlib::XWarpPointer(get_display(), 0, 0, 0, 0, 0, 0, x, y);
        xlib::XFlush(get_display());
    };
}

pub fn mouse_move(x: i32, y: i32) {
    unsafe {
        xlib::XWarpPointer(get_display(), 0, 0, 0, 0, 0, 0, x, y);
        xlib::XFlush(get_display());
    }
}

fn send_mouse_input(button: u32, is_press: i32) {
    unsafe {
        xtest::XTestFakeButtonEvent(get_display(), button, is_press, 0);
        xlib::XFlush(get_display());
    }
}


fn send_keybd_input(scan_code: u8, is_press: i32) {
    unsafe {
        xtest::XTestFakeKeyEvent(get_display(), scan_code as u32, is_press, 0);
        xlib::XFlush(get_display());
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

/// Needs to be implemented, don't know how. xquerykeymap?
pub fn is_pressed(vk_code: Code) -> bool {
    false
}