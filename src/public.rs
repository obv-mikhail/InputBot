use crate::common::*;
use std::{thread::sleep, time::Duration};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub enum BlockInput {
    Block,
    DontBlock,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
pub enum KeybdKey {
    BackspaceKey,
    TabKey,
    EnterKey,
    EscapeKey,
    SpaceKey,
    PageUpKey,
    PageDownKey,
    EndKey,
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
    LSuper,
    RSuper,
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
    LAltKey,
    RAltKey,

    BrowserBackKey,
    BrowserForwardKey,
    BrowserRefreshKey,

    VolumeMuteKey,
    VolumeDownKey,
    VolumeUpKey,

    MediaNextTrackKey,
    MediaPrevTrackKey,
    MediaStopKey,
    MediaPlayPauseKey,

    BackquoteKey,
    SlashKey,
    BackslashKey,
    CommaKey,
    PeriodKey,
    MinusKey,
    QuoteKey,
    SemicolonKey,
    LBracketKey,
    RBracketKey,
    EqualKey,

    #[strum(disabled)]
    OtherKey(u64),
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
pub enum MouseButton {
    LeftButton,
    MiddleButton,
    RightButton,
    X1Button,
    X2Button,

    #[strum(disabled)]
    OtherButton(u32),
}

pub struct MouseCursor;

pub struct MouseWheel;

impl KeybdKey {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Normal(Arc::new(callback)));
    }

    pub fn block_bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Block(Arc::new(callback)));
    }

    pub fn blockable_bind<F: Fn() -> BlockInput + Send + Sync + 'static>(self, callback: F) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Blockable(Arc::new(callback)));
    }

    pub fn bind_all<F: Fn(KeybdKey) + Send + Sync + Clone + 'static>(callback: F) {
        for key in KeybdKey::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(key);
            };

            KEYBD_BINDS
                .lock()
                .unwrap()
                .insert(key, Bind::Normal(Arc::new(fire)));
        }
    }

    pub fn unbind(self) {
        KEYBD_BINDS.lock().unwrap().remove(&self);
    }
}

