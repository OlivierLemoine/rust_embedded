use super::register::{Bit, Register};

pub enum TimerPeriph {
    _2,
}

pub struct Timer {
    periph: TimerPeriph,

    cr1: Register,
    dier: Register,
    egr: Register,
    cnt: Register,
    psc: Register,
    arr: Register,
}

impl Timer {
    pub fn new(timer: TimerPeriph) -> Timer {
        let base = match timer {
            TimerPeriph::_2 => 0x4000_0000,
        };

        Timer {
            periph: timer,

            cr1: Register::new(base + 0x00),
            dier: Register::new(base + 0x0C),
            egr: Register::new(base + 0x14),
            cnt: Register::new(base + 0x24),
            psc: Register::new(base + 0x28),
            arr: Register::new(base + 0x2C),
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

    pub fn update_generator(&mut self) -> Bit {
        Bit::new(self.egr.copy(), 0)
    }

    pub fn count(&mut self) -> Bit {
        Bit::new(self.cr1.copy(), 0)
    }

    pub fn auto_reload_register_enabled(&mut self) -> Bit {
        Bit::new(self.cr1.copy(), 7)
    }

    pub fn trigger_interrupt_enabled(&mut self) -> Bit {
        Bit::new(self.dier.copy(), 6)
    }

    pub fn update_interrupt_enabled(&mut self) -> Bit {
        Bit::new(self.dier.copy(), 0)
    }

    pub fn counter(&mut self) -> Register {
        self.cnt.copy()
    }

    pub fn prescaler(&mut self) -> Register {
        self.psc.copy()
    }

    pub fn auto_reload_register(&mut self) -> Register {
        self.arr.copy()
    }
}
