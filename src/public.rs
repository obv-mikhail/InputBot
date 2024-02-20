use crate::common::*;
use std::{thread::sleep, time::Duration};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[cfg(feature = "serde")]
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{
    de::{Deserializer, Error},
    Deserialize, Serialize,
};
#[cfg(feature = "serde")]
use thiserror::Error;

pub enum BlockInput {
    Block,
    DontBlock,
}

#[cfg(feature = "serde")]
fn other_key_regex() -> &'static Regex {
    use std::sync::OnceLock;

    static OTHER_KEY: OnceLock<Regex> = OnceLock::new();
    OTHER_KEY.get_or_init(|| Regex::new(r#"OtherKey\((\d+)\)"#).unwrap())
}

#[cfg(feature = "serde")]
fn other_mouse_regex() -> &'static Regex {
    use std::sync::OnceLock;

    static OTHER_KEY: OnceLock<Regex> = OnceLock::new();
    OTHER_KEY.get_or_init(|| Regex::new(r#"MouseButton\((\d+)\)"#).unwrap())
}

#[cfg(feature = "serde")]
fn keyboard_canonical_names() -> &'static HashMap<String, KeybdKey> {
    use std::sync::OnceLock;

    static KEYBOARD_CANONICAL_NAMES: OnceLock<HashMap<String, KeybdKey>> = OnceLock::new();

    KEYBOARD_CANONICAL_NAMES.get_or_init(|| {
        KeybdKey::iter()
            .filter_map(|k| match k {
                KeybdKey::OtherKey(_) => None,
                _ => Some((k.canonical_name(), k)),
            })
            .collect()
    })
}

#[cfg(feature = "serde")]
fn keyboard_canonical_names_lower() -> &'static HashMap<String, KeybdKey> {
    use std::sync::OnceLock;
    static KEYBOARD_CANONICAL_NAMES_LOWER: OnceLock<HashMap<String, KeybdKey>> = OnceLock::new();

    KEYBOARD_CANONICAL_NAMES_LOWER.get_or_init(|| {
        keyboard_canonical_names()
            .iter()
            .map(|(k, v)| (k.to_lowercase(), v.clone()))
            .collect()
    })
}
#[cfg(feature = "serde")]
fn mouse_canonical_names() -> &'static HashMap<String, MouseButton> {
    use std::sync::OnceLock;

    static MOUSE_CANONICAL_NAMES: OnceLock<HashMap<String, MouseButton>> = OnceLock::new();
    MOUSE_CANONICAL_NAMES.get_or_init(|| {
        MouseButton::iter()
            .filter_map(|k| match k {
                MouseButton::OtherButton(_) => None,
                _ => Some((k.canonical_name(), k)),
            })
            .collect()
    })
}

