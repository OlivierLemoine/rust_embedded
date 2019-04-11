#![no_main]
#![no_std]

extern crate startup;
use startup::gpio;
use startup::timer;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    let gpio = gpio::Gpio::new(gpio::gpio_config::GpioPeriph::A, 5);

    gpio.into_output().write(true);

    let mut tim2 = timer::timer_config::TimerConfig::new(timer::timer_config::TimerPeriph::_2);
    tim2.enable();
    tim2.count().set(true);

    loop {
        let val = tim2.counter().read();
        let val2 = val + 1;
    }
}
