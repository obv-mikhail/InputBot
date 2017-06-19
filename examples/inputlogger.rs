extern crate inputbot;

use inputbot::{capture_input};

fn main() {
    while let Some(input) = capture_input() {
        println!("{:?}", input)
    }
}