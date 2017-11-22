use ::*;

pub type BindHandler = Arc<Fn() + Send + Sync + 'static>;
pub type KeybdBindMap = HashMap<KeybdKey, BindHandler>;
pub type MouseBindMap = HashMap<MouseButton, BindHandler>;

impl KeybdKey {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBD_BINDS.lock().unwrap().insert(self, Arc::new(callback));
    }

    pub fn unbind(self) {
        KEYBD_BINDS.lock().unwrap().remove(&self);
    }
}

impl MouseButton {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS.lock().unwrap().insert(self, Arc::new(callback));
    }

    pub fn unbind(self) {
        MOUSE_BINDS.lock().unwrap().remove(&self);
    }
}

lazy_static! {
    pub static ref KEYBD_BINDS: Mutex<KeybdBindMap> = Mutex::new(KeybdBindMap::new());
    pub static ref MOUSE_BINDS: Mutex<MouseBindMap> = Mutex::new(MouseBindMap::new());
}
