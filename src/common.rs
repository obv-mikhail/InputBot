pub use std::sync::{Arc, Mutex};
pub use std::collections::hash_map::HashMap;
pub use std::sync::atomic::{AtomicPtr, Ordering};
pub use std::thread::spawn;
use ::*;

pub type BindHandler = Arc<Fn() + Send + Sync + 'static>;
pub type KeybdBindMap = HashMap<KeybdKey, BindHandler>;
pub type MouseBindMap = HashMap<MouseButton, BindHandler>;

lazy_static! {
    pub static ref KEYBD_BINDS: Mutex<KeybdBindMap> = Mutex::new(KeybdBindMap::new());
    pub static ref MOUSE_BINDS: Mutex<MouseBindMap> = Mutex::new(MouseBindMap::new());
}
