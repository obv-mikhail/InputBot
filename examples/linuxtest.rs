extern crate inputbot;

use inputbot::*;
use Event::*;
use codes::*;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    KeybdPress(w()).bind(|| {
        println!("t");
        sleep(Duration::from_millis(500));
        keybd_press(x());
        sleep(Duration::from_millis(50));
        keybd_release(x());
    });
    start_capture();
}
