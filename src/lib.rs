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
    MouseReleaseXButton2,
    MouseScroll,
    MouseMove
}

impl Event {
    pub fn bind<F>(self, callback: F) where F: 'static + Send + Fn() {
        HOTKEYS.lock().unwrap().insert(self, Box::new(callback));
    }

    pub fn unbind(self) {
        HOTKEYS.lock().unwrap().remove(&self);
    }
}

lazy_static! {
    static ref HOTKEYS: Arc<Mutex<HashMap<Event, Box<Fn() + Send + 'static>>>> = Arc::new(Mutex::new(HashMap::<Event, Box<Fn() + Send + 'static>>::new()));
    static ref CAPTURE_HOTKEYS: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub fn stop_capture() {
    *CAPTURE_HOTKEYS.lock().unwrap() = false;
}

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;