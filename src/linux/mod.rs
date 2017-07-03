extern crate x11;

use self::x11::xlib::*;
use self::x11::xtest::*;
use Event::*;
use std::mem::{uninitialized, transmute};
use ::*;
use std::ptr::null;

pub mod codes;

impl Event {
    pub fn bind<F>(self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        HOTKEYS.lock().unwrap().insert(self, Arc::new(callback));
        if HOTKEYS.lock().unwrap().len() != 1 {
            return;
        };
        spawn(move || while HOTKEYS.lock().unwrap().len() != 0 {
            if let Some(event) = unsafe { get_event() } {
                if let Some(cb) = HOTKEYS.lock().unwrap().get_mut(&event) {
                    let cb = cb.clone();
                    spawn(move || cb());
                };
            }
        });
    }

    pub fn unbind(self) {
        HOTKEYS.lock().unwrap().remove(&self);
    }
}

fn get_key_code(code: Code) -> u8 {
    let display = unsafe { XOpenDisplay(null()) };
    let key_code = unsafe { XKeysymToKeycode(display, code as _) };
    unsafe { XCloseDisplay(display) };
    key_code
}

fn with_display<F>(cb: F)
where
    F: FnOnce(*mut Display),
{
    let display = unsafe { XOpenDisplay(null()) };
    cb(display);
    unsafe { XCloseDisplay(display) };
}

fn grab_button(button: u32, display: *mut Display, window: u64) {
    unsafe {
        XGrabButton(
            display,
            button,
            0,
            window,
            0,
            (ButtonPressMask | ButtonReleaseMask) as u32,
            GrabModeAsync,
            GrabModeAsync,
            0,
            0,
        );
    }
}

pub unsafe fn get_event() -> Option<Event> {
    let mut ev = uninitialized();
    with_display(|display| {
        let window = XDefaultRootWindow(display);
        for hotkey in HOTKEYS.lock().unwrap().keys() {
            match hotkey {
                &KeybdPress(key_code) |
                &KeybdRelease(key_code) => {
                    XGrabKey(
                        display,
                        key_code as i32,
                        0,
                        window,
                        0,
                        GrabModeAsync,
                        GrabModeAsync,
                    );
                }
                &MousePressLeft |
                &MouseReleaseLeft => grab_button(Button1, display, window),
                &MousePressRight |
                &MouseReleaseRight => grab_button(Button3, display, window),
                &MousePressMiddle |
                &MouseReleaseMiddle => grab_button(Button2, display, window),
                _ => {} 
            }
        }
        XNextEvent(display, &mut ev);
    });
    let hotkey = match ev.get_type() {
        KeyPress => Some(KeybdPress((ev.as_ref() as &XKeyEvent).keycode as u8)),
        KeyRelease => Some(KeybdRelease((ev.as_ref() as &XKeyEvent).keycode as u8)),
        ButtonPress => {
            match (ev.as_ref() as &XKeyEvent).keycode {
                1 => Some(MousePressLeft),
                2 => Some(MousePressMiddle),
                3 => Some(MousePressRight),
                _ => None,
            }
        }
        ButtonRelease => {
            match (ev.as_ref() as &XKeyEvent).keycode {
                1 => Some(MouseReleaseLeft),
                2 => Some(MouseReleaseMiddle),
                3 => Some(MouseReleaseRight),
                _ => None,
            }
        }
        _ => None,
    };
    hotkey
}

pub fn mouse_move_to(x: i32, y: i32) {
    with_display(|display| unsafe {
        XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
    });
}

pub fn mouse_move(x: i32, y: i32) {
    with_display(|display| unsafe {
        XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
    });
}

fn send_mouse_input(button: u32, is_press: i32) {
    with_display(|display| unsafe {
        XTestFakeButtonEvent(display, button, is_press, 0);
    });
}

fn send_keybd_input(code: u8, is_press: i32) {
    with_display(|display| unsafe {
        XTestFakeKeyEvent(display, code as u32, is_press, 0);
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

pub fn keybd_press(code: Code) {
    send_keybd_input(code, 1);
}

pub fn keybd_release(code: Code) {
    send_keybd_input(code, 0);
}

pub fn num_lock_is_toggled() -> bool {
    let mut state: XKeyboardState = unsafe { uninitialized() };
    with_display(|display| unsafe {
        XGetKeyboardControl(display, &mut state);
    });
    (state.led_mask & 2 != 0)
}

pub fn caps_lock_is_toggled() -> bool {
    let mut state: XKeyboardState = unsafe { uninitialized() };
    with_display(|display| unsafe {
        XGetKeyboardControl(display, &mut state);
    });
    (state.led_mask & 1 != 0)
}

pub fn is_pressed(code: Code) -> bool {
    let mut array: [i8; 32] = [0; 32];
    with_display(|display| unsafe {
        XQueryKeymap(display, transmute(&mut array));
    });
    array[(code >> 3) as usize] & (1 << (code & 7)) != 0
}
