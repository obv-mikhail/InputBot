use ::*;

pub type BindHandler = Arc<Fn() + Send + Sync + 'static>;
pub type InputBindMap = Mutex<HashMap<InputEvent, BindHandler>>;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum InputEvent {
    ButtonPress(MouseButton),
    KeyPress(KeybdKey),
}

lazy_static! {
    pub static ref INPUT_BINDS: InputBindMap = Mutex::new(HashMap::<InputEvent, BindHandler>::new());
}