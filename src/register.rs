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
        unsafe { read_volatile(self.r as *const u32) }
    }
}

pub struct Register16 {
    r: u32,
}

impl Register16 {
    pub fn new(addr: u32) -> Register16 {
        Register16 { r: addr }
    }

    pub fn copy(&self) -> Register16 {
        Register16 { r: self.r }
    }

    pub fn write(&mut self, val: u16) {
        unsafe { write_volatile(self.r as *mut u32, val as u32) };
    }

    pub fn read(&self) -> u16 {
        unsafe { read_volatile(self.r as *const u32) as u16 }
    }
}

pub struct Register8 {
    r: u32,
}

impl Register8 {
    pub fn new(addr: u32) -> Register8 {
        Register8 { r: addr }
    }

    pub fn copy(&self) -> Register8 {
        Register8 { r: self.r }
    }

    pub fn write(&mut self, val: u8) {
        unsafe { write_volatile(self.r as *mut u32, val as u32) };
    }

    pub fn read(&self) -> u8 {
        unsafe { read_volatile(self.r as *const u32) as u8 }
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

    pub fn get(&self) -> bool {
        let tmp = self.r.read();
        (tmp & (1 << self.b)) != 0
    }
}

pub struct MUBit {
    //Multiple register Bit
    r: Bit,
    w: Bit,
}

impl MUBit {
    pub fn new(r: Bit, w: Bit) -> MUBit {
        MUBit { r, w }
    }

    pub fn set(&mut self, value: bool) {
        self.w.set(value);
    }

    pub fn get(&self) -> bool {
        self.r.get()
    }
}
