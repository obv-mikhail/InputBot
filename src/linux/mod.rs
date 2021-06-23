use crate::{common::*, linux::inputs::*, public::*};
use input::{
    event::{
        keyboard::{
            KeyState, {KeyboardEvent, KeyboardEventTrait},
        },
        pointer::{ButtonState, PointerEvent::*},
        Event::{self, *},
    },
    Libinput, LibinputInterface,
};
use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::close,
};
use std::{
    os::{
        unix::io::RawFd,
        raw::c_char,
    },
    path::Path, thread::sleep, time::Duration, ptr::null, mem::MaybeUninit,
};
use uinput::event::relative::Position;
use x11::{xlib::*, xtest::*};
use once_cell::sync::Lazy;

mod inputs;

type ButtonStatesMap = HashMap<MouseButton, bool>;

static BUTTON_STATES: Lazy<Mutex<ButtonStatesMap>> = Lazy::new(|| Mutex::new(ButtonStatesMap::new()));
static SEND_DISPLAY: Lazy<AtomicPtr<Display>> = Lazy::new(|| {
    unsafe { XInitThreads() };
    AtomicPtr::new(unsafe { XOpenDisplay(null()) })
});
static KEYBD_DEVICE: Lazy<Mutex<uinput::Device>> = Lazy::new(|| {
    Mutex::new(
        uinput::default()
            .unwrap()
            .name("test")
            .unwrap()
            .event(uinput::event::Keyboard::All)
            .unwrap()
            .event(Position::X)
            .unwrap()
            .event(Position::Y)
            .unwrap()
            .create()
            .unwrap(),
    )
});

impl KeybdKey {
    pub fn is_pressed(self) -> bool {
        let code = get_key_code(u64::from(self) as _);
        let mut array: [c_char; 32] = [0; 32];
        SEND_DISPLAY.with(|display| unsafe {
            XQueryKeymap(display, &mut array as *mut [c_char; 32] as *mut c_char);
        });
        array[(code >> 3) as usize] & (1 << (code & 7)) != 0
    }

    pub fn press(self) {
        KEYBD_DEVICE
            .lock()
            .unwrap()
            .write(0x01, key_to_scan_code(self), 1)
            .unwrap();
    }

    pub fn release(self) {
        KEYBD_DEVICE
            .lock()
            .unwrap()
            .write(0x01, key_to_scan_code(self), 0)
            .unwrap();
    }

    pub fn is_toggled(self) -> bool {
        if let Some(key) = match self {
            KeybdKey::ScrollLockKey => Some(4),
            KeybdKey::NumLockKey => Some(2),
            KeybdKey::CapsLockKey => Some(1),
            _ => None,
        } {
            let mut state: XKeyboardState = unsafe { MaybeUninit::zeroed().assume_init() };
            SEND_DISPLAY.with(|display| unsafe {
                XGetKeyboardControl(display, &mut state);
            });
            state.led_mask & key != 0
        } else {
            false
        }
    }
}

impl MouseButton {
    pub fn is_pressed(self) -> bool {
        *BUTTON_STATES.lock().unwrap().entry(self).or_insert(false)
    }

    pub fn press(self) {
        //KEYBD_DEVICE.lock().unwrap().write(0x01, key_to_scan_code(self), 1).unwrap();
        send_mouse_input(u32::from(self), 1);
    }

    pub fn release(self) {
        send_mouse_input(u32::from(self), 0);
    }
}

impl MouseCursor {
    pub fn move_rel(x: i32, y: i32) {
        KEYBD_DEVICE
            .lock()
            .unwrap()
            .position(&Position::X, x)
            .unwrap();
        KEYBD_DEVICE
            .lock()
            .unwrap()
            .position(&Position::Y, y)
            .unwrap();
        KEYBD_DEVICE.lock().unwrap().synchronize().unwrap();
        //SEND_DISPLAY.with(|display| unsafe {
        //    XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
        //});
    }

