use inputbot::{KeybdKey, MouseButton};

/// This example demonstrates binding all of the keyboard keys and mouse buttons to a
/// simple function. The function prints the key or button name that was pressed.

fn main() {
    // Bind all keys to a common callback event.
    KeybdKey::bind_all(|event| {
        match inputbot::from_keybd_key(event) {
            Some(c) => println!("{c}"),
            None => println!("Unregistered Key"),
        };
    });

    // Bind all release keys to a common callback event (Windows only).
    KeybdKey::bind_all_release(|event| {
        match inputbot::from_keybd_key(event) {
            Some(c) => println!("released {c}"),
            None => println!("Unregistered Key"),
        };
    });

    // Bind all mouse buttons to a common callback event.
    MouseButton::bind_all(|event| {
        println!("{:?}", event);
    });

    // Bind all release mouse buttons to a common callback event (Windows only).
    MouseButton::bind_all_release(|event| {
        println!("released {:?}", event);
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events(false);
}
