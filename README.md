# InputBot [![docs link](https://docs.rs/inputbot/badge.svg)](https://docs.rs/inputbot) [![crates.io version](https://img.shields.io/crates/v/inputbot.svg)](https://crates.io/crates/inputbot) 
A Rust library for creating global hotkeys, and emulating inputs.
Unlike AutoHotkey, InputBot handles hotkeys concurrently and supports both Windows and Linux.

# How-To
The code below demonstrates how to create some simple hotkeys.

```Rust
use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::{thread::sleep, time::Duration};

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

    // Send a key sequence.
    RKey.bind(|| KeySequence("Sample text").send());

    // Move mouse.
    QKey.bind(|| MouseCursor.move_rel(10, 10));

    // Call this to start listening for bound inputs.
    handle_input_events();
}
```
