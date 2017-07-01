extern crate x11;

use self::x11::xlib::*;
use self::x11::xtest::*;
use Event::*;
use std::mem::uninitialized;
use std::sync::{Arc, Mutex};
use *;
use std::cell::RefCell;
use std::ptr::null;

pub mod codes;

lazy_static! {
    static ref STATE: (u64, u64) = {unsafe{
        let display = XOpenDisplay(null());
        (display as u64, XDefaultRootWindow(display))
    }};
    static ref STATE2: (u64, u64) = {unsafe{
        let display = XOpenDisplay(null());
        (display as u64, XDefaultRootWindow(display))
    }};
}

fn get_display() -> *mut Display {
    STATE.0 as *mut Display
}

fn get_display2() -> *mut Display {
    STATE2.0 as *mut Display
}

fn get_window() -> u64 {
    STATE.1
}

pub unsafe fn get_event() -> Option<Event> {
    for hotkey in HOTKEYS.lock().unwrap().keys() {match hotkey {
        &KeybdPress(scan_code) | &KeybdRelease(scan_code) => {
            unsafe{XGrabKey
            (get_display(), scan_code as i32, AnyModifier, get_window(), false as i32, GrabModeAsync, GrabModeAsync)};
        },
        _ => {} 
    }};
    XSelectInput(get_display(), get_window(), KeyPressMask | KeyReleaseMask);
    let mut ev = unsafe{uninitialized()};
    unsafe{XNextEvent(get_display(), &mut ev)};
    for hotkey in HOTKEYS.lock().unwrap().keys() {match hotkey {
        &KeybdPress(scan_code) | &KeybdRelease(scan_code) => {
            unsafe{XUngrabKey(get_display(), scan_code as i32, 0, get_window())};
        },
        _ => {} 
    }};
    match ev.get_type() {
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
    }
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
    unsafe {
        XWarpPointer(get_display2(), 0, 0, 0, 0, 0, 0, x, y);
        XFlush(get_display2());
    };
}

pub fn mouse_move(x: i32, y: i32) {
    unsafe {
        XWarpPointer(get_display2(), 0, 0, 0, 0, 0, 0, x, y);
        XFlush(get_display2());
    }
}

fn send_mouse_input(button: u32, is_press: i32) {
    unsafe {
        XTestFakeButtonEvent(get_display2(), button, is_press, 0);
        XFlush(get_display2());
    }
}


fn send_keybd_input(scan_code: u8, is_press: i32) {
    unsafe {
        XTestFakeKeyEvent(get_display2(), scan_code as u32, is_press, 0);
        XFlush(get_display2());
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