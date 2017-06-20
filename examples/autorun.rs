extern crate inputbot;

use inputbot::*;
use Hotkey::*;
use KeybdHotkeyType::*;
use MouseHotkeyType::*;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    register(KeybdHotkey(Press, vk::NUMLOCK), || {
        while get_toggle_state(vk::NUMLOCK) {
            keybd_press(vk::SHIFT);
            keybd_press(vk::W);
            sleep(Duration::from_millis(50));
            keybd_release(vk::SHIFT);
            keybd_release(vk::W);
        }
    });
    capture_input();
}