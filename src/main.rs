#![no_main]
#![no_std]

#![feature(asm)]

// extern crate HAL;
// use HAL::gpio;
// use HAL::nvic;
// use HAL::timer;
// use HAL::usart;

mod gpio;
mod nvic;
mod register;
mod timer;
mod usart;

mod panic_handler;

mod kernel;

fn timer_config() {
    let timer = timer::Timer::new(timer::TIMER_2);
    timer.enabled().set(true);

    timer.auto_reload_register_enabled().set(false);
    timer.auto_reload_register().write(0xFFFF);

    timer.prescaler().write(0xF);

    timer.clock_division().1.set(true);

    timer.update_interrupt_enabled().set(true);

    timer.update_generator().set(true);

    timer.count().set(true);
}

#[no_mangle]
pub unsafe extern "C" fn SVC_Handler() {
    // loop {}
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    nvic::NVIC::new().tim2_set_enabled().set(true);

    asm!("swi 0");

    let led = gpio::Gpio::new(gpio::GPIO_A, 5).unwrap();
    led.enabled().set(true);
    led.mode().1.set(true);
    led.value().set(true);

    let v = kernel::alloc::malloc(10).unwrap();

    // let rx = gpio::Gpio::new(gpio::GPIO_A, 3).unwrap();
    // let tx = gpio::Gpio::new(gpio::GPIO_A, 2).unwrap();

    // rx.enabled().set(true);
    // rx.mode().0.set(true);

    // tx.enabled().set(true);
    // tx.mode().0.set(true);

    // for i in [usart::USART2, usart::USART3, usart::USART4, usart::USART5].iter() {
    //     let uart = usart::Usart::new(*i);
    //     uart.enabled().set(true);
    //     uart.transmiter_enabled().set(true);
    //     uart.data().write(65);
    // }

    timer_config();

    loop {}
}
