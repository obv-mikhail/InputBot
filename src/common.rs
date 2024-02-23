use crate::public::*;
use once_cell::sync::Lazy;
pub use std::{
    collections::hash_map::HashMap,
    sync::atomic::{AtomicPtr, Ordering},
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread::spawn,
};

pub enum Bind {
    Normal(Handler),
    #[cfg(target_os = "windows")]
    Release(Handler),
    Block(BlockHandler),
    Blockable(BlockableHandler),
}

pub type Handler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockableHandler = Arc<dyn Fn() -> BlockInput + Send + Sync + 'static>;
pub type KeybdBindMap = HashMap<KeybdKey, Bind>;
pub type MouseBindMap = HashMap<MouseButton, Bind>;

pub static HANDLE_EVENTS: AtomicBool = AtomicBool::new(false);
pub static KEYBD_BINDS: Lazy<Mutex<KeybdBindMap>> = Lazy::new(|| Mutex::new(KeybdBindMap::new()));
pub static KEYBD_RELEASE_BINDS: Lazy<Mutex<KeybdBindMap>> =
    Lazy::new(|| Mutex::new(KeybdBindMap::new()));
pub static MOUSE_BINDS: Lazy<Mutex<MouseBindMap>> = Lazy::new(|| Mutex::new(MouseBindMap::new()));
pub static MOUSE_RELEASE_BINDS: Lazy<Mutex<MouseBindMap>> =
    Lazy::new(|| Mutex::new(MouseBindMap::new()));

pub fn should_continue(auto_stop: bool) -> bool {
    HANDLE_EVENTS.load(Ordering::Relaxed)
        && (!auto_stop
            || !MOUSE_BINDS.lock().unwrap().is_empty()
            || !KEYBD_BINDS.lock().unwrap().is_empty()
            || !KEYBD_RELEASE_BINDS.lock().unwrap().is_empty()
            || !MOUSE_RELEASE_BINDS.lock().unwrap().is_empty())
}
