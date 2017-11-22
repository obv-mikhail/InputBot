#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum MouseButton {
    LeftButton,
    MiddleButton,
    RightButton,
    X1Button,
    X2Button,
    OtherButton(u32),
}