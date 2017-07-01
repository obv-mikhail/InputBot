#![recursion_limit="256"]

#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::collections::hash_map::HashMap;

pub type Code = u8;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Event {
    KeybdPress(Code),
    KeybdRelease(Code),
    MousePressLeft,
    MouseReleaseLeft,
    MousePressRight,
    MouseReleaseRight,
    MousePressMiddle,
    MouseReleaseMiddle,
    MousePressXButton1,
    MouseReleaseXButton1,
    MousePressXButton2,
    MouseReleaseXButton2
}

impl Event {
    pub fn bind<F>(self, callback: F) where F: Fn() + Send + Sync + 'static {
        std::thread::spawn(move || {
            HOTKEYS.lock().unwrap().insert(self, Arc::new(callback));
            if unsafe {!CAPTURE_EVENTS} {
                unsafe {CAPTURE_EVENTS = true};
                unsafe {start_capture()};
                while unsafe {CAPTURE_EVENTS} {
                    if let Some(event) = unsafe {get_event()} {
                        if let Some(cb) = HOTKEYS.lock().unwrap().get_mut(&event) {
                            let cb = cb.clone();
                            std::thread::spawn(move || cb());
                        };
                    }
                }
                unsafe {stop_capture()};
            }
        });
    }

    pub fn unbind(self) {
        HOTKEYS.lock().unwrap().remove(&self);
        if HOTKEYS.lock().unwrap().len() == 0 {
            unsafe {CAPTURE_EVENTS = false};
        }
    }
}

static mut CAPTURE_EVENTS: bool = false;

lazy_static! {
    static ref HOTKEYS: Arc<Mutex<HashMap<Event, Arc<Fn() + Send + Sync + 'static>>>> = Arc::new(Mutex::new(HashMap::<Event, Arc<Fn() + Send + Sync + 'static>>::new()));
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;
#[cfg(target_os = "windows")]
use windows::{start_capture, stop_capture, get_event};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "linux")]
use linux::{start_capture, stop_capture, get_event};