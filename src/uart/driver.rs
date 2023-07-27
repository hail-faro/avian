use core::fmt::Error;

use crate::config::uart::UartConfig;
use crate::gpio::driver::GpioSystem;

pub struct UartDriver {
    config: UartConfig,
    gpio: GpioSystem,
}

impl UartDriver {
    pub fn new(config: UartConfig, gpio: GpioSystem) {}

    // pub fn read() -> Result<u8, Error> {}

    // pub fn write() -> Result<(), Error> {}

    // pub fn set_config() -> Result<(), Error> {}
}
