//! A module to contain general public API items

use std::error;
use std::fmt;

#[derive(Debug)]
pub struct MyError {
    details: String
}

impl MyError {
    pub fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl error::Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}