    pub fn move_abs(x: i32, y: i32) {
        //KEYBD_DEVICE.lock().unwrap().position(&Position::X, x).unwrap();
        //KEYBD_DEVICE.lock().unwrap().position(&Position::Y, y).unwrap();
        SEND_DISPLAY.with(|display| unsafe {
            XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
        });
    }
}

impl MouseWheel {
    pub fn scroll_ver(y: i32) {
        if y < 0 {
          MouseButton::OtherButton(4).press();
          MouseButton::OtherButton(4).release();
        } else {
          MouseButton::OtherButton(5).press();
          MouseButton::OtherButton(5).release();
        }
    }
    pub fn scroll_hor(x: i32) {
        if x < 0 {
          MouseButton::OtherButton(6).press();
          MouseButton::OtherButton(6).release();
        } else {
          MouseButton::OtherButton(7).press();
          MouseButton::OtherButton(7).release();
        }
    }
}

struct LibinputInterfaceRaw;

impl LibinputInterfaceRaw {
    fn seat(&self) -> String {
        String::from("seat0")
    }
}

impl LibinputInterface for LibinputInterfaceRaw {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> std::result::Result<RawFd, i32> {
        if let Ok(fd) = open(path, OFlag::from_bits_truncate(flags), Mode::empty()) {
            Ok(fd)
        } else {
            Err(1)
        }
    }

    fn close_restricted(&mut self, fd: RawFd) {
        let _ = close(fd);
    }
}

pub fn handle_input_events() {
    let mut libinput_context = Libinput::new_with_udev(LibinputInterfaceRaw);
    libinput_context
        .udev_assign_seat(&LibinputInterfaceRaw.seat())
        .unwrap();
    while !MOUSE_BINDS.lock().unwrap().is_empty() || !KEYBD_BINDS.lock().unwrap().is_empty() {
        libinput_context.dispatch().unwrap();
        while let Some(event) = libinput_context.next() {
            handle_input_event(event);
        }
        sleep(Duration::from_millis(10));
    }
}

fn handle_input_event(event: Event) {
    match event {
        Keyboard(keyboard_event) => {
            let KeyboardEvent::Key(keyboard_key_event) = keyboard_event;
            let key = keyboard_key_event.key();
            if let Some(keybd_key) = scan_code_to_key(key) {
                if keyboard_key_event.key_state() == KeyState::Pressed {
                    if let Some(Bind::NormalBind(cb)) = KEYBD_BINDS.lock().unwrap().get(&keybd_key) {
                        let cb = Arc::clone(cb);
                        spawn(move || cb());
                    };
                }
            }
        }
        Pointer(pointer_event) => {
            if let Button(button_event) = pointer_event {
                let button = button_event.button();
                if let Some(mouse_button) = match button {
                    272 => Some(MouseButton::LeftButton),
                    273 => Some(MouseButton::RightButton),
                    274 => Some(MouseButton::MiddleButton),
                    275 => Some(MouseButton::X1Button),
                    276 => Some(MouseButton::X2Button),
                    _ => None,
                } {
                    if button_event.button_state() == ButtonState::Pressed {
                        BUTTON_STATES.lock().unwrap().insert(mouse_button, true);
                        if let Some(Bind::NormalBind(cb)) = MOUSE_BINDS.lock().unwrap().get(&mouse_button) {
                            let cb = Arc::clone(cb);
                            spawn(move || cb());
                        };
                    } else {
                        BUTTON_STATES.lock().unwrap().insert(mouse_button, false);
                    }
                }
            }
        }
        _ => {}
    }
}

fn get_key_code(code: u64) -> u8 {
    SEND_DISPLAY.with(|display| unsafe { XKeysymToKeycode(display, code) })
}

fn send_mouse_input(button: u32, is_press: i32) {
    SEND_DISPLAY.with(|display| unsafe {
        XTestFakeButtonEvent(display, button, is_press, 0);
    });
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
