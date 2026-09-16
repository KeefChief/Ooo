use std::io::Error;

use sdl2::{IntegerOrSdlError, render::SdlError, video::WindowBuildError};

//Platform error handling
//TODO: Understand why theres' an unused error on error types data and maybe fix it
//_____________________

#[derive(Debug)]
pub enum PlatformError {
    Sdl(SdlError),
    IntOrSdl(IntegerOrSdlError),
    Window(WindowBuildError),
    Io(Error),
    String(String)
}

//Implementing From for necessary error types
//_____________________

impl From<SdlError> for PlatformError {
    fn from(value: SdlError) -> Self {
        PlatformError::Sdl(value)
    }
}

impl From<WindowBuildError> for PlatformError {
    fn from(value: WindowBuildError) -> Self {
        PlatformError::Window(value)
    }
}

impl From<IntegerOrSdlError> for PlatformError {
    fn from(value: IntegerOrSdlError) -> Self {
        PlatformError::IntOrSdl(value)
    }
}

impl From<Error> for PlatformError {
    fn from(value: Error) -> Self {
        PlatformError::Io(value)
    }
}

impl From<String> for PlatformError {
    fn from(value: String) -> Self {
        PlatformError::String(value)
    }
}
