#![allow(dead_code)]

use super::register::{Bit, Register, Register16, Register8};

pub type UsartAddr = u32;
pub const USART2: UsartAddr = 0x4000_4400;
pub const USART3: UsartAddr = 0x4000_4800;
pub const USART4: UsartAddr = 0x4000_4C00;
pub const USART5: UsartAddr = 0x4000_5000;

pub struct Usart {
    base: UsartAddr,
}

impl Usart {
    pub fn new(periph: UsartAddr) -> Usart {
        Usart { base: periph }
    }

    pub fn enabled(&self) -> Bit {
        let bit = match self.base {
            0x4000_4400 => 17,
            0x4000_4800 => 18,
            0x4000_4C00 => 19,
            0x4000_5000 => 20,
            _ => 0,
        };
        Bit::new(Register::new(0x4002_3800 + 0x40), bit)
    }

    pub fn transmit_data_register_empty(&self) -> Bit {
        Bit::new(Register::new(self.base), 7)
    }

    pub fn transmission_complete(&self) -> Bit {
        Bit::new(Register::new(self.base), 6)
    }

    pub fn read_data_register_not_empty(&self) -> Bit {
        Bit::new(Register::new(self.base), 5)
    }

    pub fn data(&self) -> Register8 {
        Register8::new(self.base + 0x04)
    }

    pub fn baud_rate(&self) -> Register16 {
        Register16::new(self.base + 0x08)
    }

    pub fn usart_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 13)
    }

    pub fn parity_control_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 10)
    }

    pub fn parity_odd_not_even(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 9)
    }

    pub fn transmit_data_register_empty_interrupt_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 7)
    }

    pub fn transmission_complete_interrupt_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 6)
    }
    pub fn read_data_register_not_empty_interrupt_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 5)
    }

    pub fn transmiter_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 3)
    }

    pub fn receiver_enabled(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 2)
    }

    pub fn send_break_caracter(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x0C), 0)
    }

    pub fn stop_bit(&self) -> (Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x10), 13),
            Bit::new(Register::new(self.base + 0x10), 12),
        )
    }

    pub fn guard_time_and_prescaler(&self) -> Register16 {
        Register16::new(self.base + 0x18)
    }
}
