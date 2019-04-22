#![allow(dead_code)]

pub mod raw;

pub struct Rcc {
    base: raw::Rcc,
}

impl Rcc {
    pub fn new() -> Rcc {
        Rcc {
            base: raw::Rcc::new(),
        }
    }

    pub fn enable_hse(self) -> Rcc {
        self.base.hse_on_not_off().set(true);
        while !self.base.hse_is_ready().get() {}
        self
    }

    pub fn enable_hsi(self) -> Rcc {
        self.base.hsi_on_not_off().set(true);
        while !self.base.hsi_is_ready().get() {}
        self
    }

    pub fn sysclock_into_hsi(self) -> Rcc {
        let (mut b1, mut b2) = self.base.system_clock_switch();
        b1.set(false);
        b2.set(false);
        self
    }

    pub fn sysclock_into_hse(self) -> Rcc {
        let (mut b1, mut b2) = self.base.system_clock_switch();
        b1.set(false);
        b2.set(true);
        self
    }

    pub fn set_pll_r(self, value: u8) -> Rcc {
        let (mut b1, mut b2, mut b3) = self.base.pll_r();
        b1.set((value & 0x04) == 0x04);
        b2.set((value & 0x02) == 0x02);
        b3.set((value & 0x01) == 0x01);
        self
    }

    pub fn set_pll_q(self, value: u8) -> Rcc {
        let (mut b1, mut b2, mut b3, mut b4) = self.base.pll_q();
        b1.set((value & 0x08) == 0x08);
        b2.set((value & 0x04) == 0x04);
        b3.set((value & 0x02) == 0x02);
        b4.set((value & 0x01) == 0x01);
        self
    }

    pub fn main_pll_src_into_hsi(self) -> Rcc {
        self.base.main_pll_and_i2s_pll_src().set(false);
        self
    }

    pub fn main_pll_src_into_hse(self) -> Rcc {
        self.base.main_pll_and_i2s_pll_src().set(true);
        self
    }
}