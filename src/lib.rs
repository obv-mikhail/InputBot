#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::collections::hash_map::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum InputEvent {
    PressKey(u64),
    ReleaseKey(u64),
    PressLButton,
    ReleaseLButton,
    PressRButton,
    ReleaseRButton,
    PressMButton,
    ReleaseMButton,
    PressXButton1,
    ReleaseXButton1,
    PressXButton2,
    ReleaseXButton2
}

lazy_static! {
    static ref BINDS: Arc<Mutex<HashMap<InputEvent, Arc<Fn() + Send + Sync + 'static>>>> = Arc::new(Mutex::new(HashMap::<InputEvent, Arc<Fn() + Send + Sync + 'static>>::new()));
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
