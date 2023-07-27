use crate::config::offsets::*;
use crate::types::gpio::fsel_pins_per_register::FuncSelPinsPerRegister;
use crate::types::system::byte_size::ByteSize;
use crate::types::system::register_width::RegisterWidth;
pub struct HardwareConfig {
    pub byte_size: ByteSize,
    pub register_width: RegisterWidth,
    pub fsel_pins_per_register: FuncSelPinsPerRegister,
}

impl HardwareConfig {
    pub fn initialize() -> HardwareConfig {
        // TODO check hardware for byte size
        HardwareConfig {
            byte_size: ByteSize(BYTE_SIZE),
            register_width: RegisterWidth(REGISTER_WIDTH),
            fsel_pins_per_register: FuncSelPinsPerRegister(FUNC_SEL_PINS_PER_REGISTER),
        }
    }
}
