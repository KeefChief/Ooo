use crate::platform::{Platform, error::PlatformError};

//That spaeks for itself
//______________________

pub trait GameInterface {
    type Error: From<PlatformError>;

    fn init(&mut self, platform: &mut Platform) -> Result<(), Self::Error>;

    fn update(&mut self, platform: &mut Platform, delta: f64);

    fn draw(&mut self, platform: &mut Platform);
}
