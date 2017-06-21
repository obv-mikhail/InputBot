#[macro_use]
extern crate lazy_static;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Hotkey {
    KeybdHotkey(KeybdHotkeyType, VKCode), 
    MouseHotkey(MouseHotkeyType)
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum KeybdHotkeyType {
    Press,
    Release
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
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

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;