#![no_main]
#![no_std]

extern crate alloc;
extern crate allocator;
#[macro_use]
extern crate hal;
extern crate kernel;

mod panic_handler;

use alloc::string::String;

#[no_mangle]
pub unsafe extern "C" fn main() {
    hal::init();
    allocator::init();
    kernel::init();
    println!("\n");

    // kernel::net::wifi::connect(String::from("Livebox-092d"), String::from("wifieasy"));

    // let socket = kernel::net::tcp::Tcp::new();
    // socket.connect(String::from("192.168.1.21"), String::from("8000"));

    // let serial = hal::usart::Usart::reopen_com(hal::usart::raw::USART2);

    loop {
        // if serial.has_received_char() {
        //     serial.put_char(serial.read_char());
        // }
    }
}
