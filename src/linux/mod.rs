extern crate x11;

use self::x11::xlib::*;
use self::x11::xtest::*;
use InputEvent::*;
use std::mem::{uninitialized, transmute};
use ::*;
use std::ptr::null;
use std::thread::spawn;

pub mod codes;

lazy_static! {
    static ref LBUTTON_STATE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref MBUTTON_STATE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref RBUTTON_STATE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

impl InputEvent {
    pub fn bind<F>(self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let input = fix_input_event(self);
        BINDS.lock().unwrap().insert(input, Arc::new(callback));
        if BINDS.lock().unwrap().len() != 1 {
            return;
        };
        spawn(move || while BINDS.lock().unwrap().len() != 0 {
            if let Some(event) = unsafe { get_event() } {
                if let Some(cb) = BINDS.lock().unwrap().get_mut(&event) {
                    let cb = cb.clone();
                    spawn(move || cb());
                };
            }
        });
    }

    pub fn unbind(self) {
        BINDS.lock().unwrap().remove(&self);
    }
}

fn fix_input_event(input: InputEvent) -> InputEvent {
    match input {
        PressKey(mut keysym) => PressKey(get_key_code(keysym) as u64),
        ReleaseKey(mut keysym) => ReleaseKey(get_key_code(keysym) as u64),
        _ => input
    }
}

fn get_key_code(code: u64) -> u8 {
    let display = unsafe { XOpenDisplay(null()) };
    let key_code = unsafe { XKeysymToKeycode(display, code) };
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
            AnyModifier,
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

pub unsafe fn get_event() -> Option<InputEvent> {
    let mut ev = uninitialized();
    with_display(|display| {
        let window = XDefaultRootWindow(display);
        for hotkey in BINDS.lock().unwrap().keys() {
            match hotkey {
                &PressKey(key_code) |
                &ReleaseKey(key_code) => {
                    XGrabKey(
                        display,
                        key_code as i32,
                        ShiftMask,
                        window,
                        0,
                        GrabModeAsync,
                        GrabModeAsync,
                    );
                    XGrabKey(
                        display,
                        key_code as i32,
                        0,
                        window,
                        0,
                        GrabModeAsync,
                        GrabModeAsync,
                    );
                },
                &PressLButton |
                &ReleaseLButton => grab_button(Button1, display, window),
                &PressRButton |
                &ReleaseRButton => grab_button(Button3, display, window),
                &PressMButton |
                &ReleaseMButton => grab_button(Button2, display, window),
                _ => {} 
            }
        }
        XNextEvent(display, &mut ev);
    });
    let hotkey = match ev.get_type() {
        KeyPress => Some(PressKey((ev.as_ref() as &XKeyEvent).keycode as u64)),
        KeyRelease => Some(ReleaseKey((ev.as_ref() as &XKeyEvent).keycode as u64)),
        ButtonPress => {
            match (ev.as_ref() as &XKeyEvent).keycode {
                1 => {
                    *LBUTTON_STATE.lock().unwrap() = true;
                    Some(PressLButton)
                },
                2 => {
                    *MBUTTON_STATE.lock().unwrap() = true;
                    Some(PressMButton)
                },
                3 => {
                    *RBUTTON_STATE.lock().unwrap() = true;
                    Some(PressRButton)
                },
                _ => None,
            }
        }
        ButtonRelease => {
            match (ev.as_ref() as &XKeyEvent).keycode {
                1 => {
                    *LBUTTON_STATE.lock().unwrap() = false;
                    Some(ReleaseLButton)
                },
                2 => {
                    *MBUTTON_STATE.lock().unwrap() = false;
                    Some(ReleaseMButton)
                },
                3 => {
                    *RBUTTON_STATE.lock().unwrap() = false;
                    Some(ReleaseRButton)
                },
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

pub fn keybd_press(code: u64) {
    send_keybd_input(get_key_code(code), 1);
}

pub fn keybd_release(code: u64) {
    send_keybd_input(get_key_code(code), 0);
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

pub fn is_pressed(code: u64) -> bool {
    let code = get_key_code(code);
    let mut array: [i8; 32] = [0; 32];
    with_display(|display| unsafe {
        XQueryKeymap(display, transmute(&mut array));
    });
    array[(code >> 3) as usize] & (1 << (code & 7)) != 0
}

pub fn is_pressed_lbutton() -> bool {
    unsafe{*LBUTTON_STATE.lock().unwrap()}
}

pub fn is_pressed_mbutton() -> bool {
    unsafe{*MBUTTON_STATE.lock().unwrap()}
}

pub fn is_pressed_rbutton() -> bool {
    unsafe{*RBUTTON_STATE.lock().unwrap()}
}