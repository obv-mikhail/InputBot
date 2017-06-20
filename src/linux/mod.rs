extern crate x11_dl;

use self::x11_dl::xlib::*;
use Hotkey;
use std::mem::{transmute_copy, transmute, size_of, uninitialized};
use std::sync::{Arc, Mutex};
use std::collections::hash_map::HashMap;
use std::ops::{FnOnce, FnMut};
use std::marker::Send;

lazy_static! {
    static ref HOTKEYS: Arc<Mutex<HashMap<Hotkey, Box<FnMut() + Send + 'static>>>> = Arc::new(Mutex::new(HashMap::<Hotkey, Box<FnMut() + Send + 'static>>::new()));
}

pub fn register<F>(hotkey: Hotkey, callback: F) where for<'r> F: FnOnce() + 'static + Send + FnMut() {

}

pub fn unregister(hotkey: Hotkey) {
    HOTKEYS.lock().unwrap().remove(&hotkey);
}

pub fn capture_input() {

}