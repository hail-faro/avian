// hardware standards
pub const BYTE_SIZE: u32 = 4;
pub const REGISTER_WIDTH: u32 = 32;
pub const FUNC_SEL_PINS_PER_REGISTER: u32 = 10;

// pi 4 GPIO offsets
pub const GPIO_FSEL_OFFSET: u32 = 0x00;
pub const GPIO_PIN_SET_OFFSET: u32 = 0x1c;
pub const GPIO_PIN_CLR_OFFSET: u32 = 0x28;
pub const GPIO_PIN_LVL_OFFSET: u32 = 0x34;
pub const GPIO_PIN_PULL_UP_DOWN_OFFSET: u32 = 0xE4;

// pi 4 UART offsets
