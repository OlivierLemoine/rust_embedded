#![allow(dead_code)]

use super::super::register::{Bit, Register};

pub struct Rcc {
    base: u32,
}

impl Rcc {
    pub fn new() -> Rcc {
        Rcc { base: 0x4002_3800 }
    }

    pub fn pll_sai_is_ready(&self) -> Bit {
        Bit::new(Register::new(self.base), 25)
    }

    pub fn pll_sai_on_not_off(&self) -> Bit {
        Bit::new(Register::new(self.base), 24)
    }

    pub fn pll_i2s_is_ready(&self) -> Bit {
        Bit::new(Register::new(self.base), 25)
    }

    pub fn pll_i2s_on_not_off(&self) -> Bit {
        Bit::new(Register::new(self.base), 24)
    }

    pub fn main_pll_is_ready(&self) -> Bit {
        Bit::new(Register::new(self.base), 25)
    }

    pub fn main_pll_on_not_off(&self) -> Bit {
        Bit::new(Register::new(self.base), 24)
    }

    pub fn hse_is_ready(&self) -> Bit {
        Bit::new(Register::new(self.base), 17)
    }

    pub fn hse_on_not_off(&self) -> Bit {
        Bit::new(Register::new(self.base), 16)
    }

    pub fn hsi_is_ready(&self) -> Bit {
        Bit::new(Register::new(self.base), 1)
    }

    pub fn hsi_on_not_off(&self) -> Bit {
        Bit::new(Register::new(self.base), 0)
    }

    pub fn pll_r(&self) -> (Bit, Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x04), 30),
            Bit::new(Register::new(self.base + 0x04), 29),
            Bit::new(Register::new(self.base + 0x04), 28),
        )
    }

    pub fn pll_q(&self) -> (Bit, Bit, Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x04), 27),
            Bit::new(Register::new(self.base + 0x04), 26),
            Bit::new(Register::new(self.base + 0x04), 25),
            Bit::new(Register::new(self.base + 0x04), 24),
        )
    }

    //i2s pll = audio pll
    pub fn main_pll_and_i2s_pll_src(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x04), 22)
    }

    pub fn pll_p(&self) -> (Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x04), 17),
            Bit::new(Register::new(self.base + 0x04), 16),
        )
    }

    pub fn pll_n(&self) -> (Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x04), 14),
            Bit::new(Register::new(self.base + 0x04), 13),
            Bit::new(Register::new(self.base + 0x04), 12),
            Bit::new(Register::new(self.base + 0x04), 11),
            Bit::new(Register::new(self.base + 0x04), 10),
            Bit::new(Register::new(self.base + 0x04), 9),
            Bit::new(Register::new(self.base + 0x04), 8),
            Bit::new(Register::new(self.base + 0x04), 7),
            Bit::new(Register::new(self.base + 0x04), 6),
        )
    }

    pub fn pll_m(&self) -> (Bit, Bit, Bit, Bit, Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x04), 5),
            Bit::new(Register::new(self.base + 0x04), 4),
            Bit::new(Register::new(self.base + 0x04), 3),
            Bit::new(Register::new(self.base + 0x04), 2),
            Bit::new(Register::new(self.base + 0x04), 1),
            Bit::new(Register::new(self.base + 0x04), 0),
        )
    }

    pub fn system_clock_status(&self) -> (Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x08), 3),
            Bit::new(Register::new(self.base + 0x08), 2),
        )
    }

    pub fn system_clock_switch(&self) -> (Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x08), 1),
            Bit::new(Register::new(self.base + 0x08), 0),
        )
    }
}