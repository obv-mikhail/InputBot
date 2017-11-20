extern crate x11;

use self::x11::xlib::*;
use self::x11::xtest::*;
use std::mem::uninitialized;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread::spawn;
use std::ptr::null;
use ::*;

pub mod inputs;

type KeybdBindMap = Mutex<HashMap<u64, Arc<Fn() + Send + Sync + 'static>>>;
type MouseBindMap = Mutex<HashMap<MouseButton, Arc<Fn() + Send + Sync + 'static>>>;

lazy_static! {
    static ref LBUTTON_STATE: AtomicBool = AtomicBool::new(false);
    static ref MBUTTON_STATE: AtomicBool = AtomicBool::new(false);
    static ref RBUTTON_STATE: AtomicBool = AtomicBool::new(false);
    static ref KEYBD_BINDS: KeybdBindMap = Mutex::new(HashMap::<u64, Arc<Fn() + Send + Sync + 'static>>::new());
    static ref MOUSE_BINDS: MouseBindMap = Mutex::new(HashMap::<MouseButton, Arc<Fn() + Send + Sync + 'static>>::new());
    static ref SEND_DISPLAY: Arc<Mutex<u64>> = {
        unsafe{
            XInitThreads();
            Arc::new(Mutex::new(XOpenDisplay(null()) as u64))
        }
    };
    static ref RECV_DISPLAY: Arc<Mutex<u64>> = {
        unsafe{
            XInitThreads();
            Arc::new(Mutex::new(XOpenDisplay(null()) as u64))
        }
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
        with_recv_display(|display| {
            let window = unsafe { XDefaultRootWindow(display) };
            unsafe {
                XGrabKey(
                    display,
                    input as i32,
                    ShiftMask,
                    window,
                    0,
                    GrabModeAsync,
                    GrabModeAsync,
                )
            };
            unsafe {
                XGrabKey(
                    display,
                    input as i32,
                    0,
                    window,
                    0,
                    GrabModeAsync,
                    GrabModeAsync,
                )
            };
        });
        if (KEYBD_BINDS.lock().unwrap().len() + MOUSE_BINDS.lock().unwrap().len()) != 1 {
            return;
        };
        spawn(move || {
            while KEYBD_BINDS.lock().unwrap().len() != 0 {
                unsafe { handle_event() };
            }
        });
    }

    pub fn unbind(self) {
        let input = u64::from(get_key_code(self as u64));
        KEYBD_BINDS.lock().unwrap().remove(&input);
    }

    pub fn is_pressed(self) -> bool {
        let code = get_key_code(self as _);
        let mut array: [i8; 32] = [0; 32];
        with_send_display(|display| unsafe {
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
        with_recv_display(|display| {
            let window = unsafe { XDefaultRootWindow(display) };
            grab_button(self as _, display, window);
        });
        if (KEYBD_BINDS.lock().unwrap().len() + MOUSE_BINDS.lock().unwrap().len()) != 1 {
            return;
        };
        spawn(move || {
            while KEYBD_BINDS.lock().unwrap().len() != 0 {
                unsafe { handle_event() };
            }
        });
    }

    pub fn unbind(self) {
        let input = u64::from(get_key_code(self as u64));
        KEYBD_BINDS.lock().unwrap().remove(&input);
    }

    pub fn is_pressed(self) -> bool {
        match self {
            MouseButton::LeftButton => LBUTTON_STATE.load(Ordering::Relaxed),
            MouseButton::RightButton => RBUTTON_STATE.load(Ordering::Relaxed),
            MouseButton::MiddleButton => MBUTTON_STATE.load(Ordering::Relaxed),
            _ => false,
        }
    }

    pub fn press(self) {
        send_mouse_input(self as _, 1);
    }

    pub fn release(self) {
        send_mouse_input(self as _, 0);
    }
}

fn get_key_code(code: u64) -> u8 {
    with_send_display(|display| unsafe { XKeysymToKeycode(display, code) })
}

fn with_send_display<F, Z>(cb: F) -> Z
where
    F: FnOnce(*mut Display) -> Z,
{
    let display = *SEND_DISPLAY.lock().unwrap() as *mut Display;
    unsafe {
        XLockDisplay(display);
    };
    let cb_result = cb(display);
    unsafe {
        XUnlockDisplay(display);
        XFlush(display)
    };
    cb_result
}

fn with_recv_display<F>(cb: F)
where
    F: FnOnce(*mut Display),
{
    let display = *RECV_DISPLAY.lock().unwrap() as *mut Display;
    unsafe {
        XLockDisplay(display);
    };
    cb(display);
    unsafe {
        XUnlockDisplay(display);
        XFlush(display)
    };
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

#[allow(non_upper_case_globals)]
unsafe fn handle_event() {
    let mut ev = uninitialized();
    with_recv_display(|display| {
        XNextEvent(display, &mut ev);
    });
    let mut keybd_key: Option<u64> = None;
    let mut mouse_button: Option<MouseButton> = None;
    match ev.get_type() {
        KeyPress => keybd_key = Some(u64::from((ev.as_ref() as &XKeyEvent).keycode)),
        ButtonPress => match (ev.as_ref() as &XKeyEvent).keycode {
            1 => {
                LBUTTON_STATE.store(true, Ordering::Relaxed);
                mouse_button = Some(MouseButton::LeftButton)
            }
            2 => {
                MBUTTON_STATE.store(true, Ordering::Relaxed);
                mouse_button = Some(MouseButton::MiddleButton)
            }
            3 => {
                RBUTTON_STATE.store(true, Ordering::Relaxed);
                mouse_button = Some(MouseButton::RightButton)
            }
            _ => {}
        },
        ButtonRelease => match (ev.as_ref() as &XKeyEvent).keycode {
            1 => LBUTTON_STATE.store(false, Ordering::Relaxed),
            2 => MBUTTON_STATE.store(false, Ordering::Relaxed),
            3 => RBUTTON_STATE.store(false, Ordering::Relaxed),
            _ => {}
        },
        _ => {}
    };
    if let Some(event) = keybd_key {
        if let Some(cb) = KEYBD_BINDS.lock().unwrap().get_mut(&event) {
            let cb = Arc::clone(cb);
            spawn(move || cb());
        };
    }
    if let Some(event) = mouse_button {
        if let Some(cb) = MOUSE_BINDS.lock().unwrap().get_mut(&event) {
            let cb = Arc::clone(cb);
            spawn(move || cb());
        };
    }
}

pub fn mouse_move_to(x: i32, y: i32) {
    with_send_display(|display| unsafe {
        XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
    });
}

pub fn mouse_move(x: i32, y: i32) {
    with_send_display(|display| unsafe {
        XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
    });
}

fn send_mouse_input(button: u32, is_press: i32) {
    with_send_display(|display| unsafe {
        XTestFakeButtonEvent(display, button, is_press, 0);
    });
}

fn send_keybd_input(code: u8, is_press: i32) {
    with_send_display(|display| unsafe {
        XTestFakeKeyEvent(display, u32::from(code), is_press, 0);
    });
}

pub fn num_lock_is_toggled() -> bool {
    let mut state: XKeyboardState = unsafe { uninitialized() };
    with_send_display(|display| unsafe {
        XGetKeyboardControl(display, &mut state);
    });
    (state.led_mask & 2 != 0)
}


pub fn caps_lock_is_toggled() -> bool {
    let mut state: XKeyboardState = unsafe { uninitialized() };
    with_send_display(|display| unsafe {
        XGetKeyboardControl(display, &mut state);
    });
    (state.led_mask & 1 != 0)
}
