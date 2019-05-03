#![allow(dead_code)]

pub mod raw;

pub const HSI_SPEED: u32 = 16_000_000;

static mut PLL_M: u32 = 1;
static mut PLL_N: u32 = 1;
static mut PLL_P: u32 = 1;
static mut PLL_Q: u32 = 1;
static mut PLL_R: u32 = 1;

static mut SYS_CLOCK_SPEED: u32 = 1;

static mut AHB_PRESC: u32 = 1;
static mut APB1_PRESC: u32 = 1;
static mut APB2_PRESC: u32 = 1;

static mut AHB_SPEED: u32 = 1;
static mut APB1_SPEED: u32 = 1;
static mut APB2_SPEED: u32 = 1;

pub fn get_usart_speed() -> u32 {
    unsafe { APB1_SPEED }
}

pub mod pll_state {
    pub struct On;
    pub struct Off;
}

pub struct Rcc<STATE> {
    base: raw::Rcc,

    pll_state: STATE,
}

impl Rcc<pll_state::Off> {
    pub fn new() -> Rcc<pll_state::Off> {
        Rcc {
            base: raw::Rcc::new(),

            pll_state: pll_state::Off,
        }
    }

    pub fn get_sys_clock_speed() -> u32 {
        0
    }

    pub fn enable_pll(self) -> Result<Rcc<pll_state::On>, bool> {
        let vco_in = HSI_SPEED / unsafe { PLL_M };
        if vco_in < 1_000_000 || vco_in > 2_000_000 {
            return Err(false);
        }

        let vco_out = vco_in * unsafe { PLL_N };
        if vco_out <= 100_000_000 || vco_out > 432_000_000 {
            return Err(false);
        }

        let usb_otg_fs = vco_out / unsafe { PLL_Q };
        if usb_otg_fs != 48_000_000 {
            return Err(false);
        }

        let pll_out = vco_out / unsafe { PLL_P };
        if pll_out > 180_000_000 {
            return Err(false);
        }

        unsafe { SYS_CLOCK_SPEED = pll_out };

        let ahb_out = pll_out / unsafe { AHB_PRESC };

        unsafe { AHB_SPEED = pll_out };

        let apb1_out = ahb_out / unsafe { APB1_PRESC };
        if apb1_out > 45_000_000 {
            return Err(false);
        }

        unsafe { APB1_SPEED = pll_out };

        let apb2_out = ahb_out / unsafe { APB2_PRESC };
        if apb2_out > 90_000_000 {
            return Err(false);
        }

        unsafe { APB2_SPEED = pll_out };

        self.base.main_pll_on_not_off().set(true);

        Ok(Rcc {
            base: self.base,
            pll_state: pll_state::On,
        })
    }

    pub fn set_pll_r(self, value: u8) -> Result<Rcc<pll_state::Off>, bool> {
        if value < 2 || value > 7 {
            return Err(false);
        }
        let (mut b1, mut b2, mut b3) = self.base.pll_r();
        b1.set((value & 0x04) == 0x04);
        b2.set((value & 0x02) == 0x02);
        b3.set((value & 0x01) == 0x01);

        unsafe { PLL_R = value as u32 };

        Ok(self)
    }

    pub fn set_pll_q(self, value: u8) -> Rcc<pll_state::Off> {
        let (mut b1, mut b2, mut b3, mut b4) = self.base.pll_q();
        b1.set((value & 0x08) == 0x08);
        b2.set((value & 0x04) == 0x04);
        b3.set((value & 0x02) == 0x02);
        b4.set((value & 0x01) == 0x01);

        unsafe { PLL_Q = value as u32 };

        self
    }

    pub fn main_pll_src_into_hsi(self) -> Rcc<pll_state::Off> {
        self.base.main_pll_and_i2s_pll_src().set(false);
        self
    }

    pub fn main_pll_src_into_hse(self) -> Rcc<pll_state::Off> {
        self.base.main_pll_and_i2s_pll_src().set(true);
        self
    }

    pub fn set_pll_p(self, div_factor: u8) -> Rcc<pll_state::Off> {
        let (mut b1, mut b2) = self.base.pll_p();
        b1.set(match div_factor {
            6 | 8 => true,
            _ => false,
        });
        b2.set(match div_factor {
            4 | 8 => true,
            _ => false,
        });

        unsafe { PLL_P = div_factor as u32 };

        self
    }

