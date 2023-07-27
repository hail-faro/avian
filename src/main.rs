#![cfg_attr(not(test), no_std)]
#![no_main]

mod config;
mod gpio;
mod types;
mod uart;
mod utils;

use crate::gpio::driver::GpioSystem;
use crate::types::gpio::{pin_function::*, pin_level::*};
use crate::utils::delay;
use core::panic::PanicInfo;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// struct Pin<'a> {
//     gpio_system: &'a GpioSystem,
//     pin_num: u8,
//     pin_function: PinFunction,
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Entry point for AvianOS
/// Performs startup operations and then enters the main loop
/// # Arguments
/// None
/// # Returns
/// None
/// # Panics
/// Panics if the hardware is not supported
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let gpio_config = GpioSystem::initialize().unwrap();

    gpio_config.set_fsel(17, PinFunction::Output);
    gpio_config.set_fsel(26, PinFunction::Output);

    gpio_config.write(26, PinLevel::High);

    loop {
        gpio_config.write(17, PinLevel::High);
        delay(10000000);
        gpio_config.write(17, PinLevel::Low);
        delay(10000000);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.add(i) = c as u8;
        i += 1;
    }
    s
}