#[cfg(feature = "serde")]
fn mouse_canonical_names_lower() -> &'static HashMap<String, MouseButton> {
    use std::sync::OnceLock;

    static MOUSE_CANONICAL_NAMES_LOWER: OnceLock<HashMap<String, MouseButton>> = OnceLock::new();
    MOUSE_CANONICAL_NAMES_LOWER.get_or_init(|| {
        mouse_canonical_names()
            .iter()
            .map(|(k, v)| (k.to_lowercase(), v.clone()))
            .collect()
    })
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize))]

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
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum MouseButton {
    LeftButton,
    MiddleButton,
    RightButton,
    X1Button,
    X2Button,
    MousewheelUp,
    MousewheelDown,
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

    #[cfg(target_os = "windows")]
    pub fn bind_release<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBD_RELEASE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Release(Arc::new(callback)));
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

    #[cfg(target_os = "windows")]
    pub fn bind_all_release<F: Fn(KeybdKey) + Send + Sync + Clone + 'static>(callback: F) {
        for key in KeybdKey::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(key);
            };

            KEYBD_RELEASE_BINDS
                .lock()
                .unwrap()
                .insert(key, Bind::Release(Arc::new(fire)));
        }
    }

    pub fn is_bound(self) -> bool {
        KEYBD_BINDS.lock().unwrap().contains_key(&self)
    }

    pub fn unbind(self) {
        KEYBD_BINDS.lock().unwrap().remove(&self);
    }

    // the canonical_name is guaranteed to roundtrip to and from the serialization format.
    pub fn canonical_name(self) -> String {
        match self {
            KeybdKey::LSuper => "LeftSuper".to_owned(),
            KeybdKey::RSuper => "RightSuper".to_owned(),
            _ => format!("{}", self),
        }
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
                        "LeftWindows"
                    } else if cfg!(target_os = "macos") {
                        "LeftCommand"
                    } else {
                        "LeftSuper"
                    },
                KeybdKey::RSuper =>
                    if cfg!(target_os = "windows") {
                        "RightWindows"
                    } else if cfg!(target_os = "macos") {
                        "RightCommand"
                    } else {
                        "RightSuper"
                    },
                KeybdKey::Numpad0Key => "NumPad0",
                KeybdKey::Numpad1Key => "NumPad1",
                KeybdKey::Numpad2Key => "NumPad2",
                KeybdKey::Numpad3Key => "NumPad3",
                KeybdKey::Numpad4Key => "NumPad4",
                KeybdKey::Numpad5Key => "NumPad5",
                KeybdKey::Numpad6Key => "NumPad6",
                KeybdKey::Numpad7Key => "NumPad7",
                KeybdKey::Numpad8Key => "NumPad8",
                KeybdKey::Numpad9Key => "NumPad9",
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
                KeybdKey::NumLockKey => "NumLock",
                KeybdKey::ScrollLockKey => "ScrollLock",
                KeybdKey::CapsLockKey => "CapsLock",
                KeybdKey::LShiftKey => "LeftShift",
                KeybdKey::RShiftKey => "RightShift",
                KeybdKey::LControlKey => "LeftControl",
                KeybdKey::RControlKey => "RightControl",
                KeybdKey::LAltKey => "LeftAlt",
                KeybdKey::RAltKey => "RightAlt",
                KeybdKey::BrowserBackKey => "Back",
                KeybdKey::BrowserForwardKey => "Forward",
                KeybdKey::BrowserRefreshKey => "Refresh",
                KeybdKey::VolumeMuteKey => "VolumeMute",
                KeybdKey::VolumeDownKey => "VolumeDown",
                KeybdKey::VolumeUpKey => "VolumeUp",
                KeybdKey::MediaNextTrackKey => "MediaNext",
                KeybdKey::MediaPrevTrackKey => "MediaPrevious",
                KeybdKey::MediaStopKey => "MediaStop",
                KeybdKey::MediaPlayPauseKey => "MediaPlay",
                KeybdKey::BackquoteKey => "Backquote",
                KeybdKey::SlashKey => "Slash",
                KeybdKey::BackslashKey => "Backslash",
                KeybdKey::CommaKey => "Comma",
                KeybdKey::PeriodKey => "Period",
                KeybdKey::MinusKey => "Minus",
                KeybdKey::QuoteKey => "QuoteKey",
                KeybdKey::SemicolonKey => "Semicolon",
                KeybdKey::LBracketKey => "LeftBracket",
                KeybdKey::RBracketKey => "RightBracket",
                KeybdKey::EqualKey => "Equal",
                KeybdKey::OtherKey(code) => return write!(f, "OtherKey({code})"),
            }
        )
    }
}

#[cfg(feature = "serde")]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unable to parse the keycode value")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
        backtrace: std::backtrace::Backtrace,
    },
    #[error("Unknown format '{val}'")]
    UnknownFormat {
        val: String,
        backtrace: std::backtrace::Backtrace,
    },
}

