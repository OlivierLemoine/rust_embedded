#![allow(dead_code)]

#[no_mangle]
pub unsafe extern "C" fn TIM2_IRQHandler() {
    let mut b = super::super::gpio::raw::Gpio::new(super::super::gpio::raw::GPIO_A, 5)
        .unwrap()
        .value();
    if b.get() {
        b.set(false);
    } else {
        b.set(true);
    }

    super::raw::Timer::new(super::raw::TIMER_2)
        .update_interrupt_flag()
        .set(false);
}

#[no_mangle]
pub unsafe extern "C" fn TIM3_IRQHandler() {
    super::raw::Timer::new(super::raw::TIMER_3)
        .update_interrupt_flag()
        .set(false);
}

#[no_mangle]
pub unsafe extern "C" fn TIM4_IRQHandler() {
    super::raw::Timer::new(super::raw::TIMER_4)
        .update_interrupt_flag()
        .set(false);
}
