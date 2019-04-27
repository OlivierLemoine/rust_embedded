#![no_main]
#![no_std]

mod gpio;
mod nvic;
mod rcc;
mod register;
mod timer;
#[macro_use]
mod usart;

mod panic_handler;

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

#[no_mangle]
pub unsafe extern "C" fn main() {
    rcc::Rcc::new().enable_hsi().sysclock_into_hsi();
    let serial = usart::Usart::new_usb_serial(9600);
    println!("\n");

    timer_config();

    println!("Test");

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

    wifi.n_write("AT\r\n\r\n".as_bytes());

    loop {
        if serial.has_received_char() {
            let c = serial.read_char();
            wifi.n_put_char(c);
        }
        if wifi.has_received_char() {
            let c = wifi.read_char();
            serial.n_put_char(c);
        }
    }
}
