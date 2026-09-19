use std::array::TryFromSliceError;

use crate::platform::error::PlatformError;

pub enum EditorError {
    Platform(PlatformError),
    Io(std::io::Error),
    FileOpen(String),
    TrySlice(TryFromSliceError),
}

impl From<PlatformError> for EditorError {
    fn from(value: PlatformError) -> Self {
        EditorError::Platform(value)
    }
}

impl From<TryFromSliceError> for EditorError {
    fn from(value: TryFromSliceError) -> Self {
        EditorError::TrySlice(value)
    }
}

impl From<std::io::Error> for EditorError {
    fn from(value: std::io::Error) -> Self {
        EditorError::Io(value)
    }
}

impl std::fmt::Display for EditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorError::FileOpen(path) => write!(f, "File at: {path} is not a map"),
            _ => write!(f, "Unknown error type for Editor"),
        }
    }
}
