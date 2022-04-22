use inputbot::{KeybdKey, MouseButton};

/// This example demonstrates binding all of the keyboard and mouse buttons to a simple function.
/// The function prints the key or button that was pressed.

fn main() {
    // This is not strictly neccesary, but by calling this function, you can avoid a 'startup delay'
    // when executing your first callback.
    inputbot::init_device();

    // Bind all keys to a common callback event.
    KeybdKey::bind_all(|event| {
        match inputbot::from_keybd_key(event) {
            Some(c) => println!("{c}"),
            None => println!("Unregistered Key"),
        };
    });

    // Bind all mouse buttons to a common callback event.
    MouseButton::bind_all(|event| {
        println!("{:?}", event);
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
