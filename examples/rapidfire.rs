extern crate inputbot;

use inputbot::{capture_input, get_key_state};
use inputbot::Input::Mouse;
use inputbot::MouseInput::{PressLeft, ReleaseLeft, PressRight};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    while let Some(input) = capture_input() {
        match input {
            Mouse(PressRight, _, _) => {
                while get_key_state(0x02) {
                    Mouse(PressLeft, 0, 0).send();
                    sleep(Duration::from_millis(50));
                    Mouse(ReleaseLeft, 0, 0).send();
                }
            },
            _ => {}
        }
    }
}