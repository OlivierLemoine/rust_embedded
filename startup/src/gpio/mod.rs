pub mod gpio_config;

use super::register::Bit;

pub mod mode {
    pub struct Input;
    pub struct Output;
}

pub mod state {
    pub struct Enable;
    pub struct Disable;
}

pub struct DontCare;

pub struct Gpio<STATE, MODE> {
    periph: gpio_config::GpioConfig,
    state: STATE,
    mode: MODE,
}

impl Gpio<state::Disable, DontCare> {
    pub fn new(gpio: gpio_config::GpioPeriph, bit: u32) -> Gpio<state::Disable, DontCare> {
        Gpio {
            periph: gpio_config::GpioConfig::new(gpio, bit),
            state: state::Disable,
            mode: DontCare,
        }
    }

    pub fn into_input(mut self) -> Gpio<state::Enable, mode::Input> {
        self.periph.enable();
        let (mut bit1, mut bit2) = self.periph.mode();
        bit1.set(false);
        bit2.set(false);

        Gpio {
            periph: self.periph,
            state: state::Enable,
            mode: mode::Input,
        }
    }

    pub fn into_output(mut self) -> Gpio<state::Enable, mode::Output> {
        self.periph.enable();
        let (mut bit1, mut bit2) = self.periph.mode();
        bit1.set(false);
        bit2.set(true);

        Gpio {
            periph: self.periph,
            state: state::Enable,
            mode: mode::Output,
        }
    }
}

impl Gpio<state::Enable, mode::Output> {
    pub fn write(&mut self, val: bool) {
        self.periph.set_bit(val);
    }
}

impl Gpio<state::Enable, mode::Input> {
    pub fn read(&mut self) -> bool{
        self.periph.get_bit()
    }
}