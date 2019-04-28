#![allow(dead_code)]

use super::register::{Bit, Register, Register8};

pub struct Mmu {
    base: u32,
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu { base: 0xE000_ED90 }
    }

    pub fn type_reg(&self) -> Register {
        Register::new(self.base)
    }

    pub fn priviliged_software_default_map(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x04), 2)
    }

    pub fn mpu_in_interrupt_handler(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x04), 2)
    }

    pub fn enable(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x04), 0)
    }

    pub fn region_number_register(&self) -> Register8 {
        Register8::new(self.base + 0x08)
    }

    pub fn region_base_address(&self) -> Register {
        Register::new(self.base + 0x0C)
    }
}