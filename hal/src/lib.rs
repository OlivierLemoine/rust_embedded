#![no_std]

extern crate allocator;
extern crate alloc;


pub mod gpio;
pub mod nvic;
pub mod rcc;
pub mod register;
pub mod timer;
#[macro_use]
pub mod usart;
pub mod mmu;

pub fn init(){
    usart::Usart::new_usb_serial(115200);
}