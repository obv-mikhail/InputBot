# InputBot [![docs link](https://docs.rs/inputbot/badge.svg)](https://docs.rs/inputbot) [![crates.io version](https://img.shields.io/crates/v/inputbot.svg)](https://crates.io/crates/inputbot) 
Cross-platform (Windows & Linux) library for simulating keyboard/mouse input events and registering global input device event handlers.

Allows writing automation programs that collapse long action-sequences into single key-presses.

## Usage sample

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

    // Create a handler to trigger on any and all keyboard events.
    inputbot::KeybdKey::bind_all(|evnt| {
        println!("{:?}", evnt);
    });

    // Send a key sequence.
    RKey.bind(|| KeySequence("Sample text").send());

    // Move mouse.
    QKey.bind(|| MouseCursor::move_rel(10, 10));

    // Call this to start listening for bound inputs.
    handle_input_events();
}
```

## Build Dependencies
### Debian or Ubuntu based distros
* **libx11-dev**
* **libxtst-dev**
* **libudev-dev**
* **libinput-dev**

**Note:** libinput requires InputBot to be run with sudo on Linux - `sudo ./target/debug/<program name>`.
