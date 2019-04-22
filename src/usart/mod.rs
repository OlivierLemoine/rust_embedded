#![allow(dead_code)]

pub mod raw;

use super::gpio;

pub mod states {
    pub struct Disable;
    pub struct Enable;
}

pub mod mode {
    pub struct Rx;
    pub struct Tx;
    pub struct RxTx;
}

pub mod usart_state {
    pub struct Ready;
    pub struct Waiting;
}

pub struct Undefined;

pub struct Usart<STATE, MODE, USARTSTATE> {
    base: raw::Usart,
    state: STATE,
    mode: MODE,
    usart_state: USARTSTATE,
}

impl Usart<Undefined, Undefined, Undefined> {
    pub fn new(periph: raw::UsartAddr) -> Usart<states::Disable, Undefined, Undefined> {
        Usart {
            base: raw::Usart::new(periph),
            state: states::Disable,
            mode: Undefined,
            usart_state: Undefined,
        }
    }

    pub fn new_usb_serial() -> Usart<states::Enable, mode::RxTx, usart_state::Ready> {
        gpio::Gpio::new_usb_serial_pins();
        Usart::new(raw::USART2)
            .set_active()
            .into_rxtx()
            .into_1_stop_bit()
            .into_no_parity()
            .set_baud_rate(9600)
            .ready_usart()
    }
}

impl Usart<states::Disable, Undefined, Undefined> {
    pub fn set_active(self) -> Usart<states::Enable, Undefined, usart_state::Waiting> {
        self.base.enabled();
        Usart {
            base: self.base,
            state: states::Enable,
            mode: Undefined,
            usart_state: usart_state::Waiting,
        }
    }
}

impl<MODE> Usart<states::Enable, MODE, usart_state::Waiting> {
    pub fn ready_usart(self) -> Usart<states::Enable, MODE, usart_state::Ready> {
        self.base.usart_enabled();
        Usart {
            base: self.base,
            state: states::Enable,
            mode: self.mode,
            usart_state: usart_state::Ready,
        }
    }

    pub fn into_no_parity(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        self.base.parity_control_enabled().set(false);
        self
    }

    pub fn into_even_parity(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        self.base.parity_control_enabled().set(true);
        self.base.parity_odd_not_even().set(false);
        self
    }

    pub fn into_odd_parity(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        self.base.parity_control_enabled().set(true);
        self.base.parity_odd_not_even().set(true);
        self
    }

    pub fn into_1_stop_bit(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        let (mut b1, mut b2) = self.base.stop_bit();
        b1.set(false);
        b2.set(false);
        self
    }

    pub fn into_0_5_stop_bit(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        let (mut b1, mut b2) = self.base.stop_bit();
        b1.set(false);
        b2.set(true);
        self
    }

    pub fn into_2_stop_bit(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        let (mut b1, mut b2) = self.base.stop_bit();
        b1.set(true);
        b2.set(false);
        self
    }

    pub fn into_1_5_stop_bit(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        let (mut b1, mut b2) = self.base.stop_bit();
        b1.set(true);
        b2.set(true);
        self
    }

    pub fn set_baud_rate(self, baud: u32) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        let v = match baud {
            9600 => 0x683,
            _ => 0x0,
        };
        self.base.baud_rate().write(v);
        self
    }
}

impl Usart<states::Enable, Undefined, usart_state::Waiting> {
    pub fn into_rxtx(self) -> Usart<states::Enable, mode::RxTx, usart_state::Waiting> {
        self.base.transmiter_enabled().set(true);
        self.base.receiver_enabled().set(true);
        Usart {
            base: self.base,
            state: states::Enable,
            mode: mode::RxTx,
            usart_state: usart_state::Waiting,
        }
    }

    pub fn into_rx(self) -> Usart<states::Enable, mode::Rx, usart_state::Waiting> {
        self.base.transmiter_enabled().set(false);
        self.base.receiver_enabled().set(true);
        Usart {
            base: self.base,
            state: states::Enable,
            mode: mode::Rx,
            usart_state: usart_state::Waiting,
        }
    }

    pub fn into_tx(self) -> Usart<states::Enable, mode::Tx, usart_state::Waiting> {
        self.base.transmiter_enabled().set(true);
        self.base.receiver_enabled().set(false);
        Usart {
            base: self.base,
            state: states::Enable,
            mode: mode::Tx,
            usart_state: usart_state::Waiting,
        }
    }
}

impl Usart<states::Enable, mode::Tx, usart_state::Ready> {
    pub fn put_char(self, c: u8) -> Usart<states::Enable, mode::Tx, usart_state::Ready> {
        self.base.data().write(c);
        while !self.base.transmit_data_register_empty().get() {}
        self
    }

    pub fn write(mut self, s: &str) -> Usart<states::Enable, mode::Tx, usart_state::Ready> {
        // let tmp = &self;
        for c in s.bytes() {
            self = self.put_char(c);
        }
        self
    }
}

impl Usart<states::Enable, mode::RxTx, usart_state::Ready> {
    pub fn put_char(self, c: u8) -> Usart<states::Enable, mode::RxTx, usart_state::Ready> {
        self.base.data().write(c);
        while !self.base.transmit_data_register_empty().get() {}
        self
    }

    pub fn write(mut self, s: &str) -> Usart<states::Enable, mode::RxTx, usart_state::Ready> {
        // let tmp = &self;
        for c in s.bytes() {
            self = self.put_char(c);
        }
        self
    }
}
