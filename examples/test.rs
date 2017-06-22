extern crate inputbot;

use inputbot::*;
use Event::*;
use codes::*;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    // Autorun for videogames.
    KeybdPress(num_lock()).bind(|| loop {
        keybd_press(shift());
        keybd_press(w());
        sleep(Duration::from_millis(50));
        keybd_release(shift());
        keybd_release(w());
        if !is_toggled(num_lock()) {break}
    });
    // Rapidfire for videogames.
    MousePressRight.bind(|| loop {
        mouse_press_left();
        sleep(Duration::from_millis(50));
        mouse_release_left();
        if !is_pressed(rbutton()) {break}
    });
    // Stops capture.
    KeybdPress(scroll_lock()).bind(|| stop_capture());
    start_capture();
}