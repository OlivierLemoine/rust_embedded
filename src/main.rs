#![no_main]
#![no_std]

extern crate alloc;
extern crate allocator;
#[macro_use]
extern crate hal;
extern crate kernel;

mod panic_handler;

#[no_mangle]
pub unsafe extern "C" fn main() {
    hal::init();
    println!("\n");

    // let socket = kernel::net::tcp::Tcp::new();
    // socket.connect(String::from("192.168.1.21"), String::from("8000"));

    let mut i: u32 = 0;

    // wifi.write("AT+CWJAP=\"Livebox-092d\",\"wifieasy\"\r\n".as_bytes());

    loop {
        // if serial.has_received_char() {
        //     let c = serial.read_char();
        //     wifi.n_put_char(c);
        // }

        if i == 400_000 {
            println!("");
            // wifi.write("AT+CIFSR\r\n".as_bytes());
        }

        if i == 600_000 {
            println!("");
            // wifi.write("AT+CIPSTART=\"TCP\",\"192.168.1.21\",8000\r\n".as_bytes());
        }

        // if wifi.has_received_char() {
        // let c = wifi.read_char();
        // serial.put_char(c);
        // }

        i += 1;
    }
}