#[cfg(feature = "serde")]
impl std::str::FromStr for KeybdKey {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();
        if let Some(k) = keyboard_canonical_names_lower().get(&s_lower) {
            return Ok(*k);
        }
        match s_lower.as_str() {
            "leftwindows" => return Ok(KeybdKey::LSuper),
            "leftcommand" => return Ok(KeybdKey::LSuper),
            "rightwindows" => return Ok(KeybdKey::RSuper),
            "rightcommand" => return Ok(KeybdKey::RSuper),
            _ => {}
        }
        if let Some(caps) = other_key_regex().captures(s) {
            let v = &caps[1]
                .parse::<u64>()
                .map_err(|err| Into::<ParseError>::into(err))?;
            return Ok(KeybdKey::OtherKey(*v));
        }

        Err(ParseError::UnknownFormat {
            val: s.to_string(),
            backtrace: std::backtrace::Backtrace::capture(),
        })
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for KeybdKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        std::str::FromStr::from_str(&s).map_err(Error::custom)
    }
}

impl MouseButton {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Normal(Arc::new(callback)));
    }

    #[cfg(target_os = "windows")]
    pub fn bind_release<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_RELEASE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Release(Arc::new(callback)));
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

    #[cfg(target_os = "windows")]
    pub fn bind_all_release<F: Fn(MouseButton) + Send + Sync + Clone + 'static>(callback: F) {
        for btn in MouseButton::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(btn);
            };

            MOUSE_RELEASE_BINDS
                .lock()
                .unwrap()
                .insert(btn, Bind::Release(Arc::new(fire)));
        }
    }

    pub fn is_bound(self) -> bool {
        MOUSE_BINDS.lock().unwrap().contains_key(&self)
    }

    pub fn unbind(self) {
        MOUSE_BINDS.lock().unwrap().remove(&self);
    }

    pub fn canonical_name(self) -> String {
        format!("{}", &self)
    }
}

impl std::fmt::Display for MouseButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MouseButton::LeftButton => "LeftClick",
                MouseButton::MiddleButton => "MiddleClick",
                MouseButton::RightButton => "RightClick",
                MouseButton::X1Button => "MouseBackward",
                MouseButton::X2Button => "MouseForward",
                MouseButton::OtherButton(code) => return write!(f, "MouseButton({code})"),
                MouseButton::MousewheelDown => "MousewheelDown",
                MouseButton::MousewheelUp => "MousewheelUp",
            }
        )
    }
}

#[cfg(feature = "serde")]
impl std::str::FromStr for MouseButton {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();
        if let Some(k) = mouse_canonical_names_lower().get(&s_lower) {
            return Ok(*k);
        }
        if let Some(caps) = other_mouse_regex().captures(s) {
            let v = &caps[1]
                .parse::<u32>()
                .map_err(|err| Into::<ParseError>::into(err))?;
            return Ok(MouseButton::OtherButton(*v));
        }

