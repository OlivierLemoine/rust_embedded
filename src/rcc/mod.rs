#![allow(dead_code)]

pub mod raw;

pub struct Rcc{
    base: raw::Rcc,
}

impl Rcc{
    pub fn new() -> Rcc{
        Rcc{
            base: raw::Rcc::new(),
        }
    }

    pub fn enable_hse(self) -> Rcc {
        self.base.hse_on_not_off().set(true);
        while !self.base.hse_is_ready().get() {};
        self
    }

    pub fn enable_hsi(self) -> Rcc {
        self.base.hsi_on_not_off().set(true);
        while !self.base.hsi_is_ready().get() {};
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
}