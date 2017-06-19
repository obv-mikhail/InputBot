# InputBot [![alt text](https://docs.rs/InputBot/badge.svg)](https://docs.rs/InputBot)
A very small AutoHotkey inspired library for creating global hotkeys, as well as emulating mouse and keyboard input.

# How-To
Hotkeys can be created by matching input within a capture loop.

The code below demonstrates how to create a rapidfire hotkey for videogames.

```Rust
extern crate inputbot;

use inputbot::{capture_input, get_key_state, send_input};
use inputbot::Input::Mouse;
use inputbot::MouseInput::{PressLeft, ReleaseLeft, PressRight};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    while let Some(input) = capture_input() {
        match input {
            Mouse(PressRight, _, _) => {
                while get_key_state(0x02) {
                    send_input(Mouse(PressLeft, 0, 0));
                    sleep(Duration::from_millis(50));
                    send_input(Mouse(ReleaseLeft, 0, 0));
                }
            },
            _ => {}
        }
    }
}
```

Check out the examples for more code samples, or read the documentation.