extern crate x11;

use self::x11::xlib::*;
use self::x11::xtest::*;
use std::mem::uninitialized;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::thread::spawn;
use std::ptr::null;
use ::*;

pub mod inputs;

type KeybdBindMap = Mutex<HashMap<u64, Arc<Fn() + Send + Sync + 'static>>>;
type MouseBindMap = Mutex<HashMap<MouseButton, Arc<Fn() + Send + Sync + 'static>>>;

lazy_static! {
    static ref BUTTON_STATES: Mutex<HashMap<MouseButton, bool>> = Mutex::new(HashMap::<MouseButton, bool>::new());
    static ref KEYBD_BINDS: KeybdBindMap = Mutex::new(HashMap::<u64, Arc<Fn() + Send + Sync + 'static>>::new());
    static ref MOUSE_BINDS: MouseBindMap = Mutex::new(HashMap::<MouseButton, Arc<Fn() + Send + Sync + 'static>>::new());
    static ref SEND_DISPLAY: AtomicPtr<Display> = {
        unsafe { XInitThreads() };
        AtomicPtr::new(unsafe { XOpenDisplay(null()) })
    };
    static ref RECV_DISPLAY: AtomicPtr<Display> = {
        unsafe { XInitThreads() };
        AtomicPtr::new(unsafe { XOpenDisplay(null()) })
    };
}

impl KeybdKey {
    pub fn bind<F>(self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let input = u64::from(get_key_code(self as u64));
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(input, Arc::new(callback));
        RECV_DISPLAY.with(|display| {
            let window = unsafe { XDefaultRootWindow(display) };
            grab_key(input as i32, ShiftMask, display, window);
            grab_key(input as i32, 0, display, window);
        });
        handle_events();
    }

    pub fn unbind(self) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .remove(&u64::from(get_key_code(self as u64)));
    }

    pub fn is_pressed(self) -> bool {
        let code = get_key_code(self as _);
        let mut array: [i8; 32] = [0; 32];
        SEND_DISPLAY.with(|display| unsafe {
            XQueryKeymap(display, &mut array as *mut [i8; 32] as *mut i8);
        });
        array[(code >> 3) as usize] & (1 << (code & 7)) != 0
    }

    pub fn press(self) {
        send_keybd_input(get_key_code(self as _), 1);
    }

    pub fn release(self) {
        send_keybd_input(get_key_code(self as _), 0);
    }
}

impl MouseButton {
    pub fn bind<F>(self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        MOUSE_BINDS.lock().unwrap().insert(self, Arc::new(callback));
        RECV_DISPLAY.with(|display| {
            let window = unsafe { XDefaultRootWindow(display) };
            grab_button(u32::from(self), display, window);
        });
        handle_events();
    }

    pub fn unbind(self) {
        MOUSE_BINDS.lock().unwrap().remove(&self);
    }

    pub fn is_pressed(self) -> bool {
        *BUTTON_STATES.lock().unwrap().entry(self).or_insert(false)
    }

    pub fn press(self) {
        send_mouse_input(u32::from(self), 1);
    }

    pub fn release(self) {
        send_mouse_input(u32::from(self), 0);
    }
}

fn get_key_code(code: u64) -> u8 {
    SEND_DISPLAY.with(|display| unsafe { XKeysymToKeycode(display, code) })
}

trait DisplayAcquirable {
    fn with<F, Z>(&self, cb: F) -> Z
    where
        F: FnOnce(*mut Display) -> Z;
}

impl DisplayAcquirable for AtomicPtr<Display> {
    fn with<F, Z>(&self, cb: F) -> Z
    where
        F: FnOnce(*mut Display) -> Z,
    {
        let display = self.load(Ordering::Relaxed);
        unsafe {
            XLockDisplay(display);
        };
        let cb_result = cb(display);
        unsafe {
            XFlush(display);
            XUnlockDisplay(display);
        };
        cb_result
    }
}

fn grab_button(button: u32, display: *mut Display, window: u64) {
    unsafe {
        XGrabButton(
            display,
            button,
            AnyModifier,
            window,
            1,
            (ButtonPressMask | ButtonReleaseMask) as u32,
            GrabModeAsync,
            GrabModeAsync,
            0,
            0,
        );
    }
}

fn grab_key(key: i32, mask: u32, display: *mut Display, window: u64) {
    unsafe {
        XGrabKey(display, key, mask, window, 0, GrabModeAsync, GrabModeAsync);
    }
}

fn handle_events() {
    if (KEYBD_BINDS.lock().unwrap().len() + MOUSE_BINDS.lock().unwrap().len()) == 1 {
        spawn(move || {
            while (KEYBD_BINDS.lock().unwrap().len() + MOUSE_BINDS.lock().unwrap().len()) != 0 {
                unsafe { handle_event() };
            }
        });
    };
}

#[allow(non_upper_case_globals)]
unsafe fn handle_event() {
    let mut ev = uninitialized();
    RECV_DISPLAY.with(|display| XNextEvent(display, &mut ev));
    match ev.get_type() {
        KeyPress => {
            if let Some(cb) = KEYBD_BINDS
                .lock()
                .unwrap()
                .get_mut(&u64::from((ev.as_ref() as &XKeyEvent).keycode))
            {
                let cb = Arc::clone(cb);
                spawn(move || cb());
            };
        }
        ButtonPress => {
            let mouse_button = MouseButton::from((ev.as_ref() as &XKeyEvent).keycode);
            BUTTON_STATES.lock().unwrap().insert(mouse_button, true);
            if let Some(cb) = MOUSE_BINDS.lock().unwrap().get_mut(&mouse_button) {
                let cb = Arc::clone(cb);
                spawn(move || cb());
            };
        }
        ButtonRelease => {
            BUTTON_STATES.lock().unwrap().insert(
                MouseButton::from((ev.as_ref() as &XKeyEvent).keycode),
                false,
            );
        }
        _ => {}
    };
}

pub fn mouse_move_to(x: i32, y: i32) {
    SEND_DISPLAY.with(|display| unsafe {
        XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
    });
}

pub fn mouse_move(x: i32, y: i32) {
    SEND_DISPLAY.with(|display| unsafe {
        XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
    });
}

fn send_mouse_input(button: u32, is_press: i32) {
    SEND_DISPLAY.with(|display| unsafe {
        XTestFakeButtonEvent(display, button, is_press, 0);
    });
}

fn send_keybd_input(code: u8, is_press: i32) {
    SEND_DISPLAY.with(|display| unsafe {
        XTestFakeKeyEvent(display, u32::from(code), is_press, 0);
    });
}

pub fn num_lock_is_toggled() -> bool {
    let mut state: XKeyboardState = unsafe { uninitialized() };
    SEND_DISPLAY.with(|display| unsafe {
        XGetKeyboardControl(display, &mut state);
    });
    (state.led_mask & 2 != 0)
}

pub fn caps_lock_is_toggled() -> bool {
    let mut state: XKeyboardState = unsafe { uninitialized() };
    SEND_DISPLAY.with(|display| unsafe {
        XGetKeyboardControl(display, &mut state);
    });
    (state.led_mask & 1 != 0)
}
