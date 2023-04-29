use std::{error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleError {
    reason: String,
}

impl SimpleError {
    pub const fn new(reason: String) -> Self {
        Self { reason }
    }
}

impl error::Error for SimpleError {}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.reason)
    }
}
