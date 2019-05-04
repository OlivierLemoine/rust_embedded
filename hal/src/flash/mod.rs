#![allow(dead_code)]

use super::register::{Bit, Register};

pub struct Flash{
    base: u32,
}

impl Flash{
    pub fn new() -> Flash {
        Flash{
            base: 0x4002_3C00
        }
    }

    pub fn latency(&self) -> (Bit, Bit, Bit, Bit) {
        (
            Bit::new(Register::new(self.base), 3),
            Bit::new(Register::new(self.base), 2),
            Bit::new(Register::new(self.base), 1),
            Bit::new(Register::new(self.base), 0),
        )
    }
}