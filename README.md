# InputBot [![docs link](https://docs.rs/inputbot/badge.svg)](https://docs.rs/inputbot) [![crates.io version](https://img.shields.io/crates/v/inputbot.svg)](https://crates.io/crates/inputbot) 
A very small AutoHotkey inspired library for creating global hotkeys, as well as emulating mouse and keyboard input. Works on Windows and X11 Linux. Unlike AutoHotkey, can handle multiple hotkeys concurrently in one process.

# How-To
The code below demonstrates how to create some simple hotkeys.

```Rust
extern crate inputbot;

use inputbot::*;
use KeybdKey::*;
use MouseButton::*;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    // Autorun for videogames.
    NumLockKey.bind(|| {
        while NumLockKey.is_toggled() {
            LShiftKey.press();
            WKey.press();
            sleep(Duration::from_millis(50));
            WKey.release();
            LShiftKey.release();
        }
    });

    // Rapidfire for videogames.
    RightButton.bind(|| {
        while RightButton.is_pressed() {
            LeftButton.press();
            sleep(Duration::from_millis(50));
            LeftButton.release();
        }
    });

    // Mouse movement test
    QKey.bind(|| MouseCursor.move_rel(10, 10));

    handle_input_events();
}
```