    pub fn set_pll_n(self, value: u16) -> Result<Rcc<pll_state::Off>, bool> {
        if value < 50 || value > 432 {
            return Err(false);
        }
        let (mut b1, mut b2, mut b3, mut b4, mut b5, mut b6, mut b7, mut b8, mut b9) =
            self.base.pll_n();
        b1.set((value & 0x100) == 0x100);
        b2.set((value & 0x080) == 0x080);
        b3.set((value & 0x040) == 0x040);
        b4.set((value & 0x020) == 0x020);
        b5.set((value & 0x010) == 0x010);
        b6.set((value & 0x008) == 0x008);
        b7.set((value & 0x004) == 0x004);
        b8.set((value & 0x002) == 0x002);
        b9.set((value & 0x001) == 0x001);

        unsafe { PLL_N = value as u32 };

        Ok(self)
    }

    pub fn set_pll_m(self, value: u8) -> Result<Rcc<pll_state::Off>, bool> {
        if value < 2 || value > 63 {
            return Err(false);
        }
        let (mut b1, mut b2, mut b3, mut b4, mut b5, mut b6) = self.base.pll_m();
        b1.set((value & 0x20) == 0x20);
        b2.set((value & 0x10) == 0x10);
        b3.set((value & 0x08) == 0x08);
        b4.set((value & 0x04) == 0x04);
        b5.set((value & 0x02) == 0x02);
        b6.set((value & 0x01) == 0x01);

        unsafe { PLL_M = value as u32 };

        Ok(self)
    }

    pub fn set_ahb_prescaler(self, value: u16) -> Rcc<pll_state::Off> {
        let (mut b1, mut b2, mut b3, mut b4) = self.base.ahb();
        b1.set(match value {
            2 | 4 | 8 | 16 | 64 | 128 | 256 | 512 => true,
            _ => false,
        });
        b2.set(match value {
            64 | 128 | 256 | 512 => true,
            _ => false,
        });
        b3.set(match value {
            8 | 16 | 256 | 512 => true,
            _ => false,
        });
        b4.set(match value {
            4 | 16 | 128 | 512 => true,
            _ => false,
        });

        unsafe { AHB_PRESC = value as u32 };

        self
    }

    pub fn set_apb1_prescaler(self, value: u8) -> Rcc<pll_state::Off> {
        let (mut b1, mut b2, mut b3) = self.base.apb_1();
        b1.set(match value {
            2 | 4 | 8 | 16 => true,
            _ => false,
        });
        b2.set(match value {
            8 | 16 => true,
            _ => false,
        });
        b3.set(match value {
            4 | 16 => true,
            _ => false,
        });

        unsafe { APB1_PRESC = value as u32 };

        self
    }

    pub fn set_apb2_prescaler(self, value: u8) -> Rcc<pll_state::Off> {
        let (mut b1, mut b2, mut b3) = self.base.apb_2();
        b1.set(match value {
            2 | 4 | 8 | 16 => true,
            _ => false,
        });
        b2.set(match value {
            8 | 16 => true,
            _ => false,
        });
        b3.set(match value {
            4 | 16 => true,
            _ => false,
        });

        unsafe { APB2_PRESC = value as u32 };

        self
    }
}

impl<STATE> Rcc<STATE> {
    pub fn enable_hse(self) -> Rcc<STATE> {
        self.base.hse_on_not_off().set(true);
        while !self.base.hse_is_ready().get() {}
        self
    }

    pub fn enable_hsi(self) -> Rcc<STATE> {
        self.base.hsi_on_not_off().set(true);
        while !self.base.hsi_is_ready().get() {}
        self
    }

    pub fn sysclock_into_hsi(self) -> Rcc<STATE> {
        let (mut b1, mut b2) = self.base.system_clock_switch();
        b1.set(false);
        b2.set(false);

        unsafe { SYS_CLOCK_SPEED = HSI_SPEED };

        self
    }

    pub fn sysclock_into_hse(self) -> Rcc<STATE> {
        let (mut b1, mut b2) = self.base.system_clock_switch();
        b1.set(false);
        b2.set(true);
        self
    }
}

impl Rcc<pll_state::On> {
    pub fn sysclock_into_pll(self) -> Result<Rcc<pll_state::On>, bool> {
        let (mut b1, mut b2) = self.base.system_clock_switch();
        b1.set(true);
        b2.set(false);
        Ok(self)
    }
}
