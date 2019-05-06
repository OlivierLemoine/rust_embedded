#![allow(dead_code)]

// use super::super::register::{Bit, Register};

pub struct I2C {
    base: u32,
}

impl I2C {
    pub fn new() -> I2C {
        I2C { base: 0 }
    }
}