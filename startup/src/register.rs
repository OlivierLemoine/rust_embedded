use core::ptr::{read_volatile, write_volatile};

pub struct Register {
    r: u32,
}

impl Register {
    pub fn new(addr: u32) -> Register {
        Register { r: addr }
    }

    pub fn copy(&self) -> Register {
        Register { r: self.r }
    }

    pub fn write(&mut self, val: u32) {
        unsafe { write_volatile(self.r as *mut u32, val) };
    }

    pub fn read(&self) -> u32 {
        unsafe {
            return read_volatile(self.r as *const u32);
        }
    }
}

pub struct Bit {
    r: Register,
    b: u32,
}

impl Bit {
    pub fn new(r: Register, b: u32) -> Bit {
        Bit { r, b }
    }

    pub fn set(&mut self, value: bool) {
        let tmp = self.r.read();
        if value {
            self.r.write(tmp | (1 << self.b));
        } else {
            self.r.write(tmp & !(1 << self.b));
        }
    }

    pub fn get(&mut self) -> bool {
        let tmp = self.r.read();
        (tmp & (1 << self.b)) != 0
    }
}
