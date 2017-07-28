# InputBot [![docs link](https://img.shields.io/badge/docs-0.2.0-red.svg)](https://obv-mikhail.github.io/doc/inputbot/) [![crates.io version](https://img.shields.io/crates/v/inputbot.svg)](https://crates.io/crates/inputbot) 
A very small AutoHotkey inspired library for creating global hotkeys, as well as emulating mouse and keyboard input. Works on Windows and X11 Linux.

# How-To
The code below demonstrates how to create some simple hotkeys.

```Rust
extern crate inputbot;

use inputbot::*;
use KeybdKey::*;
use MouseButton::*;
use std::time::Duration;
use std::thread::{sleep, park};

fn main() {
    // Autorun for videogames.
    NumLockKey.bind(|| while num_lock_is_toggled() {
        LShiftKey.press();
        WKey.press();
        sleep(Duration::from_millis(50));
        WKey.release();
        LShiftKey.release();
    });

    // Rapidfire for videogames.
    RightButton.bind(|| while RightButton.is_pressed() {
        LeftButton.press();
        sleep(Duration::from_millis(50));
        LeftButton.release();
    });

    // Mouse movement test
    QKey.bind(|| mouse_move(10, 10));

    // Prevent main thread from exiting.
    park();
}

```