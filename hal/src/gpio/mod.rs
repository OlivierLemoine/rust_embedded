use super::register::{Bit, Register};

pub enum GpioPeriph {
    A,
    C,
}

pub struct Gpio {
    periph: GpioPeriph,

    moder: Register,
    odr: Register,
    idr: Register,

    bit: u32,
}

impl Gpio {
    pub fn new(gpio: GpioPeriph, bit: u32) -> Gpio {
        let base = match gpio {
            GpioPeriph::A => 0x4002_0000,
            GpioPeriph::C => 0x4002_0800,
        };

        Gpio {
            periph: gpio,

            moder: Register::new(base + 0x00),
            idr: Register::new(base + 0x10),
            odr: Register::new(base + 0x14),

            bit: bit,
        }
    }

    pub fn enabled(&mut self) -> Bit {
        let bit = match self.periph {
            GpioPeriph::A => 0,
            GpioPeriph::C => 2,
        };
        Bit::new(Register::new(0x4002_3800 + 0x30), bit)
    }

    pub fn mode(&mut self) -> Bit {
        Bit::new(self.moder.copy(), self.bit * 2)
    }

    pub fn set_bit(&mut self, val: bool) {
        let tmp = self.odr.read();

        if val {
            self.odr.write(tmp | (1 << self.bit));
        } else {
            self.odr.write(tmp & !(1 << self.bit));
        }
    }

    pub fn get_bit(&mut self) -> bool {
        let tmp = self.idr.read();
        (tmp & (1 << self.bit)) != 0
    }
}
