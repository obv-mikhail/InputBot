use inputbot::{BlockInput::*, KeybdKey::*, MouseButton::*, *};
use std::{thread::sleep, time::Duration};

fn main() {

    // Autorun for videogames.
    NumLockKey.bind(|| {
        while NumLockKey.is_toggled() {
            LShiftKey.press();
            WKey.press();
            sleep(Duration::from_millis(50));
            WKey.release();
            LShiftKey.release();
        }
    });

    // Rapidfire for videogames.
    RightButton.bind(|| {
        while RightButton.is_pressed() {
            LeftButton.press();
            sleep(Duration::from_millis(50));
            LeftButton.release();
        }
    });

    // Send a key sequence.
    RKey.bind(|| KeySequence("Sample text").send());

    // Move mouse.
    QKey.bind(|| MouseCursor::move_rel(10, 10));

    // Bind all keys to a common callback event.
    KeybdKey::bind_all(|event| {
        match inputbot::from_keybd_key(event) {
            Some(c) => println!("{}", c),
            None => println!("{}", "Unregistered Key")
        };
    });

    inputbot::handle_input_events();
    // Block the A key when left shift is held.
    // Note: callbacks for blockable binds won't be executed in new threads, 
    //    so for long-running processes create new threads inside the callback if needed. 
    AKey.blockable_bind(|| {
        if LShiftKey.is_pressed() {
            Block
        } else {
            DontBlock
        }
    });

    // Block the A key when left shift is held.
    KKey.block_bind(|| ());

    // Call this to start listening for bound inputs.
    handle_input_events();
}