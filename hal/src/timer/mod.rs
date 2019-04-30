#![allow(dead_code)]

pub mod raw;
mod timer_handlers;

pub mod counter {
    pub struct On;
    pub struct Off;
}

pub mod states {
    pub struct Disable;
    pub struct Enable;
}

pub struct Undefined;

pub struct Timer<STATE, COUNTER> {
    base: raw::Timer,

    state: STATE,
    counter: COUNTER,
}

impl Timer<states::Disable, Undefined> {
    pub fn new(periph: raw::TimerAddr) -> Timer<states::Disable, Undefined> {
        Timer {
            base: raw::Timer::new(periph),

            state: states::Disable,
            counter: Undefined,
        }
    }

    pub fn disable_update_interrupt_flag(periph: raw::TimerAddr) {
        raw::Timer::new(periph).update_interrupt_flag().set(false);
    }
}

impl Timer<states::Disable, Undefined> {
    pub fn enable(self) -> Timer<states::Enable, counter::Off> {
        self.base.enabled();
        Timer {
            base: self.base,

            state: states::Enable,
            counter: counter::Off,
        }
    }
}

impl Timer<states::Enable, counter::Off> {
    pub fn start_count(self) -> Timer<states::Enable, counter::On> {
        self.base.count().set(true);
        Timer {
            base: self.base,

            state: states::Enable,
            counter: counter::On,
        }
    }

    pub fn into_one_pulse_mode(self) -> Timer<states::Enable, counter::Off> {
        self.base.one_pulse_mode().set(true);
        self
    }

    pub fn into_multiple_pulse_mode(self) -> Timer<states::Enable, counter::Off> {
        self.base.one_pulse_mode().set(false);
        self
    }

    pub fn count_upward(self) -> Timer<states::Enable, counter::Off> {
        self.base.count_direction().set(false);
        self
    }

    pub fn count_downward(self) -> Timer<states::Enable, counter::Off> {
        self.base.count_direction().set(true);
        self
    }

    pub fn enable_auto_reload_register(self) -> Timer<states::Enable, counter::Off> {
        self.base.auto_reload_register_enabled().set(true);
        self
    }

    pub fn disable_auto_reload_register(self) -> Timer<states::Enable, counter::Off> {
        self.base.auto_reload_register_enabled().set(false);
        self
    }

    pub fn into_clock_div_by_1(self) -> Timer<states::Enable, counter::Off> {
        let (mut b1, mut b2) = self.base.clock_division();
        b1.set(false);
        b2.set(false);
        self
    }

    pub fn into_clock_div_by_2(self) -> Timer<states::Enable, counter::Off> {
        let (mut b1, mut b2) = self.base.clock_division();
        b1.set(false);
        b2.set(true);
        self
    }

    pub fn into_clock_div_by_4(self) -> Timer<states::Enable, counter::Off> {
        let (mut b1, mut b2) = self.base.clock_division();
        b1.set(true);
        b2.set(false);
        self
    }

    pub fn enable_update_interrupt(self) -> Timer<states::Enable, counter::Off> {
        self.base.update_interrupt_enabled().set(true);
        self
    }
}

impl<COUNTER> Timer<states::Enable, COUNTER> {
    pub fn set_prescaler(self, p: u16) -> Timer<states::Enable, COUNTER> {
        self.base.prescaler().write(p);
        self
    }

    pub fn set_auto_reload_register(self, a: u16) -> Timer<states::Enable, COUNTER> {
        self.base.auto_reload_register().write(a);
        self
    }

    pub fn reset(self) -> Timer<states::Enable, COUNTER> {
        self.base.update_generator().set(true);
        self
    }

    pub fn counter_value(&self) -> u16 {
        self.base.counter().read()
    }
}

impl Timer<states::Enable, counter::On> {
    pub fn stop_count(self) -> Timer<states::Enable, counter::Off>{
        self.base.count().set(false);
        Timer {
            base: self.base,
            state: states::Enable,
            counter: counter::Off,
        }
    }
}