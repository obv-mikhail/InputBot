#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::collections::hash_map::HashMap;
use std::ops::{FnOnce, FnMut};
use std::marker::Send;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Hotkey {
    KeybdHotkey(KeybdHotkeyType, VKCode), 
    MouseHotkey(MouseHotkeyType)
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum KeybdHotkeyType {
    Press,
    Release
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MouseHotkeyType {
    PressLeft, 
    ReleaseLeft, 
    PressRight, 
    ReleaseRight, 
    PressMiddle, 
    ReleaseMiddle, 
    Move,
    Scroll
}

lazy_static! {
    static ref HOTKEYS: Arc<Mutex<HashMap<Hotkey, Box<FnMut() + Send + 'static>>>> = Arc::new(Mutex::new(HashMap::<Hotkey, Box<FnMut() + Send + 'static>>::new()));
}

pub fn register<F>(hotkey: Hotkey, callback: F) where for<'r> F: FnOnce() + 'static + Send + FnMut() {
    HOTKEYS.lock().unwrap().insert(hotkey, Box::new(callback));
}

pub fn unregister(hotkey: Hotkey) {
    HOTKEYS.lock().unwrap().remove(&hotkey);
}

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;