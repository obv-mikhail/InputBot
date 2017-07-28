extern crate inputbot;

use inputbot::*;
use KeybdKey::*;
use MouseButton::*;
use std::time::Duration;
use std::thread::{sleep, park};

fn main() {
    // Autorun for videogames.
    NumLockKey.bind(|| while num_lock_is_toggled() {
        ShiftKey.press();
        WKey.press();
        sleep(Duration::from_millis(50));
        WKey.release();
        ShiftKey.release();
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
