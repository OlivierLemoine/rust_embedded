// use super::super::register::{Bit, Register};

#[no_mangle]
pub unsafe extern "C" fn TIM2_IRQHandler() {
    let mut b = super::super::gpio::Gpio::new(super::super::gpio::GPIO_A, 5)
        .unwrap()
        .value();
    if b.get() {
        b.set(false);
    } else {
        b.set(true);
    }

    super::Timer::new(super::TIMER_2)
        .update_interrupt_flag()
        .set(false);
}

#[no_mangle]
pub unsafe extern "C" fn TIM3_IRQHandler() {
    super::Timer::new(super::TIMER_3)
        .update_interrupt_flag()
        .set(false);
}

#[no_mangle]
pub unsafe extern "C" fn TIM4_IRQHandler() {
    super::Timer::new(super::TIMER_4)
        .update_interrupt_flag()
        .set(false);
}
