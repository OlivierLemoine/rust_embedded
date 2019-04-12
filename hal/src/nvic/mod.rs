use super::register::{Bit, Register};

pub struct NVIC {
    pub iser: u32,
    pub icer: u32,
    pub ispr: u32,
    pub icpr: u32,
}

impl NVIC {
    pub fn new() -> NVIC {
        NVIC {
            iser: 0xE000_E100,
            icer: 0xE000_E180,
            ispr: 0xE000_E200,
            icpr: 0xE000_E280,
        }
    }

    pub fn tim2_set_enabled(&mut self) -> Bit {
        Bit::new(Register::new(self.iser + 0), 28)
    }
}
