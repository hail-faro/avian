use crate::config::hardware::HardwareConfig;
use crate::types::gpio::{pin_function::*, pin_level::*};

use crate::config::offsets::*;
use crate::types::gpio::clr_offset::ClrOffset;
use crate::types::gpio::fsel_offset::FSelOffset;
use crate::types::gpio::level_offset::LevelOffset;
use crate::types::gpio::pull_up_down_offset::PullUpDownOffset;
use crate::types::gpio::set_offset::SetOffset;
use crate::types::system::address::Address;
use core::fmt::Error;

pub struct GpioSystem {
    base_address: Address,
    fsel_offset: FSelOffset,
    set_offset: SetOffset,
    clr_offset: ClrOffset,
    level_offset: LevelOffset,
    pud_offset: PullUpDownOffset,
    hardware_config: HardwareConfig,
}

impl GpioSystem {
    pub fn initialize() -> Result<GpioSystem, Error> {
        // TODO check hardware and return specific settings

        // Rasp Pi 4 B
        Ok(GpioSystem {
            base_address: Address(0xFE200000),
            fsel_offset: FSelOffset(GPIO_FSEL_OFFSET),
            set_offset: SetOffset(GPIO_PIN_SET_OFFSET),
            clr_offset: ClrOffset(GPIO_PIN_CLR_OFFSET),
            level_offset: LevelOffset(GPIO_PIN_LVL_OFFSET),
            pud_offset: PullUpDownOffset(GPIO_PIN_PULL_UP_DOWN_OFFSET),
            hardware_config: HardwareConfig::initialize(),
        })
    }

    fn clear_mask(&self, shift_size: u32) -> u32 {
        // TODO Set this to be hw specific
        !(7 << shift_size) as u32
    }

    fn set_mask(&self, function: u32, shift_size: u32) -> u32 {
        (function << shift_size) as u32
    }

    fn calc_pin_register_offset(&self, gpio_pin: u8) -> Result<u32, Error> {
        // TODO Check pin out of range
        Ok(self.base_address.0
            + ((gpio_pin as u32 / self.hardware_config.register_width.0)
                * self.hardware_config.byte_size.0))
    }

    fn calc_pin_fsel_offset(&self, gpio_pin: u8) -> Result<u32, Error> {
        // TODO Set this to be hw specific (divide by 10)
        Ok(self.base_address.0
            + self.fsel_offset.0
            + (self.hardware_config.byte_size.0
                * (gpio_pin as u32 / self.hardware_config.fsel_pins_per_register.0)))
    }

    pub fn set_fsel(&self, gpio_pin: u8, function: PinFunction) -> Result<(), Error> {
        // TODO Set this to be hw specific (func to u8 conversion)
        let func: u32 = function.into();
        let gpio_fseln: *mut u32 = self.calc_pin_fsel_offset(gpio_pin)? as *mut u32;

        // TODO Set this to be hw specific (mod 10 times 3)
        let shift_size = (gpio_pin as u32 % self.hardware_config.fsel_pins_per_register.0) * 3;

        // NOTE Check this safe?
        unsafe {
            gpio_fseln.write_volatile(
                (gpio_fseln.read_volatile() & self.clear_mask(shift_size))
                    | self.set_mask(func, shift_size),
            );
        }

        Ok(())
    }

    pub fn read(&self, gpio_pin: u8) -> Result<PinLevel, Error> {
        let gpio_register_addr: *mut u32 =
            (self.calc_pin_register_offset(gpio_pin)? + self.level_offset.0) as *mut u32;

        // TODO check gpio_pin range
        unsafe {
            let reading = gpio_register_addr.read_volatile()
                & (1 << gpio_pin as u32 % self.hardware_config.register_width.0 as u32);

            match reading {
                0 => Ok(PinLevel::Low),
                _ => Ok(PinLevel::High),
            }
        }
    }

    pub fn write(&self, gpio_pin: u8, level: PinLevel) -> Result<(), Error> {
        let offset: u32 = match level {
            PinLevel::High => self.set_offset.0,
            PinLevel::Low => self.clr_offset.0,
        };

        let gpio_register_addr: *mut u32 =
            (self.calc_pin_register_offset(gpio_pin)? + offset) as *mut u32;

        unsafe {
            gpio_register_addr.write_volatile(1 << gpio_pin);
        }

        Ok(())
    }
}
