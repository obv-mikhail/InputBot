extern crate inputbot;

use inputbot::*;
use Event::*;
use codes::*;
use std::time::Duration;
use std::thread::{sleep, park};

fn main() {
    // Autorun for videogames.
    KeybdPress(num_lock()).bind(|| while is_toggled(num_lock()) {
        keybd_press(shift());
        keybd_press(w());
        sleep(Duration::from_millis(50));
        keybd_release(shift());
        keybd_release(w());
    });
    // Rapidfire for videogames.
    MousePressRight.bind(|| while is_pressed(rbutton()) {
        mouse_press_left();
        sleep(Duration::from_millis(50));
        mouse_release_left();
    });
    park();
}