        Err(ParseError::UnknownFormat {
            val: s.to_string(),
            backtrace: std::backtrace::Backtrace::capture(),
        })
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for MouseButton {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        std::str::FromStr::from_str(&s).map_err(Error::custom)
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

/// Stops `handle_input_events()` (threadsafe)
pub fn stop_handling_input_events() {
    HANDLE_EVENTS.store(false, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "serde")]
    fn to_string_roundtrips() -> Result<(), Box<dyn std::error::Error>> {
        use crate::{KeybdKey, MouseButton};
        use std::{collections::HashSet, str::FromStr};

        use strum::IntoEnumIterator;

        let serialized_keys: Vec<String> = KeybdKey::iter().map(|k| k.to_string()).collect();
        let deserialized_keys: HashSet<KeybdKey> = serialized_keys
            .iter()
            .map(|k| KeybdKey::from_str(k).unwrap())
            .collect();
        for k in KeybdKey::iter() {
            assert!(deserialized_keys.contains(&k));
        }

        let other_key_string = KeybdKey::OtherKey(42).to_string();
        let other_key = KeybdKey::from_str(&other_key_string)?;
        assert!(other_key == KeybdKey::OtherKey(42));

        let serialized_mouse: Vec<String> = MouseButton::iter().map(|b| b.to_string()).collect();
        let deserialized_mouse: HashSet<MouseButton> = serialized_mouse
            .iter()
            .map(|b| MouseButton::from_str(b).unwrap())
            .collect();
        for b in MouseButton::iter() {
            assert!(deserialized_mouse.contains(&b));
        }
        let other_mouse_string = MouseButton::OtherButton(42).to_string();
        let other_mouse = MouseButton::from_str(&other_mouse_string)?;
        assert!(other_mouse == MouseButton::OtherButton(42));
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn canonical_name_roundtrips() -> Result<(), Box<dyn std::error::Error>> {
        use crate::{KeybdKey, MouseButton};
        use std::{collections::HashSet, str::FromStr};

        use strum::IntoEnumIterator;

        let serialized_keys: Vec<String> = KeybdKey::iter().map(|k| k.canonical_name()).collect();
        let deserialized_keys: HashSet<KeybdKey> = serialized_keys
            .iter()
            .map(|k| KeybdKey::from_str(k).unwrap())
            .collect();
        for k in KeybdKey::iter() {
            assert!(deserialized_keys.contains(&k));
        }

        let other_key_string = KeybdKey::OtherKey(42).canonical_name();
        let other_key = KeybdKey::from_str(&other_key_string)?;
        assert!(other_key == KeybdKey::OtherKey(42));

        let serialized_mouse: Vec<String> =
            MouseButton::iter().map(|b| b.canonical_name()).collect();
        let deserialized_mouse: HashSet<MouseButton> = serialized_mouse
            .iter()
            .map(|b| MouseButton::from_str(b).unwrap())
            .collect();
        for b in MouseButton::iter() {
            assert!(deserialized_mouse.contains(&b));
        }

        let other_mouse_string = MouseButton::OtherButton(42).canonical_name();
        let other_mouse = MouseButton::from_str(&other_mouse_string)?;
        assert!(other_mouse == MouseButton::OtherButton(42));
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serialization_case_insensitive() {
        use crate::{KeybdKey, MouseButton};
        use std::{collections::HashSet, str::FromStr};

        use strum::IntoEnumIterator;
        let serialized_keys: Vec<String> = KeybdKey::iter().map(|k| k.canonical_name()).collect();
        let serialized_keys_upper: Vec<String> =
            serialized_keys.iter().map(|k| k.to_uppercase()).collect();
        let serialized_keys_lower: Vec<String> =
            serialized_keys.iter().map(|k| k.to_lowercase()).collect();
        for serialization in vec![
            serialized_keys,
            serialized_keys_upper,
            serialized_keys_lower,
        ] {
            let deserialized_keys: HashSet<KeybdKey> = serialization
                .iter()
                .map(|k| KeybdKey::from_str(k).unwrap())
                .collect();
            for k in KeybdKey::iter() {
                assert!(deserialized_keys.contains(&k));
            }
        }

        let serialized_mouse: Vec<String> =
            MouseButton::iter().map(|b| b.canonical_name()).collect();
        let serialized_mouse_upper: Vec<String> =
            serialized_mouse.iter().map(|k| k.to_uppercase()).collect();
        let serialized_mouse_lower: Vec<String> =
            serialized_mouse.iter().map(|k| k.to_uppercase()).collect();
        for serialization in vec![
            serialized_mouse,
            serialized_mouse_upper,
            serialized_mouse_lower,
        ] {
            let deserialized_mouse: HashSet<MouseButton> = serialization
                .iter()
                .map(|b| MouseButton::from_str(b).unwrap())
                .collect();
            for b in MouseButton::iter() {
                assert!(deserialized_mouse.contains(&b));
            }
        }
    }
}
