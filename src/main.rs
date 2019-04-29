#![no_main]
#![no_std]

mod gpio;
mod nvic;
mod rcc;
mod register;
mod timer;
#[macro_use]
mod usart;
mod mmu;

mod kernel;

mod panic_handler;

use kernel::vec::Vec;
use kernel::string::String;

fn timer_config() {
    timer::Timer::new(timer::raw::TIMER_2)
        .enable()
        .enable_auto_reload_register()
        .set_auto_reload_register(0xFFFF)
        .set_prescaler(0xF)
        .count_upward()
        .into_clock_div_by_4()
        .reset()
        .enable_update_interrupt()
        .start_count();
}

fn mmu_test() {
    let mmu = mmu::Mmu::new();

    print_hexa!(mmu.type_reg().read());
    println!("");

    print_hexa!(register::Register::new(0xE000_ED90 + 0x04).read());
    println!("");

    // let a = String::from_str("").
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    rcc::Rcc::new().enable_hsi().sysclock_into_hsi();
    let serial = usart::Usart::new_usb_serial(115200);
    kernel::net::init();
    println!("\n");

    // mmu_test();

    // timer_config();

    // kernel::net

    let wifi = usart::Usart::__com(usart::raw::USART4);

    let mut i: u32 = 0;

    let tcp = kernel::net::tcp::Tcp::new();

    wifi.write("AT+CWJAP=\"Livebox-092d\",\"wifieasy\"\r\n".as_bytes());

    loop {
        // if serial.has_received_char() {
        //     let c = serial.read_char();
        //     wifi.n_put_char(c);
        // }

        if i == 400_000 {
            println!("");
            wifi.write("AT+CIFSR\r\n".as_bytes());
        }

        if i == 600_000 {
            println!("");
            tcp.connect(String::from_str("192.168.1.21"), String::from_str("8000"))
            // wifi.write("AT+CIPSTART=\"TCP\",\"192.168.1.21\",8000\r\n".as_bytes());
        }

        if wifi.has_received_char() {
            let c = wifi.read_char();
            serial.put_char(c);
        }

        i += 1;
    }
}
