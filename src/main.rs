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

mod kernel;

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
    usart::Usart::new_usb_serial();
    
    timer_config();

    println!("Test");
}
