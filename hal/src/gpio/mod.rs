use super::register::{Bit, MUBit, Register};

// MODER = base + 0x00
// IDR = base + 0x10
// ODR = base + 0x14

pub type GpioAddr = u32;
pub const GPIO_A: GpioAddr = 0x4002_0000;
pub const GPIO_B: GpioAddr = 0x4002_0400;
pub const GPIO_C: GpioAddr = 0x4002_0800;
pub const GPIO_D: GpioAddr = 0x4002_0C00;
pub const GPIO_E: GpioAddr = 0x4002_1000;
pub const GPIO_F: GpioAddr = 0x4002_1400;
pub const GPIO_G: GpioAddr = 0x4002_1800;
pub const GPIO_H: GpioAddr = 0x4002_1C00;
pub const GPIO_I: GpioAddr = 0x4002_2000;
pub const GPIO_J: GpioAddr = 0x4002_2400;
pub const GPIO_K: GpioAddr = 0x4002_2800;

pub struct Gpio {
    base: GpioAddr,
    bit: u32,
}

impl Gpio {
    pub fn new(periph: GpioAddr, bit: u32) -> Result<Gpio, bool> {
        if bit > 15 {
            return Err(false);
        }
        Ok(Gpio { base: periph, bit })
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

    pub fn open_drain_not_push_pull(&self) -> Bit {
        Bit::new(Register::new(self.base + 0x04), self.bit)
    }

    pub fn speed(&self) -> (Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x08), self.bit * 2 + 1),
            Bit::new(Register::new(self.base + 0x08), self.bit * 2),
        )
    }

    pub fn pull_up_pull_down(&self) -> (Bit, Bit) {
        (
            Bit::new(Register::new(self.base + 0x0C), self.bit * 2 + 1),
            Bit::new(Register::new(self.base + 0x0C), self.bit * 2),
        )
    }

    pub fn value(&self) -> MUBit {
        MUBit::new(
            Bit::new(Register::new(self.base + 0x10), self.bit),
            Bit::new(Register::new(self.base + 0x14), self.bit),
        )
    }
}
