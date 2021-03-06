#![allow(dead_code)]

use super::register::{Bit, Register};

// ISER = 0xE000_E100
// ICER = 0xE000_E180
// ISPR = 0xE000_E200
// ICPR = 0xE000_E280

pub struct NVIC {}

impl NVIC {
    pub fn new() -> NVIC {
        NVIC {}
    }

    pub fn tim2_set_enabled(&mut self) -> Bit {
        Bit::new(Register::new(0xE000_E100 /*+ 0*/), 28)
    }

    pub fn usart2_set_enabled(&mut self) -> Bit {
        Bit::new(Register::new(0xE000_E100 + 4), 6)
    }

    pub fn usart4_set_enabled(&mut self) -> Bit {
        Bit::new(Register::new(0xE000_E100 + 4), 20)
    }
}
