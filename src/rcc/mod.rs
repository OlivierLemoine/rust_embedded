#![allow(dead_code)]

use super::register::{Bit, Register};

// mod timer_handlers;

// pub type TimerAddr = u32;
// pub const TIMER_2: TimerAddr = 0x4000_0000;
// pub const TIMER_3: TimerAddr = 0x4000_0400;
// pub const TIMER_4: TimerAddr = 0x4000_0800;
// pub const TIMER_5: TimerAddr = 0x4000_0C00;
// pub const TIMER_6: TimerAddr = 0x4000_1000;
// pub const TIMER_7: TimerAddr = 0x4000_1400;

pub struct Rcc {
    base: u32,
}

impl Rcc {
    pub fn new() -> Rcc {
        Rcc { base: 0x4002_3800 }
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

//     pub fn auto_reload_register_enabled(&self) -> Bit {
//         Bit::new(Register::new(self.base), 7)
//     }

//     pub fn count_direction(&self) -> Bit {
//         Bit::new(Register::new(self.base), 4)
//     }

//     pub fn one_pulse_mode(&self) -> Bit {
//         Bit::new(Register::new(self.base), 3)
//     }

//     pub fn update_disabled(&self) -> Bit {
//         Bit::new(Register::new(self.base), 1)
//     }

//     pub fn count(&self) -> Bit {
//         Bit::new(Register::new(self.base), 0)
//     }

//     pub fn clock_division(&self) -> (Bit, Bit) {
//         (
//             Bit::new(Register::new(self.base), 9),
//             Bit::new(Register::new(self.base), 8),
//         )
//     }

//     pub fn trigger_interrupt_enabled(&self) -> Bit {
//         Bit::new(Register::new(self.base + 0x0C), 6)
//     }

//     pub fn update_interrupt_enabled(&self) -> Bit {
//         Bit::new(Register::new(self.base + 0x0C), 0)
//     }

//     pub fn update_interrupt_flag(&self) -> Bit {
//         Bit::new(Register::new(self.base + 0x10), 0)
//     }

//     pub fn update_generator(&self) -> Bit {
//         Bit::new(Register::new(self.base + 0x14), 0)
//     }

//     pub fn counter(&self) -> Register16 {
//         Register16::new(self.base + 0x24)
//     }

//     pub fn prescaler(&self) -> Register16 {
//         Register16::new(self.base + 0x28)
//     }

//     pub fn auto_reload_register(&self) -> Register16 {
//         Register16::new(self.base + 0x2C)
//     }
// }
