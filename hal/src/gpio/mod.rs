use super::register::{Bit, MUBit, Register};

// MODER = base + 0x00
// IDR = base + 0x10
// ODR = base + 0x14

// A = 0x4002_0000
// C = 0x4002_0800

pub type GpioAddr = u32;
pub const GPIO_A: GpioAddr = 0x4002_0000;

pub struct Gpio {
    base: u32,
    bit: u32,
}

impl Gpio {
    pub fn new(periph: GpioAddr, bit: u32) -> Gpio {
        Gpio { base: periph, bit }
    }

    pub fn enabled(&self) -> Bit {
        let bit = match self.base {
            0x4002_0000 => 0,
            0x4002_0800 => 2,
            _ => 0,
        };
        Bit::new(Register::new(0x4002_3800 + 0x30), bit)
    }

    pub fn mode(&self) -> Bit {
        Bit::new(Register::new(self.base /* + 0x00*/), self.bit * 2)
    }

    pub fn value(&self) -> MUBit {
        MUBit::new(
            Bit::new(Register::new(self.base + 0x10), self.bit),
            Bit::new(Register::new(self.base + 0x14), self.bit),
        )
    }
}
