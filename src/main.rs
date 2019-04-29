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
    println!("\n");

    let mut v: Vec<u8> = Vec::new();
    v.push(0x02);

    // mmu_test();

    // timer_config();

    gpio::Gpio::new(gpio::raw::GPIO_A, 1)
        .set_active()
        .into_alternate()
        .alternate_function(gpio::alternate_function::USART4)
        .into_high_speed()
        .into_no_pull()
        .into_push_pull();
    gpio::Gpio::new(gpio::raw::GPIO_A, 0)
        .set_active()
        .into_alternate()
        .alternate_function(gpio::alternate_function::USART4)
        .into_high_speed()
        .into_no_pull()
        .into_push_pull();

    let wifi = usart::Usart::new(usart::raw::USART4)
        .set_active()
        .into_rxtx()
        .into_8_bit_message()
        .into_no_parity()
        .into_1_stop_bit()
        .set_baud_rate(115200)
        .ready_usart();

    let mut i: u32 = 0;

    wifi.n_write("AT\r\n".as_bytes());

    loop {
        // if serial.has_received_char() {
        //     let c = serial.read_char();
        //     wifi.n_put_char(c);
        // }
        if i == 100_000 {
            println!("");
            wifi.n_write("AT+GMR\r\n".as_bytes());
        }

        if i == 200_000 {
            println!("");
            wifi.n_write("AT+CWLAP\r\n".as_bytes());
        }

        if i == 400_000 {
            println!("");
            wifi.n_write("AT+CWLAP\r\nAT+CWJAP=\"Livebox-92d\",\"wifieasy\"\r\n".as_bytes());
        }

        if i == 600_000 {
            println!("");
            wifi.n_write("AT+CIFSR\r\n".as_bytes());
        }

        if i == 800_000 {
            println!("");
            wifi.n_write("AT+CIPSTART=\"TCP\",\"192.168.1.21\",8000\r\n".as_bytes());
        }

        if i == 1000_000 {
            println!("");
            wifi.n_write("AT+CIPSENDBUF=4\r\ntest\r\n".as_bytes());
        }

        if i == 1200_000 {
            println!("");
            wifi.n_write("AT+CIPCLOSE\r\n".as_bytes());
        }

        if wifi.has_received_char() {
            let c = wifi.read_char();
            serial.n_put_char(c);
        }

        i += 1;
    }
}
