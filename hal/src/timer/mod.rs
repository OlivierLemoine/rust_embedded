use super::register::{Bit, Register, Register16};

pub type TimerAddr = u32;
pub const TIMER_2: TimerAddr = 0x4000_0000;

// CR1 = 0x00
// DIER = 0x0C
// EGR = 0x14
// CNT = 0x24
// PSC = 0x28
// ARR = 0x2C

pub struct Timer {
    base: u32,
}

impl Timer {
    pub fn new(periph: TimerAddr) -> Timer {
        Timer { base: periph }
    }

    pub fn enabled(&self) -> Bit {
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

    pub fn counter(&self) -> Register16 {
        Register16::new(self.base + 0x24)
    }

    pub fn prescaler(&self) -> Register16 {
        Register16::new(self.base + 0x28)
    }

    pub fn auto_reload_register(&self) -> Register16 {
        Register16::new(self.base + 0x2C)
    }
}
