use std;
use std::fmt;

#[derive(Debug, Clone)]
pub struct QAPError {
    pub message: String,
    pub file: String,
    pub line: u32,
    pub code: u32,
}

impl QAPError {
    pub fn create(message: &str, file: &str, line: u32, code: u32) -> Self {
        Self { 
            message: message.to_string(), 
            file: file.to_string(), 
            line,
            code,
        }
    }
}

impl fmt::Display for QAPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "QAPError[{}] ({}:{}): Could not create QAP: {}", 
            self.code,
            self.file, 
            self.line, 
            self.message
        )
    }
}
