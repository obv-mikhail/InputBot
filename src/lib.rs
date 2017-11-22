#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::collections::hash_map::HashMap;

mod common;
use common::*;

mod public;
pub use public::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;
#[cfg(target_os = "windows")]
pub use windows::inputs::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "linux")]
pub use linux::inputs::*;
