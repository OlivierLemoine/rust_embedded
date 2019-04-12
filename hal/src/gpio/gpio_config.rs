use super::super::register::{Bit, Register};

pub enum GpioPeriph {
    A,
    C,
}

pub struct GpioConfig {
    periph: GpioPeriph,

    moder: Register,
    odr: Register,
    idr: Register,

    bit: u32,
}

impl GpioConfig {
    pub fn new(gpio: GpioPeriph, bit: u32) -> GpioConfig {
        let base = match gpio {
            GpioPeriph::A => 0x4002_0000,
            GpioPeriph::C => 0x4002_0800,
        };

        GpioConfig {
            periph: gpio,

            moder: Register::new(base + 0x00),
            idr: Register::new(base + 0x10),
            odr: Register::new(base + 0x14),

            bit: bit,
        }
    }

    pub fn enable(&mut self) {
        let mut rcc = Register::new(0x4002_3800 + 0x30);

        let mut temp = rcc.read();
        temp |= match self.periph {
            GpioPeriph::A => 0x01,
            GpioPeriph::C => 0x04,
        };
        rcc.write(temp);
    }

    pub fn mode(&mut self) -> (Bit, Bit) {
        (
            Bit::new(self.moder.copy(), self.bit * 2 + 1),
            Bit::new(self.moder.copy(), self.bit * 2),
        )
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
