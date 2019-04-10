#![no_main]
#![no_std]

extern crate startup;
use startup::gpio;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    let gpio = gpio::Gpio::new(gpio::gpio_config::GpioPeriph::A, 5);

    gpio.into_output().write(true);

    loop {}
}
