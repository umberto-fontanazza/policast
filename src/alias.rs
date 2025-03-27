/* Type aliases are collected here */

use image::{ImageBuffer, Rgba};

pub type StopSignal = ();
pub type Frame = ImageBuffer<Rgba<u8>, Vec<u8>>;
