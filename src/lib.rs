//! # Image Changer
//!
//! A library for basic image filtering.

pub mod filters;
pub mod img;
pub mod misc;
mod utils;

pub use img::new_from_file;
pub use img::save;
pub use img::Img;
