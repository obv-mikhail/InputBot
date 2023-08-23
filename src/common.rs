use crate::public::*;
use once_cell::sync::Lazy;
pub use std::{
    collections::hash_map::HashMap,
    sync::atomic::{AtomicPtr, Ordering},
    sync::{Arc, Mutex},
    thread::spawn,
};

pub enum Bind {
    Normal(Handler),
    Block(BlockHandler),
    Blockable(BlockableHandler),
}

pub type Handler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockableHandler = Arc<dyn Fn() -> BlockInput + Send + Sync + 'static>;
pub type KeybdBindMap = HashMap<KeybdKey, Bind>;
pub type MouseBindMap = HashMap<MouseButton, Bind>;

pub static KEYBD_BINDS: Lazy<Mutex<KeybdBindMap>> = Lazy::new(|| Mutex::new(KeybdBindMap::new()));
pub static MOUSE_BINDS: Lazy<Mutex<MouseBindMap>> = Lazy::new(|| Mutex::new(MouseBindMap::new()));
