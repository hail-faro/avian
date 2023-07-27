use core::arch::asm;

pub fn delay(count: u32) {
    for _ in 0..count {
        unsafe {
            asm!("nop");
        }
    }
}
