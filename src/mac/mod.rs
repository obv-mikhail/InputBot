#![allow(unused)]
#![allow(dead_code)]

use crate::KeybdKey;
mod inputs;

impl KeybdKey {
    #[inline(always)]
    pub fn is_pressed(self) -> bool {
        let code = u64::from(self);
        unsafe {
            return CGEventSourceKeyState(0, code);
        }
    }

    pub fn press(self) {
        todo!("I'm not implemented yet")
    }

    pub fn release(self) {
        todo!("I'm not implemented yet")
    }

    pub fn is_toggled(self) -> bool {
        todo!("I'm not implemented yet")
    }
}

// I learned how to link the MacOS native lib from here:
// originally I was going to try and use bindgen along with a 
// c header file with the appropriate library included, but this was a 
// life saver to be honest --> https://github.com/segeljakt/readkey/blob/master/src/lib.rs
#[link(name = "AppKit", kind = "framework")]
extern {
    // -1: private
    // 0: combined
    // 1: system
    fn CGEventSourceKeyState(state: i32, keycode: u64) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_keypress() {
        loop {
            if KeybdKey::UpKey.is_pressed() {
                println!("Up key was pressed!");
            } else if KeybdKey::FKey.is_pressed() {
                // this can only be done with elevated privilege
                println!("F key was pressed");
            }
        }
    }
}