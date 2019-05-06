use hal::gpio;
use hal::usart;

use alloc::string::String;
use alloc::vec::Vec;

const ESP_BUFFER_SIZE: usize = 128;

static mut AT_HANDLER: Option<ATHandler> = None;

pub enum ConnectionType {
    TCP,
    UDP,
}

struct __Connection {
    c_type: ConnectionType,
    id: usize,
    data: String,
}

struct ATHandler {
    connections: Vec<__Connection>,
    data_in: [char; ESP_BUFFER_SIZE],
    ptr_write: usize,
    ptr_read: usize,
    tmp_parse: String,
    wifi_in: String,
    wifi_end: bool,
    state: i32,
    curr_connection_number: u32,
    size_to_read: u32,
}

pub type ConnectionFd = usize;

pub trait Connection {
    fn connect_to(&self, ip: String, port: String) -> String;
    fn send(&self, msg: String) -> String;
    fn read(&self) -> String;
}

impl Connection for ConnectionFd {
    fn connect_to(&self, ip: String, port: String) -> String {
        let tmp = find_corresponding_connection(*self).unwrap();
        tmp.connect_to(ip, port);
        read_wifi()
    }

    fn send(&self, msg: String) -> String {
        let tmp = find_corresponding_connection(*self).unwrap();
        tmp.send(msg);
        read_wifi()
    }

    fn read(&self) -> String {
        let tmp = find_corresponding_connection(*self).unwrap();

        unsafe { dispatch() };

        let res = tmp.data.clone();
        tmp.data = String::new();
        res
    }
}

fn find_corresponding_connection<'a>(id: usize) -> Option<&'a mut __Connection> {
    match unsafe { &mut AT_HANDLER } {
        Some(a) => {
            for i in &mut a.connections {
                if i.id == id {
                    return Some(i);
                }
            }
        }
        None => panic!(),
    }
    None
}

impl __Connection {
    pub fn new(c_type: ConnectionType, id: usize) -> __Connection {
        __Connection {
            c_type,
            id,
            data: String::new(),
        }
    }

    pub fn connect_to(&self, ip: String, port: String) {
        let u = usart::Usart::reopen_com(usart::raw::USART4);
        u.write("AT+CIPSTART=".as_bytes());
        u.write_dec(self.id as u32);
        u.write(match self.c_type {
            ConnectionType::TCP => ",\"TCP\",\"".as_bytes(),
            ConnectionType::UDP => ",\"UDP\",\"".as_bytes(),
        });
        u.write(ip.as_bytes());
        u.write("\",".as_bytes());
        u.write(port.as_bytes());
        u.write("\r\n".as_bytes());
    }

    pub fn send(&self, s: String) {
        let u = usart::Usart::reopen_com(usart::raw::USART4);
        u.write("AT+CIPSEND=".as_bytes());
        u.write_dec(self.id as u32);
        u.write(",".as_bytes());
        u.write_dec(s.len() as u32);
        u.write("\r\n".as_bytes());
        // read_wifi();
        // u.write(s.as_bytes());
    }

    fn push_c(&mut self, c: char) {
        self.data.push(c);
    }
}

pub fn init() {
    gpio::Gpio::new(gpio::raw::GPIO_A, 1)
        .set_active()
        .into_alternate()
        .alternate_function(gpio::alternate_function::USART4)
        .into_high_speed()
        .into_no_pull()
        .into_push_pull();
    gpio::Gpio::new(gpio::raw::GPIO_A, 0)
        .set_active()
        .into_alternate()
        .alternate_function(gpio::alternate_function::USART4)
        .into_high_speed()
        .into_no_pull()
        .into_push_pull();

    let tmp = (&(add_new_char as *const fn(char)) as *const *const fn(char)) as *const fn(char);

    usart::Usart::new(usart::raw::USART4)
        .set_active()
        .into_rxtx()
        .into_8_bit_message()
        .into_no_parity()
        .into_1_stop_bit()
        .enable_receive_interrupt()
        .set_on_received_callback(tmp)
        .set_baud_rate(115200)
        .ready_usart()
        .write("\r\n\r\n".as_bytes());

    unsafe {
        AT_HANDLER = Some(ATHandler {
            connections: Vec::new(),
            data_in: ['\0'; ESP_BUFFER_SIZE],
            ptr_write: 0,
            ptr_read: 0,
            tmp_parse: String::new(),
            wifi_in: String::new(),
            wifi_end: false,
            state: -1,
            curr_connection_number: 0,
            size_to_read: 0,
        });
    }
}

