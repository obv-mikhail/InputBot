use inputbot::{KeySequence, KeybdKey};
use serde::Deserialize;
use toml;
/// This example demonstrates sending sequences of key presses / characters via a KeySequence.
/// This can be used, for example, to create a macro which types a specific string.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // If you are on Linux, you may wish to call this function first to avoid a startup delay when
    // the fake device is created. Otherwise, your first input event - if it is a key sequence - may
    // have missing characters.
    //     inputbot::init_device();

    // With the serde feature, KeybdKey and MouseButton are Deserialize, so structs
    // that include these types can derive Deserialize.
    #[derive(Deserialize)]
    struct Config {
        hello: KeybdKey,
        world: KeybdKey,
    }

    let config: Config = toml::from_str(
        r#"
    hello = "numpad1"
    world = "numpad2"
    "#,
    )?;

    config.hello.block_bind(|| {
        KeySequence("Hello,").send();
    });
    config.world.block_bind(|| KeySequence(" World!").send());

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events(false);
    Ok(())
}
