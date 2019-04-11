use super::super::register::{Bit, Register};

pub enum TimerPeriph {
    _2,
}

pub struct TimerConfig {
    periph: TimerPeriph,

    cr1: Register,
    egr: Register,
    cnt: Register,
    psc: Register,
}

impl TimerConfig {
    pub fn new(timer: TimerPeriph) -> TimerConfig {
        let base = match timer {
            TimerPeriph::_2 => 0x4000_0000,
        };

        TimerConfig {
            periph: timer,

            cr1: Register::new(base + 0x00),
            egr: Register::new(base + 0x14),
            cnt: Register::new(base + 0x24),
            psc: Register::new(base + 0x28),
        }
    }

    pub fn enable(&mut self) {
        let mut rcc = Register::new(0x4002_3800 + 0x40);

        let mut temp = rcc.read();
        temp |= match self.periph {
            TimerPeriph::_2 => 0x01,
        };
        rcc.write(temp);
    }

    pub fn reset_and_update(&mut self) -> Bit {
        Bit::new(self.egr.copy(), 0)
    }

    pub fn count(&mut self) -> Bit {
        Bit::new(self.cr1.copy(), 0)
    }

    pub fn counter (&mut self) -> Register {
        self.cnt.copy()
    }

    pub fn prescaler (&mut self) -> Register {
        self.psc.copy()
    }
}
