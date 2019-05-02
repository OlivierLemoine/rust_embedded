#![allow(dead_code)]

#[macro_export]
macro_rules! print_char {
    ($input:expr) => {
        hal::usart::Usart::reopen_com(hal::usart::raw::USART2).put_char($input as u8)
    };
}

#[macro_export]
macro_rules! print {
    ($input:expr) => {
        hal::usart::Usart::reopen_com(hal::usart::raw::USART2).write($input.as_bytes())
    };
}

#[macro_export]
macro_rules! println {
    ($input:expr) => {
        let u = hal::usart::Usart::reopen_com(hal::usart::raw::USART2);
        u.write($input.as_bytes());
        u.put_char(b'\n')
    };
}

#[macro_export]
macro_rules! print_u32 {
    ($input:expr) => {
        let u = hal::usart::raw::Usart::new(hal::usart::raw::USART2);
        let mut res = $input;

        let mut res_arr: [u8; 10] = [0; 10];

        for i in (0..10).rev() {
            res_arr[i] = (res % 10) as u8;
            res = res / 10;
        }

        for i in res_arr.iter() {
            u.data().write(48 + i);
            while !u.transmission_complete().get() {}
        }
    };
}

#[macro_export]
macro_rules! print_hexa {
    ($input:expr) => {
        let u = hal::usart::raw::Usart::new(hal::usart::raw::USART2);
        let mut res = $input;

        let mut res_arr: [u8; 8] = [0; 8];

        for i in (0..8).rev() {
            res_arr[i] = (res % 16) as u8;
            res = res / 16;
        }

        u.data().write(b'0');
        while !u.transmission_complete().get() {}

        u.data().write(b'x');
        while !u.transmission_complete().get() {}

        for i in res_arr.iter() {
            u.data().write(i + if *i >= 10 { 55 } else { 48 });
            while !u.transmission_complete().get() {}
        }
    };
}

pub mod raw;
pub mod usart_handlers;

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

    pub fn new_usb_serial(baud: u32) -> Usart<states::Enable, mode::RxTx, usart_state::Waiting> {
        gpio::Gpio::new_usb_serial_pins();
        Usart::new(raw::USART2)
            .set_active()
            .into_rxtx()
            .into_1_stop_bit()
            .into_no_parity()
            .into_8_bit_message()
            .set_baud_rate(baud)
    }

    pub fn reopen_com(
        periph: raw::UsartAddr,
    ) -> Usart<states::Enable, mode::RxTx, usart_state::Ready> {
        Usart {
            base: raw::Usart::new(periph),
            state: states::Enable,
            mode: mode::RxTx,
            usart_state: usart_state::Ready,
        }
    }
}

impl Usart<states::Disable, Undefined, Undefined> {
    pub fn set_active(self) -> Usart<states::Enable, Undefined, usart_state::Waiting> {
        self.base.enabled().set(true);
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
        self.base.usart_enabled().set(true);
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
        let b = 16_000_000 / baud;

        self.base.baud_rate().write(b as u16);
        self
    }

    pub fn into_8_bit_message(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        self.base.word_length_9_not_8().set(false);
        self
    }

    pub fn into_9_bit_message(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        self.base.word_length_9_not_8().set(true);
        self
    }

    pub fn enable_receive_interrupt(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        match self.base.base {
            raw::USART2 => super::nvic::NVIC::new().usart2_set_enabled().set(true),
            _ => {}
        }
        self.base
            .read_data_register_not_empty_interrupt_enabled()
            .set(true);
        self
    }

    pub fn disable_receive_interrupt(self) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        self.base
            .read_data_register_not_empty_interrupt_enabled()
            .set(false);
        self
    }

    pub fn set_on_received_callback(
        self,
        cb: *mut fn(char) -> (),
    ) -> Usart<states::Enable, MODE, usart_state::Waiting> {
        match self.base.base {
            raw::USART1 => {}
            raw::USART2 => {
                unsafe { usart_handlers::USART2_HANDLER = cb };
            }
            raw::USART3 => {}
            raw::USART4 => {
                unsafe { usart_handlers::UART4_HANDLER = cb };
            }
            raw::USART5 => {}
            _ => {}
        }
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

impl Usart<states::Enable, mode::RxTx, usart_state::Ready> {
    pub fn put_char(&self, c: u8) {
        self.base.data().write(c);
        while !self.base.transmission_complete().get() {}
    }

    pub fn write_dec(&self, v: u32) {
        let mut res = v;

        let mut res_arr: [u8; 10] = [0; 10];

        for i in (0..10).rev() {
            res_arr[i] = (res % 10) as u8;
            res = res / 10;
        }

        for i in res_arr.iter() {
            self.put_char(48 + i);
        }
    }

    pub fn write(&self, s: &[u8]) {
        // let tmp = &self;
        for c in s {
            self.put_char(*c);
        }
    }

    pub fn has_received_char(&self) -> bool {
        self.base.read_data_register_not_empty().get()
    }

    pub fn read_char(&self) -> u8 {
        while !self.base.read_data_register_not_empty().get() {}
        self.base.data().read()
    }

    pub fn read(&self, res: &mut [u8]) {
        let mut p = 0;
        loop {
            if p >= res.len() {
                break;
            }

            let c = self.read_char();
            res[p] = c;

            if c == b'\n' || c == 0 || c == b'\r' {
                break;
            }

            p += 1;
        }
    }
}
