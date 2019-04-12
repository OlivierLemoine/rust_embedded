use super::register::{Bit, Register};

pub enum TimerPeriph {
    _2,
}

// CR1 = 0x00
// DIER = 0x0C
// EGR = 0x14
// CNT = 0x24
// PSC = 0x28
// ARR = 0x2C

pub struct Timer {
    periph: TimerPeriph,
    base: u32,
}

impl Timer {
    pub fn new(timer: TimerPeriph) -> Timer {
        let base = match timer {
            TimerPeriph::_2 => 0x4000_0000,
        };

        Timer {
            periph: timer,

            base,
        }
    }

    pub fn enabled(&mut self) -> Bit {
        Bit::new(Register::new(0x4002_3800 + 0x40), 0)
    }

    pub fn update_generator(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x14), 0)
    }

    pub fn count(&self) -> Bit {
        Bit::new(Register::new(self.base /* + 0 */), 0)
    }

    pub fn auto_reload_register_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base /* + 0 */), 7)
    }

    pub fn trigger_interrupt_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 6)
    }

    pub fn update_interrupt_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 0)
    }

    pub fn counter(&self) -> Register {
        Register::new(self.base + 0x24)
    }

    pub fn prescaler(&self) -> Register {
        Register::new(self.base + 0x28)
    }

    pub fn auto_reload_register(&self) -> Register {
        Register::new(self.base + 0x2C)
    }
}
