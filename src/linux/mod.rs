extern crate input;
extern crate libc;
extern crate nix;
extern crate udev;
extern crate uinput;
extern crate x11;

use linux::uinput::event::keyboard;
use linux::uinput::event::relative::Position;

use linux::nix::fcntl::{open, OFlag};
use linux::nix::sys::stat::Mode;
use linux::nix::unistd::close;

use linux::input::event::keyboard::KeyState;
use linux::input::event::keyboard::{KeyboardEvent, KeyboardEventTrait};
use linux::input::event::pointer::ButtonState;
use linux::input::event::pointer::PointerEvent::*;
use linux::input::event::Event;
use linux::input::event::Event::*;
use linux::input::Libinput;
use linux::input::LibinputInterface;

use std::os::unix::io::RawFd;
use std::path::Path;

use std::thread::sleep;
use std::time::Duration;

use self::x11::xlib::*;
use self::x11::xtest::*;
use std::mem::uninitialized;
use std::ptr::null;
use *;

use linux::inputs::scan_code_to_key;

mod inputs;
pub use self::inputs::*;

type ButtonStatesMap = HashMap<MouseButton, bool>;

lazy_static! {
    static ref BUTTON_STATES: Mutex<ButtonStatesMap> = Mutex::new(ButtonStatesMap::new());
    static ref SEND_DISPLAY: AtomicPtr<Display> = {
        unsafe { XInitThreads() };
        AtomicPtr::new(unsafe { XOpenDisplay(null()) })
    };
    static ref KEYBD_DEVICE: Mutex<uinput::Device> = {
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
    };
}

impl KeybdKey {
    pub fn is_pressed(self) -> bool {
        let code = get_key_code(u64::from(self) as _);
        let mut array: [i8; 32] = [0; 32];
        SEND_DISPLAY.with(|display| unsafe {
            XQueryKeymap(display, &mut array as *mut [i8; 32] as *mut i8);
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
            KeybdKey::NumLockKey => Some(2),
            KeybdKey::CapsLockKey => Some(1),
            _ => None,
        } {
            let mut state: XKeyboardState = unsafe { uninitialized() };
            SEND_DISPLAY.with(|display| unsafe {
                XGetKeyboardControl(display, &mut state);
            });
            (state.led_mask & key != 0)
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
    pub fn move_rel(self, x: i32, y: i32) {
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

    pub fn move_abs(self, x: i32, y: i32) {
        //KEYBD_DEVICE.lock().unwrap().position(&Position::X, x).unwrap();
        //KEYBD_DEVICE.lock().unwrap().position(&Position::Y, y).unwrap();
        SEND_DISPLAY.with(|display| unsafe {
            XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
        });
    }
}

impl MouseWheel {
    pub fn scroll_ver(self, _: i32) {}

    pub fn scroll_hor(self, _: i32) {}
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
    let udev_context = udev::Context::new().unwrap();
    let mut libinput_context = Libinput::new_from_udev(LibinputInterfaceRaw, &udev_context);
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
                    if let Some(cb) = KEYBD_BINDS.lock().unwrap().get(&keybd_key) {
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
                        if let Some(cb) = MOUSE_BINDS.lock().unwrap().get(&mouse_button) {
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
