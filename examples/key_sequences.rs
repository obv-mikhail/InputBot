use inputbot::{KeySequence, KeybdKey::*};

/// This example demonstrates sending sequences of key presses / characters via a KeySequence.
/// This can be used, for example, to create a macro which types a specific string.

fn main() {
    // This is not strictly neccesary, but by calling this function, you can avoid a 'startup delay'
    // when executing your first callback. This can impact key sequences by cutting off the first
    // few characters.
    inputbot::init_device();

    // Bind our Backquote key (`, ~) to a function that types out the string "Hello, world!".
    // You must remember to call the `.send()` method on the KeySequence after creating it.
    // You could explicitly define the KeySequence ahead of time and send it later like so:
    //      let seq: KeySequence = KeySequence("Hello, world!");
    //      seq.send();
    BackquoteKey.bind(|| {
        KeySequence("Hello, world!").send();
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
