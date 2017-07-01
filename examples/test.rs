extern crate inputbot;

use inputbot::*;
use Event::*;
use codes::*;
use std::time::Duration;
use std::thread::{sleep, park};

fn main() {
    // Autorun for videogames.
    KeybdPress(NUM_LOCK).bind(||
        while is_toggled(NUM_LOCK) {
            keybd_press(SHIFT);
            keybd_press(W);
            sleep(Duration::from_millis(50));
            keybd_release(SHIFT);
            keybd_release(W);
        }
    );

    // Rapidfire for videogames.
    MousePressRight.bind(||
        while is_pressed(RBUTTON) {
            mouse_press_left();
            sleep(Duration::from_millis(50));
            mouse_release_left();
        }
    );

    // Prevent main thread from exiting.
    park();
}