pub enum PinFunction {
    Input,
    Output,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
}

impl Into<u32> for PinFunction {
    fn into(self) -> u32 {
        match self {
            PinFunction::Input => 0x00,
            PinFunction::Output => 0x01,
            PinFunction::Alt0 => 0x02,
            PinFunction::Alt1 => 0x03,
            PinFunction::Alt2 => 0x04,
            PinFunction::Alt3 => 0x05,
            PinFunction::Alt4 => 0x06,
            PinFunction::Alt5 => 0x07,
        }
    }
}
