use std::{error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleError {
    reason: String,
}

impl SimpleError {
    pub fn new(reason: String) -> Self {
        SimpleError { reason }
    }
}

impl error::Error for SimpleError {}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to parse frame tag data. Reason: {}",
            self.reason.as_str()
        )
    }
}
