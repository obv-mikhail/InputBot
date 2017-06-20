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