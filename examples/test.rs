extern crate inputbot;

use inputbot::*;
use Event::*;
use codes::*;
use std::time::Duration;
use std::thread::{sleep, park};

fn main() {
    // Autorun for videogames.
    KeybdPress(*E).bind(||
        {
            keybd_press(*W);
            sleep(Duration::from_millis(50));
            keybd_release(*W);
        }
    );
    KeybdPress(*R).bind(||
        {
            keybd_press(*F);
            sleep(Duration::from_millis(50));
            keybd_release(*F);
                        keybd_press(*F);
            sleep(Duration::from_millis(50));
            keybd_release(*F);
        }
    );

   MousePressLeft.bind(||
        {
            keybd_press(*F);
            sleep(Duration::from_millis(50));
            keybd_release(*F);
                        keybd_press(*F);
            sleep(Duration::from_millis(50));
            keybd_release(*F);
        }
    );
    // Prevent main thread from exiting.
    park();
}