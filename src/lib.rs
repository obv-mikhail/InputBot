#![recursion_limit="256"]

#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::collections::hash_map::HashMap;
use std::thread::spawn;

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

static mut CAPTURE_EVENTS: bool = false;

lazy_static! {
    static ref HOTKEYS: Arc<Mutex<HashMap<Event, Arc<Fn() + Send + Sync + 'static>>>> = Arc::new(Mutex::new(HashMap::<Event, Arc<Fn() + Send + Sync + 'static>>::new()));
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;