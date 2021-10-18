#![allow(unused)]
#![allow(dead_code)]

use crate::KeybdKey;
mod inputs;

use core_graphics::event::{
    CGEvent, CGEventTapLocation, CGEventType
};
use std::{
    thread::sleep, time::Duration
};
use core_graphics::event_source::{
    CGEventSource, CGEventSourceStateID,
};


impl KeybdKey {
    #[inline(always)]
    pub fn is_pressed(self) -> bool {
        let code = u64::from(self);
        unsafe {
            return CGEventSourceKeyState(CGEventSourceStateID::CombinedSessionState, code);
        }
    }

    pub fn press(self) {
        let event_source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
                .expect("Could not create CGEventSource.");

        let press_event = CGEvent::new_keyboard_event(event_source.clone(), u64::from(self) as u16, true)
            .expect("Could not create CGEvent for KeyPress")
            .post(CGEventTapLocation::HID);
        
        sleep(Duration::from_millis(50));

        self.release();
    }

    pub fn release(self) {

        let event_source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
        .expect("Could not create CGEventSource.");

        let press_event = CGEvent::new_keyboard_event(event_source.clone(), u64::from(self) as u16, false)
            .expect("Could not create CGEvent for KeyPress (release)")
            .post(CGEventTapLocation::HID);

        sleep(Duration::from_millis(50));

        // todo!("I'm not implemented yet")
    }

    pub fn is_toggled(self) -> bool {
        todo!("I'm not implemented yet")
    }
}



// I learned how to link the MacOS native lib from here:
// originally I was going to try and use bindgen along with a 
// c header file with the appropriate library included, but this was a 
// life saver to be honest --> https://github.com/segeljakt/readkey/blob/master/src/lib.rs and
// also from --> https://github.com/servo/core-foundation-rs/blob/master/cocoa/src/appkit.rs
#[link(name = "AppKit", kind = "framework")]
extern {
    fn CGEventSourceKeyState(state: CGEventSourceStateID, keycode: u64) -> bool;
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

    #[test]
    fn test_keypress() {
        use std::thread;
        let mut worked = false;

        // I spawn a thread so that the .is_pressed call
        // below can register the keypress before it is
        // released
        thread::spawn(|| {
            KeybdKey::UpKey.press();
        });

        if KeybdKey::UpKey.is_pressed() {
            worked = true;
            println!("Up key was pressed!");
        }

        assert_eq!(worked, true)
    }

    #[test]
    fn test_keyrelease() {
        use std::thread;
        let mut worked = false;

        // this is kind of an iffy test. I am relying on the fact that
        // without threading the press the .is_pressed call will not
        // register the press (which means the call to self.release in 
        // self.press works)
        KeybdKey::UpKey.press();

        if KeybdKey::UpKey.is_pressed() {
            println!("Up key was pressed :(");
        } else {
            worked = true;
        }

        assert_eq!(worked, true)
    }
}