//! A module to contain general public API items

use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ImgConversionError {
    details: String,
}

impl ImgConversionError {
    pub fn new(msg: &str) -> ImgConversionError {
        ImgConversionError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ImgConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for ImgConversionError {
    fn description(&self) -> &str {
        &self.details
    }
}
