/* Type aliases are collected here */

use egui::{Key, Modifiers};
use image::{ImageBuffer, Rgba};

pub type StopSignal = ();
pub type Frame = ImageBuffer<Rgba<u8>, Vec<u8>>;
pub type KeyCombo = (Modifiers, Key);
