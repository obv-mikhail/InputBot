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
