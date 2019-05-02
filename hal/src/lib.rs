#![no_std]

// #![feature(impl_trait_in_bindings)]

pub mod gpio;
pub mod nvic;
pub mod rcc;
pub mod register;
pub mod timer;
#[macro_use]
pub mod usart;
pub mod mmu;

// pub fn init() {
//     rcc::Rcc::new().enable_hsi().sysclock_into_hsi();
//     let mut n = nvic::NVIC::new();
//     n.tim2_set_enabled().set(true);
// }

pub fn delay(ms: u32) {
    let mut t = timer::Timer::new(timer::raw::TIMER_7)
        .enable()
        .count_downward()
        .into_clock_div_by_1()
        .set_prescaler(16_000)
        .into_one_pulse_mode()
        .reset()
        .start_count();
    for _ in 0..ms {
        t = t.reset();
        while t.counter_value() > 0 {}
    }
}