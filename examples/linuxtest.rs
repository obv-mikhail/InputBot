extern crate inputbot;

use inputbot::*;
use Event::*;
use codes::*;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    MousePressLeft.bind(|| {
        keybd_press(x());
        sleep(Duration::from_millis(50));
        keybd_release(x());
    });
    start_capture();
}
