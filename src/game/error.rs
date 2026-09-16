use crate::platform::error::PlatformError;

//Same stuff as platform errors, but for game, wraps platform
//________________________

pub enum GameError {
    Platform(PlatformError),
}

impl From<PlatformError> for GameError {
    fn from(value: PlatformError) -> Self {
        GameError::Platform(value)
    }
}
