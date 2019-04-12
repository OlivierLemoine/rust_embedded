#![no_main]
#![no_std]

extern crate HAL;
use HAL::gpio;
use HAL::nvic;
use HAL::timer;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    nvic::NVIC::new().tim2_set_enabled().set(true);

    let led = gpio::Gpio::new(gpio::GPIO_A, 5).unwrap();
    led.enabled().set(true);
    led.mode().set(true);
    led.value().set(true);

    let timer = timer::Timer::new(timer::TIMER_2);
    timer.enabled().set(true);
    timer.auto_reload_register_enabled().set(false);

    // let tmp = tim2.auto_reload_register().read();
    // tim2.auto_reload_register()
    //     .write((tmp & 0xFFFF_0000) | 0x0000_00FF);

    timer.update_interrupt_enabled().set(true);

    timer.update_generator().set(true);
    timer.count().set(true);

    loop {
        let val = timer.counter().read();
        let _val2 = val + 1;
    }
}
