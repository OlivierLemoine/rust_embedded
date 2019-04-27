#![allow(dead_code)]

pub mod raw;

pub mod alternate_function {
    pub const I2C: u8 = 4;
    pub const SPI1: u8 = 4;
    pub const SPI2: u8 = 5;
    pub const SPI3: u8 = 6;
    pub const USART1: u8 = 7;
    pub const USART2: u8 = 7;
    pub const USART3: u8 = 7;
    pub const USART4: u8 = 8;
    pub const USART5: u8 = 8;
}

pub mod states {
    pub struct Disable;
    pub struct Enable;
}

pub mod mode {
    pub struct Input;
    pub struct Output;
    pub struct Alternate;
}

pub struct Undefined;

pub struct Gpio<STATE, MODE> {
    base: raw::Gpio,
    state: STATE,
    mode: MODE,
}

impl Gpio<Undefined, Undefined> {
    pub fn new(periph: raw::GpioAddr, bit: u32) -> Gpio<states::Disable, Undefined> {
        Gpio {
            base: raw::Gpio::new(periph, bit).unwrap(),
            state: states::Disable {},
            mode: Undefined,
        }
    }

    pub fn new_user_led() -> Gpio<states::Enable, mode::Output> {
        Gpio::new(raw::GPIO_A, 5).set_active().into_output()
    }

    pub fn new_usb_serial_pins() -> (
        Gpio<states::Enable, mode::Alternate>,
        Gpio<states::Enable, mode::Alternate>,
    ) {
        (
            Gpio::new(raw::GPIO_A, 3)
                .set_active()
                .into_alternate()
                .alternate_function(alternate_function::USART2)
                .into_high_speed()
                .into_no_pull()
                .into_push_pull(),
            Gpio::new(raw::GPIO_A, 2)
                .set_active()
                .into_alternate()
                .alternate_function(alternate_function::USART2)
                .into_high_speed()
                .into_no_pull()
                .into_push_pull(),
        )
    }
}

impl Gpio<states::Disable, Undefined> {
    pub fn set_active(self) -> Gpio<states::Enable, Undefined> {
        self.base.enabled().set(true);
        Gpio {
            base: self.base,
            state: states::Enable,
            mode: Undefined,
        }
    }
}

impl Gpio<states::Enable, Undefined> {
    pub fn into_input(self) -> Gpio<states::Enable, mode::Input> {
        let (mut b1, mut b2) = self.base.mode();
        b1.set(false);
        b2.set(false);
        Gpio {
            base: self.base,
            state: states::Enable,
            mode: mode::Input,
        }
    }

    pub fn into_output(self) -> Gpio<states::Enable, mode::Output> {
        let (mut b1, mut b2) = self.base.mode();
        b1.set(false);
        b2.set(true);
        Gpio {
            base: self.base,
            state: states::Enable,
            mode: mode::Output,
        }
    }

    pub fn into_alternate(self) -> Gpio<states::Enable, mode::Alternate> {
        let (mut b1, mut b2) = self.base.mode();
        b1.set(true);
        b2.set(false);
        Gpio {
            base: self.base,
            state: states::Enable,
            mode: mode::Alternate,
        }
    }
}

impl<MODE> Gpio<states::Enable, MODE> {
    pub fn into_push_pull(self) -> Gpio<states::Enable, MODE> {
        self.base.open_drain_not_push_pull().set(false);
        self
    }

    pub fn into_open_drain(self) -> Gpio<states::Enable, MODE> {
        self.base.open_drain_not_push_pull().set(true);
        self
    }

    pub fn into_low_speed(self) -> Gpio<states::Enable, MODE> {
        let (mut b1, mut b2) = self.base.speed();
        b1.set(false);
        b2.set(false);
        self
    }

    pub fn into_medium_speed(self) -> Gpio<states::Enable, MODE> {
        let (mut b1, mut b2) = self.base.speed();
        b1.set(false);
        b2.set(true);
        self
    }

    pub fn into_fast_speed(self) -> Gpio<states::Enable, MODE> {
        let (mut b1, mut b2) = self.base.speed();
        b1.set(true);
        b2.set(false);
        self
    }

    pub fn into_high_speed(self) -> Gpio<states::Enable, MODE> {
        let (mut b1, mut b2) = self.base.speed();
        b1.set(true);
        b2.set(true);
        self
    }

    pub fn into_no_pull(self) -> Gpio<states::Enable, MODE> {
        let (mut b1, mut b2) = self.base.pull_up_pull_down();
        b1.set(false);
        b2.set(false);
        self
    }

    pub fn into_pull_up(self) -> Gpio<states::Enable, MODE> {
        let (mut b1, mut b2) = self.base.pull_up_pull_down();
        b1.set(false);
        b2.set(true);
        self
    }

    pub fn into_pull_down(self) -> Gpio<states::Enable, MODE> {
        let (mut b1, mut b2) = self.base.pull_up_pull_down();
        b1.set(true);
        b2.set(false);
        self
    }

}

impl Gpio<states::Enable, mode::Alternate> {
    pub fn alternate_function(self, function: u8) -> Gpio<states::Enable, mode::Alternate> {
        let (mut b1, mut b2, mut b3, mut b4) = self.base.alternate_function();
        b1.set((function & 0x08) == 0x08);
        b2.set((function & 0x04) == 0x04);
        b3.set((function & 0x02) == 0x02);
        b4.set((function & 0x01) == 0x01);
        self
    }
}

impl Gpio<states::Enable, mode::Input> {
    pub fn get(&self) -> bool {
        self.base.value().get()
    }
}

impl Gpio<states::Enable, mode::Output> {
    pub fn set(self, value: bool) -> Gpio<states::Enable, mode::Output> {
        self.base.value().set(value);
        self
    }
}
