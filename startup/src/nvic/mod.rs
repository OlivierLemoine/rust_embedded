use super::register::{Bit, Register};

enum nvic_list {
    tim2_irq_handler = 0x0000_00B0,
}

pub struct NVIC {
    iser: u32,
    icer: u32,
    ispr: u32,
    icpr: u32,
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
