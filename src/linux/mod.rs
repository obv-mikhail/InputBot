extern crate x11_dl;

use self::x11_dl::xtest::*;
use self::x11_dl::xlib::*;
use self::x11_dl::error::OpenError;
use Hotkey;
use std;
use std::mem::uninitialized;
use std::collections::hash_map::HashMap;
use std::ops::FnMut;
use std::cell::RefCell;
use Hotkey::*;
use KeybdHotkeyType::*;
use MouseHotkeyType::*;

/// http://wiki.linuxquestions.org/wiki/List_of_keysyms
pub type VKCode = u8;

thread_local! {
    static STATE: Result<(
        Xlib, *mut Display, u64, Xf86vmode,
        RefCell<HashMap<Hotkey, Box<FnMut() + 'static>>>
    ), OpenError> = {
        let xlib = Xlib::open()?;
        let display = unsafe{(xlib.XOpenDisplay)(std::ptr::null())};
        let window = unsafe{(xlib.XDefaultRootWindow)(display)};
        let xf86vmode = Xf86vmode::open()?;
        Ok((
            xlib, display, window, xf86vmode, 
            RefCell::new(HashMap::<Hotkey, Box<FnMut() + 'static>>::new())
        ))
    };
}

pub fn register<F>(hotkey: Hotkey, callback: F) where F: 'static + FnMut() {
    STATE.with(|state| if let &Ok((ref xlib, display, window, ref xf86vmode, ref hotkeys)) = state {
        let hotkey = match &hotkey {
            &KeybdHotkey(ref type_, scan_code) => {
                KeybdHotkey(match type_ {&Press => Press, _ => Release},
                    unsafe{(xlib.XKeysymToKeycode)(display, scan_code as _) as u32} as u8)
            },
            _ => {hotkey}
        };
        hotkeys.borrow_mut().insert(hotkey, Box::new(callback));
    });
}

pub fn unregister(hotkey: Hotkey) {
    STATE.with(|state| if let &Ok((ref xlib, display, window, ref xf86vmode, ref hotkeys)) = state {
        hotkeys.borrow_mut().remove(&hotkey);
    });
}

pub fn capture_input() {
    STATE.with(|state| if let &Ok((ref xlib, display, window, ref xf86vmode, ref hotkeys)) = state {
        for hotkey in hotkeys.borrow().keys() {match hotkey {
            &KeybdHotkey(_, scan_code) => {
                unsafe{(xlib.XGrabKey)
                (display, scan_code as i32, 0, window, false as i32, GrabModeAsync, GrabModeAsync)};
            },
            &MouseHotkey(_) => {} 
        }};
        loop {
            let mut ev = unsafe{uninitialized()};
            unsafe{(xlib.XNextEvent)(display, &mut ev)};
            let hotkey = match ev.get_type() {
                KeyPress => {
                    let ev: &XKeyEvent = ev.as_ref();
                    KeybdHotkey(Press, ev.keycode as u8)
                },
                KeyRelease => {
                    let ev: &XKeyEvent = ev.as_ref();
                    KeybdHotkey(Release, ev.keycode as u8)
                },
                _ => MouseHotkey(Move)
            };
            if let Some(func) = hotkeys.borrow_mut().get_mut(&hotkey) {func()}
        }
    });
}

pub fn mouse_move_to(x: i32, y: i32) {
    STATE.with(|state| if let &Ok((ref xlib, display, window, ref xf86vmode, ref hotkeys)) = state {
        unsafe {
            (xlib.XWarpPointer)(display, 0, 0, 0, 0, 0, 0, x, y);
            (xlib.XFlush)(display);
        };
    });
}

pub fn mouse_move(x: i32, y: i32) {
    STATE.with(|state| if let &Ok((ref xlib, display, window, ref xf86vmode, ref hotkeys)) = state {
        unsafe {
            (xlib.XWarpPointer)(display, 0, 0, 0, 0, 0, 0, x, y);
            (xlib.XFlush)(display);
        }
    });
}

fn send_mouse_input(first: u32, second: i32) {
    STATE.with(|state| if let &Ok((ref xlib, display, window, ref xf86vmode, ref hotkeys)) = state {
        unsafe {
            (xf86vmode.XTestFakeButtonEvent)(display, first, second, 0);
            (xlib.XFlush)(display);
        }
    });
}

/// Doesn't work for some reason.
fn send_keybd_input(first: u8, second: i32) {
    STATE.with(|state| if let &Ok((ref xlib, display, window, ref xf86vmode, ref hotkeys)) = state {
        let scan_code = unsafe{(xlib.XKeysymToKeycode)(display, first as _) as u32};
        println!("{}", scan_code);
        unsafe {
            (xf86vmode.XTestFakeKeyEvent)(display, scan_code, second, 0);
            (xlib.XFlush)(display);
        }
    });
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

pub fn keybd_press(vk: VKCode) {
    send_keybd_input(vk, 1);
}

pub fn keybd_release(vk: VKCode) {
    send_keybd_input(vk, 0);
}

/// Needs to be implemented.
pub fn get_toggle_state(vk_code: VKCode) -> bool {
    false
}

/// Needs to be implemented.
pub fn get_logical_state(vk_code: VKCode) -> bool {
    false
}