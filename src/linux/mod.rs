extern crate x11;

use self::x11::xlib::*;
use self::x11::xtest::*;
use Event::*;
use std::mem::uninitialized;
use std::sync::{Arc, Mutex};
use *;
use std::cell::RefCell;
use std::ptr::null;
use codes::*;
use std::time::Duration;
use std::thread::{sleep, park};

pub mod codes;

fn with_display<F>(cb: F) where F: FnOnce(*mut Display) {
    let display = unsafe {XOpenDisplay(null())};
    cb(display);
    unsafe {XCloseDisplay(display)};
}

pub unsafe fn get_event() -> Option<Event> {
    let display = unsafe {XOpenDisplay(null())};
    let window = unsafe {XDefaultRootWindow(display)};
    for hotkey in HOTKEYS.lock().unwrap().keys() {
        match hotkey {
            &KeybdPress(scan_code) | &KeybdRelease(scan_code) => {
                unsafe{XGrabKey
                (display, scan_code as i32, 0, window, false as i32, GrabModeAsync, GrabModeAsync)};
            },
            _ => {} 
        }
    };
    let mut ev = unsafe{uninitialized()};
    unsafe {XNextEvent(display, &mut ev)};
    unsafe {XCloseDisplay(display)};
    let hotkey = match ev.get_type() {
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
    };
    hotkey
}

pub fn start_capture() {
    //unsafe{XGrabPointer(get_display(), get_window(), true as _,
    //(ButtonPressMask |
    //ButtonReleaseMask) as u32,
    //GrabModeAsync,
    //GrabModeAsync, 0, 0, 0)};
}

pub fn stop_capture() {
}

pub fn mouse_move_to(x: i32, y: i32) {
    with_display(|display| unsafe {XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);});
}

pub fn mouse_move(x: i32, y: i32) {
    with_display(|display| unsafe {XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);});
}

fn send_mouse_input(button: u32, is_press: i32) {
    with_display(|display| unsafe {XTestFakeButtonEvent(display, button, is_press, 0);});
}

fn send_keybd_input(scan_code: u8, is_press: i32) {
    with_display(|display| unsafe {XTestFakeKeyEvent(display, scan_code as u32, is_press, 0);});
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


// unimplimented
pub fn mouse_scroll_hor(dwheel: i32) {
}

// unimplimented
pub fn mouse_scroll_ver(dwheel: i32) {
}

pub fn keybd_press(vk: Code) {
    send_keybd_input(vk, 1);
}

pub fn keybd_release(vk: Code) {
    send_keybd_input(vk, 0);
}

// unimplimented
pub fn is_toggled(vk_code: Code) -> bool {
    false
}

// unimplimented
pub fn is_pressed(vk_code: Code) -> bool {
    false
}