impl std::fmt::Display for KeybdKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KeybdKey::BackspaceKey => "Backspace",
                KeybdKey::TabKey => "Tab",
                KeybdKey::EnterKey => "Enter",
                KeybdKey::EscapeKey => "Escape",
                KeybdKey::SpaceKey => "Space",
                KeybdKey::PageUpKey => "PageUp",
                KeybdKey::PageDownKey => "PageDown",
                KeybdKey::EndKey => "End",
                KeybdKey::HomeKey => "Home",
                KeybdKey::LeftKey => "Left",
                KeybdKey::UpKey => "Up",
                KeybdKey::RightKey => "Right",
                KeybdKey::DownKey => "Down",
                KeybdKey::InsertKey => "Insert",
                KeybdKey::DeleteKey => "Delete",
                KeybdKey::Numrow0Key => "0",
                KeybdKey::Numrow1Key => "1",
                KeybdKey::Numrow2Key => "2",
                KeybdKey::Numrow3Key => "3",
                KeybdKey::Numrow4Key => "4",
                KeybdKey::Numrow5Key => "5",
                KeybdKey::Numrow6Key => "6",
                KeybdKey::Numrow7Key => "7",
                KeybdKey::Numrow8Key => "8",
                KeybdKey::Numrow9Key => "9",
                KeybdKey::AKey => "a",
                KeybdKey::BKey => "b",
                KeybdKey::CKey => "c",
                KeybdKey::DKey => "d",
                KeybdKey::EKey => "e",
                KeybdKey::FKey => "f",
                KeybdKey::GKey => "g",
                KeybdKey::HKey => "h",
                KeybdKey::IKey => "i",
                KeybdKey::JKey => "j",
                KeybdKey::KKey => "k",
                KeybdKey::LKey => "l",
                KeybdKey::MKey => "m",
                KeybdKey::NKey => "n",
                KeybdKey::OKey => "o",
                KeybdKey::PKey => "p",
                KeybdKey::QKey => "q",
                KeybdKey::RKey => "r",
                KeybdKey::SKey => "s",
                KeybdKey::TKey => "t",
                KeybdKey::UKey => "u",
                KeybdKey::VKey => "v",
                KeybdKey::WKey => "w",
                KeybdKey::XKey => "x",
                KeybdKey::YKey => "y",
                KeybdKey::ZKey => "z",
                KeybdKey::LSuper =>
                    if cfg!(target_os = "windows") {
                        "Left Windows"
                    } else if cfg!(target_os = "macos") {
                        "Left Command"
                    } else {
                        "Left Super"
                    },
                KeybdKey::RSuper =>
                    if cfg!(target_os = "windows") {
                        "Right Windows"
                    } else if cfg!(target_os = "macos") {
                        "Right Command"
                    } else {
                        "Right Super"
                    },
                KeybdKey::Numpad0Key => "Number Pad 0",
                KeybdKey::Numpad1Key => "Number Pad 1",
                KeybdKey::Numpad2Key => "Number Pad 2",
                KeybdKey::Numpad3Key => "Number Pad 3",
                KeybdKey::Numpad4Key => "Number Pad 4",
                KeybdKey::Numpad5Key => "Number Pad 5",
                KeybdKey::Numpad6Key => "Number Pad 6",
                KeybdKey::Numpad7Key => "Number Pad 7",
                KeybdKey::Numpad8Key => "Number Pad 8",
                KeybdKey::Numpad9Key => "Number Pad 9",
                KeybdKey::F1Key => "F1",
                KeybdKey::F2Key => "F2",
                KeybdKey::F3Key => "F3",
                KeybdKey::F4Key => "F4",
                KeybdKey::F5Key => "F5",
                KeybdKey::F6Key => "F6",
                KeybdKey::F7Key => "F7",
                KeybdKey::F8Key => "F8",
                KeybdKey::F9Key => "F9",
                KeybdKey::F10Key => "F10",
                KeybdKey::F11Key => "F11",
                KeybdKey::F12Key => "F12",
                KeybdKey::F13Key => "F13",
                KeybdKey::F14Key => "F14",
                KeybdKey::F15Key => "F15",
                KeybdKey::F16Key => "F16",
                KeybdKey::F17Key => "F17",
                KeybdKey::F18Key => "F18",
                KeybdKey::F19Key => "F19",
                KeybdKey::F20Key => "F20",
                KeybdKey::F21Key => "F21",
                KeybdKey::F22Key => "F22",
                KeybdKey::F23Key => "F23",
                KeybdKey::F24Key => "F24",
                KeybdKey::NumLockKey => "Number Lock",
                KeybdKey::ScrollLockKey => "Scroll Lock",
                KeybdKey::CapsLockKey => "Caps Lock",
                KeybdKey::LShiftKey => "Left Shift",
                KeybdKey::RShiftKey => "Right Shift",
                KeybdKey::LControlKey => "Left Control",
                KeybdKey::RControlKey => "Right Control",
                KeybdKey::LAltKey => "Left Alt",
                KeybdKey::RAltKey => "Right Alt",
                KeybdKey::BrowserBackKey => "Back",
                KeybdKey::BrowserForwardKey => "Forward",
                KeybdKey::BrowserRefreshKey => "Refresh",
                KeybdKey::VolumeMuteKey => "Volume Mute",
                KeybdKey::VolumeDownKey => "Volume Down",
                KeybdKey::VolumeUpKey => "Volume Up",
                KeybdKey::MediaNextTrackKey => "Media Next",
                KeybdKey::MediaPrevTrackKey => "Media Previous",
                KeybdKey::MediaStopKey => "Media Stop",
                KeybdKey::MediaPlayPauseKey => "Media Play",
                KeybdKey::BackquoteKey => "Backquote",
                KeybdKey::SlashKey => "Slash",
                KeybdKey::BackslashKey => "Backslash",
                KeybdKey::CommaKey => "Comma",
                KeybdKey::PeriodKey => "Period",
                KeybdKey::MinusKey => "Minus",
                KeybdKey::QuoteKey => "QuoteKey",
                KeybdKey::SemicolonKey => "Semicolon",
                KeybdKey::LBracketKey => "Left Bracket",
                KeybdKey::RBracketKey => "Right Bracket",
                KeybdKey::EqualKey => "Equal",
                KeybdKey::OtherKey(code) => return write!(f, "{code} Key"),
            }
        )
    }
}

impl MouseButton {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Normal(Arc::new(callback)));
    }

    pub fn block_bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Block(Arc::new(callback)));
    }

    pub fn blockable_bind<F: Fn() -> BlockInput + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Blockable(Arc::new(callback)));
    }

    pub fn bind_all<F: Fn(MouseButton) + Send + Sync + Clone + 'static>(callback: F) {
        for btn in MouseButton::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(btn);
            };

            MOUSE_BINDS
                .lock()
                .unwrap()
                .insert(btn, Bind::Normal(Arc::new(fire)));
        }
    }

    pub fn unbind(self) {
        MOUSE_BINDS.lock().unwrap().remove(&self);
    }
}

impl std::fmt::Display for MouseButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MouseButton::LeftButton => "Left Click",
                MouseButton::MiddleButton => "Middle Click",
                MouseButton::RightButton => "Right Click",
                MouseButton::X1Button => "Mouse Backward",
                MouseButton::X2Button => "Mouse Forward",
                MouseButton::OtherButton(code) => return write!(f, "{code} Click"),
            }
        )
    }
}

