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
mod rcc;
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
    let rcc = rcc::Rcc::new();
    rcc.hsi_on_not_off().set(true);
    rcc.system_clock_switch().0.set(false);
    rcc.system_clock_switch().1.set(false);

    // register::Register::new(0x4002_3800 + 0x04).write(0x2741_2A04);
    // register::Register::new(0x4002_3800 + 0x08).write(0x0008_1000);
    // register::Bit::new(register::Register::new(0x4002_3800), 16).set(true);
    // register::Bit::new(register::Register::new(0x4002_3800), 24).set(true);
    // register::Bit::new(register::Register::new(0x4002_3800 + 0x08), 2).set(true);

    // kernel::alloc::alloc_init();

    // nvic::NVIC::new().tim2_set_enabled().set(true);

    // timer_config();

    // asm!("swi 0");

    let (rx, tx) = gpio::Gpio::new_usb_serial_pins();

    let serial = usart::Usart::new(usart::USART2);
    serial.enabled().set(true);
    serial.transmiter_enabled().set(true);
    serial.receiver_enabled().set(true);
    serial.oversampling_8_not_16().set(false);
    serial.parity_control_enabled().set(false);
    serial.word_length_9_not_8().set(false);
    serial.stop_bit().0.set(false);
    serial.stop_bit().1.set(false);
    // serial.baud_rate().write(0x341);
    serial.baud_rate().write(0x683);
    serial.usart_enabled().set(true);

    serial.data().write(b'U');

    loop {
        while !serial.transmission_complete().get() {}
        serial.data().write(b'U');
    }
}
