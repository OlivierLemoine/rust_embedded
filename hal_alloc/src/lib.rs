#![no_std]

extern crate alloc;
extern crate allocator;
extern crate hal;
use alloc::boxed::Box;

static mut USART2_CB: Option<Box<Fn(char) -> ()>> = None;
fn usart2_handler(c: char) {
    unsafe {
        match &USART2_CB {
            Some(f) => f(c),
            None => {}
        }
    }
}

pub fn setup_usart2(cb: Box<Fn(char) -> ()>) {
    let tmp = (&(usart2_handler as *const fn(char)) as *const *const fn(char)) as *const fn(char);
    unsafe { USART2_CB = Some(cb) };
    hal::usart::Usart::new_usb_serial(115200)
        .enable_receive_interrupt()
        .set_on_received_callback(tmp)
        .ready_usart();
}