use inputbot::{
    BlockInput::*,
    KeybdKey::*,
    MouseButton::{LeftButton, MousewheelDown},
};
use std::thread;
// This example demonstrates blocking input with conditional flags, such as another key being
// pressed or toggled. This example currently does not work on Linux.

fn main() {
    // Block the A key when left shift is held. Note: callbacks for blockable binds won't be
    // executed in new threads, so for long-running processes create new threads inside the callback
    // if needed.
    AKey.blockable_bind(|| {
        if LShiftKey.is_pressed() {
            Block
        } else {
            DontBlock
        }
    });

    // Block the K key when left shift is held.
    KKey.block_bind(|| ());

    MousewheelDown.blockable_bind(|| {
        if LControlKey.is_pressed() {
            // Unlike block_bind and bind, blockable_bind runs the callback synchronously,
            // on the hooking thread.
            // This can cause performance issues one some operating systems, particularly Windows 11.
            // A solution is to spawn the work part of the callback onto a new thread
            // and synchronously return the BlockInput.

            // DANGER: Don't Send mouse input on the mouse hook thread!
            // LeftButton.press(); LeftButton.release();
            thread::spawn(|| {
                // Safe: executing on another thread.
                LeftButton.press();
                LeftButton.release();
                println!("Mousewheel Down")
            });
            return Block;
        }
        DontBlock
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events(false);
}
