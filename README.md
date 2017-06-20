# InputBot [![docs link](https://img.shields.io/badge/docs-0.2.0-red.svg)](https://obv-mikhail.github.io/doc/inputbot/) [![crates.io version](https://img.shields.io/crates/v/inputbot.svg)](https://crates.io/crates/inputbot) 
A very small AutoHotkey inspired library for creating global hotkeys, as well as emulating mouse and keyboard input.

# How-To
Hotkeys can be created by matching input within a capture loop.

The code below demonstrates how to create a rapidfire hotkey for videogames.

```Rust
extern crate inputbot;

use inputbot::*;
use Hotkey::*;
use KeybdHotkeyType::*;
use MouseHotkeyType::*;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    register(MouseHotkey(PressRight), || {
        while get_logical_state(vk::RBUTTON) {
            mouse_press_left();
            sleep(Duration::from_millis(50));
            mouse_release_left();
        }
    });
    capture_input();
}
```

Check out the [examples](https://github.com/obv-mikhail/InputBot/tree/master/examples) for more code samples, or read the documentation.