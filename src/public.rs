use crate::common::*;
use std::{thread::sleep, time::Duration};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum KeybdKey {
    BackspaceKey,
    TabKey,
    EnterKey,
    EscapeKey,
    SpaceKey,
    HomeKey,
    LeftKey,
    UpKey,
    RightKey,
    DownKey,
    InsertKey,
    DeleteKey,
    Numrow0Key,
    Numrow1Key,
    Numrow2Key,
    Numrow3Key,
    Numrow4Key,
    Numrow5Key,
    Numrow6Key,
    Numrow7Key,
    Numrow8Key,
    Numrow9Key,
    AKey,
    BKey,
    CKey,
    DKey,
    EKey,
    FKey,
    GKey,
    HKey,
    IKey,
    JKey,
    KKey,
    LKey,
    MKey,
    NKey,
    OKey,
    PKey,
    QKey,
    RKey,
    SKey,
    TKey,
    UKey,
    VKey,
    WKey,
    XKey,
    YKey,
    ZKey,
    Numpad0Key,
    Numpad1Key,
    Numpad2Key,
    Numpad3Key,
    Numpad4Key,
    Numpad5Key,
    Numpad6Key,
    Numpad7Key,
    Numpad8Key,
    Numpad9Key,
    F1Key,
    F2Key,
    F3Key,
    F4Key,
    F5Key,
    F6Key,
    F7Key,
    F8Key,
    F9Key,
    F10Key,
    F11Key,
    F12Key,
    F13Key,
    F14Key,
    F15Key,
    F16Key,
    F17Key,
    F18Key,
    F19Key,
    F20Key,
    F21Key,
    F22Key,
    F23Key,
    F24Key,
    NumLockKey,
    ScrollLockKey,
    CapsLockKey,
    LShiftKey,
    RShiftKey,
    LControlKey,
    RControlKey,
    OtherKey(u64),
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum MouseButton {
    LeftButton,
    MiddleButton,
    RightButton,
    X1Button,
    X2Button,
    OtherButton(u32),
}

pub struct MouseCursor;

pub struct MouseWheel;

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

fn get_keybd_key(c: char) -> Option<KeybdKey> {
    match c {
        ' ' => Some(KeybdKey::SpaceKey),
        'A' | 'a' => Some(KeybdKey::AKey),
        'B' | 'b' => Some(KeybdKey::BKey),
        'C' | 'c' => Some(KeybdKey::CKey),
        'D' | 'd' => Some(KeybdKey::DKey),
        'E' | 'e' => Some(KeybdKey::EKey),
        'F' | 'f' => Some(KeybdKey::FKey),
        'G' | 'g' => Some(KeybdKey::GKey),
        'H' | 'h' => Some(KeybdKey::HKey),
        'I' | 'i' => Some(KeybdKey::IKey),
        'J' | 'j' => Some(KeybdKey::JKey),
        'K' | 'k' => Some(KeybdKey::KKey),
        'L' | 'l' => Some(KeybdKey::LKey),
        'M' | 'm' => Some(KeybdKey::MKey),
        'N' | 'n' => Some(KeybdKey::NKey),
        'O' | 'o' => Some(KeybdKey::OKey),
        'P' | 'p' => Some(KeybdKey::PKey),
        'Q' | 'q' => Some(KeybdKey::QKey),
        'R' | 'r' => Some(KeybdKey::RKey),
        'S' | 's' => Some(KeybdKey::SKey),
        'T' | 't' => Some(KeybdKey::TKey),
        'U' | 'u' => Some(KeybdKey::UKey),
        'V' | 'v' => Some(KeybdKey::VKey),
        'W' | 'w' => Some(KeybdKey::WKey),
        'X' | 'x' => Some(KeybdKey::XKey),
        'Y' | 'y' => Some(KeybdKey::YKey),
        'Z' | 'z' => Some(KeybdKey::ZKey),
        _ => None,
    }
}

pub struct KeySequence(pub &'static str);

impl KeySequence {
    pub fn send(&self) {
        for c in self.0.chars() {
            let mut uppercase = false;
            if let Some(keybd_key) = {
                if c.is_uppercase() {
                    uppercase = true;
                }
                get_keybd_key(c)
            } {
                if uppercase {
                    KeybdKey::LShiftKey.press();
                }
                keybd_key.press();
                sleep(Duration::from_millis(20));
                keybd_key.release();
                if uppercase {
                    KeybdKey::LShiftKey.release();
                }
            };
        }
    }
}