pub fn create(c_type: ConnectionType) -> ConnectionFd {
    match unsafe { &mut AT_HANDLER } {
        Some(a) => {
            let mut id: usize = 0;
            loop {
                let mut fine = true;
                for i in &a.connections {
                    if i.id == id {
                        id += 1;
                        fine = false;
                    }
                }

                if fine {
                    break;
                }
            }
            let c = __Connection::new(c_type, id);
            a.connections.push(c);
            id
        }
        None => panic!(),
    }
}

unsafe fn add_new_char(c: char) {
    // hal::usart::raw::Usart::new(hal::usart::raw::USART2)
    //     .data()
    //     .write(c as u8);
    match &mut AT_HANDLER {
        Some(a) => {
            let mut t = a.data_in;
            t[a.ptr_write] = c;
            a.ptr_write = (a.ptr_write + 1) % ESP_BUFFER_SIZE;
        }
        None => panic!(),
    }
}

pub fn read_wifi() -> String {
    match unsafe { &mut AT_HANDLER } {
        Some(a) => {
            unsafe { dispatch() };
            while !a.wifi_end {
                unsafe { dispatch() };
            }
            a.wifi_end = false;
            let s: &mut String = &mut a.wifi_in;
            let res = s.clone();
            *s = String::new();
            res
        }
        None => panic!(),
    }

}


fn dispatch() {
    match unsafe { &mut AT_HANDLER } {
        Some(a) => {
            while a.ptr_write != a.ptr_read {
                let c = a.data_in[a.ptr_read];
                match a.state {
                    -40 => {
                        if c == '>' {
                            a.wifi_end = true;
                            a.state = -2;
                        }
                    }
                    -31 => {
                        if c == ',' {
                            a.state = a.curr_connection_number as i32;
                        } else {
                            a.size_to_read *= 10;
                            a.size_to_read += (c as u32) - 48;
                        }
                    }
                    -30 => {
                        if c == ',' {
                            a.state = -31;
                        } else {
                            a.curr_connection_number *= 10;
                            a.curr_connection_number += (c as u32) - 48;
                        }
                    }
                    -20 => {
                        if c == '\n' {
                            a.state = -2;
                        }
                    }
                    -2 => {
                        let s: &mut String = &mut a.wifi_in;
                        s.push(c);
                        if s.ends_with("OK") || s.ends_with("ERROR") {
                            a.state = -1;
                            a.wifi_end = true;
                        }
                    }

                    -1 => {
                        let s_tmp: &mut String = &mut a.tmp_parse;
                        s_tmp.push(c);

                        if s_tmp.ends_with("AT+GMR")
                            || s_tmp.ends_with("AT+CWMODE=")
                            || s_tmp.ends_with("AT+CWLAP")
                            || s_tmp.ends_with("AT+CWJAP=")
                            || s_tmp.ends_with("AT+CIFSR")
                            || s_tmp.ends_with("AT+CIPMUX=")
                            || s_tmp.ends_with("AT+CIPSTART=")
                            || s_tmp.ends_with("AT+CIPSEND=")
                        {
                            a.state = -20;
                            a.wifi_end = false;
                            a.tmp_parse = String::new();
                        } else if s_tmp.ends_with("CLOSE") {
                        } else if s_tmp.ends_with("AT+CIPSENDBUF=") {
                            a.state = -20;
                            a.wifi_end = false;
                            a.tmp_parse = String::new();
                        } else if s_tmp.ends_with("+IPD,") {
                            // AT_HANDLER.curr_connection_number = 0;
                            // AT_HANDLER.size_to_read = 0;
                            // AT_HANDLER.state = -30;
                        }
                    }
                    x => {
                        let v = find_corresponding_connection(x as usize).unwrap();
                        v.push_c(c);
                        a.size_to_read -= 1;
                        if a.size_to_read == 0 {
                            a.state = -1;
                        }
                    }
                }
                a.ptr_read = (a.ptr_read + 1) % ESP_BUFFER_SIZE;
            }
        }
        None => panic!(),
    }
}