pub fn from_keybd_key(k: KeybdKey) -> Option<char> {
    match k {
        KeybdKey::AKey => Some('a'),
        KeybdKey::BKey => Some('b'),
        KeybdKey::CKey => Some('c'),
        KeybdKey::DKey => Some('d'),
        KeybdKey::EKey => Some('e'),
        KeybdKey::FKey => Some('f'),
        KeybdKey::GKey => Some('g'),
        KeybdKey::HKey => Some('h'),
        KeybdKey::IKey => Some('i'),
        KeybdKey::JKey => Some('j'),
        KeybdKey::KKey => Some('k'),
        KeybdKey::LKey => Some('l'),
        KeybdKey::MKey => Some('m'),
        KeybdKey::NKey => Some('n'),
        KeybdKey::OKey => Some('o'),
        KeybdKey::PKey => Some('p'),
        KeybdKey::QKey => Some('q'),
        KeybdKey::RKey => Some('r'),
        KeybdKey::SKey => Some('s'),
        KeybdKey::TKey => Some('t'),
        KeybdKey::UKey => Some('u'),
        KeybdKey::VKey => Some('v'),
        KeybdKey::WKey => Some('w'),
        KeybdKey::XKey => Some('x'),
        KeybdKey::YKey => Some('y'),
        KeybdKey::ZKey => Some('z'),
        KeybdKey::Numpad0Key => Some('0'),
        KeybdKey::Numpad1Key => Some('1'),
        KeybdKey::Numpad2Key => Some('2'),
        KeybdKey::Numpad3Key => Some('3'),
        KeybdKey::Numpad4Key => Some('4'),
        KeybdKey::Numpad5Key => Some('5'),
        KeybdKey::Numpad6Key => Some('6'),
        KeybdKey::Numpad7Key => Some('7'),
        KeybdKey::Numpad8Key => Some('8'),
        KeybdKey::Numpad9Key => Some('9'),
        KeybdKey::Numrow0Key => Some('0'),
        KeybdKey::Numrow1Key => Some('1'),
        KeybdKey::Numrow2Key => Some('2'),
        KeybdKey::Numrow3Key => Some('3'),
        KeybdKey::Numrow4Key => Some('4'),
        KeybdKey::Numrow5Key => Some('5'),
        KeybdKey::Numrow6Key => Some('6'),
        KeybdKey::Numrow7Key => Some('7'),
        KeybdKey::Numrow8Key => Some('8'),
        KeybdKey::Numrow9Key => Some('9'),
        KeybdKey::BackslashKey => Some('\\'),
        KeybdKey::SlashKey => Some('/'),
        KeybdKey::CommaKey => Some(','),
        KeybdKey::PeriodKey => Some('.'),
        KeybdKey::MinusKey => Some('-'),
        KeybdKey::QuoteKey => Some('"'),
        KeybdKey::SemicolonKey => Some(';'),
        KeybdKey::LBracketKey => Some('['),
        KeybdKey::RBracketKey => Some(']'),
        KeybdKey::EqualKey => Some('='),
        _ => None,
    }
}

pub fn get_keybd_key(c: char) -> Option<KeybdKey> {
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
        '0' | ')' => Some(KeybdKey::Numrow0Key),
        '1' | '!' => Some(KeybdKey::Numrow1Key),
        '2' | '@' => Some(KeybdKey::Numrow2Key),
        '3' | '#' => Some(KeybdKey::Numrow3Key),
        '4' | '$' => Some(KeybdKey::Numrow4Key),
        '5' | '%' => Some(KeybdKey::Numrow5Key),
        '6' | '^' => Some(KeybdKey::Numrow6Key),
        '7' | '&' => Some(KeybdKey::Numrow7Key),
        '8' | '*' => Some(KeybdKey::Numrow8Key),
        '9' | '(' => Some(KeybdKey::Numrow9Key),
        '`' | '~' => Some(KeybdKey::BackquoteKey),
        '/' | '?' => Some(KeybdKey::SlashKey),
        ',' | '<' => Some(KeybdKey::CommaKey),
        '.' | '>' => Some(KeybdKey::PeriodKey),
        '-' | '_' => Some(KeybdKey::MinusKey),
        ';' | ':' => Some(KeybdKey::SemicolonKey),
        '[' | '{' => Some(KeybdKey::LBracketKey),
        ']' | '}' => Some(KeybdKey::RBracketKey),
        '=' | '+' => Some(KeybdKey::EqualKey),
        '\\' | '|' => Some(KeybdKey::BackslashKey),
        '\'' | '"' => Some(KeybdKey::QuoteKey),
        _ => None,
    }
}

pub struct KeySequence<'a>(pub &'a str);

impl KeySequence<'_> {
    pub fn send(&self) {
        for c in self.0.chars() {
            let mut uppercase = false;

            if let Some(keybd_key) = {
                if c.is_uppercase()
                    || [
                        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|',
                        ':', '"', '<', '>', '?', '~',
                    ]
                    .contains(&c)
                {
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
