#![no_main]
#![no_std]

extern crate HAL;
use HAL::gpio;
use HAL::nvic;
use HAL::timer;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    let mut nvic = nvic::NVIC::new();
    nvic.tim2_set_enabled().set(true);

    let mut gpio = gpio::Gpio::new(gpio::GpioPeriph::A, 5);
    gpio.enabled().set(true);
    gpio.mode().set(true);
    gpio.set_bit(true);

    let mut tim2 = timer::Timer::new(timer::TimerPeriph::_2);
    tim2.enable();

    tim2.auto_reload_register_enabled().set(true);

    let tmp = tim2.auto_reload_register().read();
    tim2.auto_reload_register()
        .write((tmp & 0xFFFF_0000) | 0x0000_00FF);

    tim2.trigger_interrupt_enabled().set(true);
    tim2.update_interrupt_enabled().set(true);

    tim2.update_generator().set(true);
    tim2.count().set(true);

    loop {
        let val = tim2.counter().read() & 0x0000_FFFF;
        let val2 = val + 1;
    }
}
