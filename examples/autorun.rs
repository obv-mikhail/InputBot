extern crate inputbot;

use inputbot::{capture_input, get_toggle_state};
use inputbot::Input::Keybd;
use inputbot::KeybdInput::*;
use inputbot::vk;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    while let Some(input) = capture_input() {
        match input {
            // An autorun for videogames activated when NumLock is on
            Keybd(Release, vk::NUMLOCK) => {
                while get_toggle_state(vk::NUMLOCK) {
                    Keybd(Press, vk::SHIFT).send();
                    Keybd(Press, vk::W).send();
                    sleep(Duration::from_millis(50));
                    Keybd(Release, vk::SHIFT).send();
                    Keybd(Release, vk::W).send();
                }
            },
            _ => {}
        }
    }
}