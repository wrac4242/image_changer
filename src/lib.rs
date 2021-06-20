//! # Image Changer
//!
//! A library for basic image filtering.

pub mod img;
pub mod filters;
pub(self) mod utils;

pub use img::Img;
pub use img::save;
pub use img::new_from_file;
