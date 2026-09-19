use crate::platform::error::PlatformError;

pub enum EditorError {
    Platform(PlatformError) 
}

impl From<PlatformError> for EditorError {
    fn from(value: PlatformError) -> Self {
        EditorError::Platform(value)
    }
}
