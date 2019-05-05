use hal::gpio;
use hal::usart;

use alloc::string::String;
use alloc::vec::Vec;

const ESP_BUFFER_SIZE: usize = 128;

static mut AT_HANDLER: ATHandler = ATHandler {
    connections: None,
    data_in: None,
    ptr_write: 0,
    ptr_read: 0,
    tmp_parse: None,
    wifi_in: None,
    wifi_end: false,
    state: -1,
    curr_connection_number: 0,
    size_to_read: 0,
};

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
    connections: Option<Vec<__Connection>>,
    data_in: Option<[char; ESP_BUFFER_SIZE]>,
    ptr_write: usize,
    ptr_read: usize,
    tmp_parse: Option<String>,
    wifi_in: Option<String>,
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
    for i in unsafe { &mut AT_HANDLER }.get_connection_mut() {
        if i.get_id() == id {
            return Some(i);
        }
    }
    None
}

impl ATHandler {
    pub fn get_connection(&self) -> &Vec<__Connection> {
        match &self.connections {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }

    pub fn get_connection_mut(&mut self) -> &mut Vec<__Connection> {
        match &mut self.connections {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }

    pub fn get_data_in(&self) -> &[char] {
        match &self.data_in {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }

    pub fn get_data_in_mut(&mut self) -> &mut [char] {
        match &mut self.data_in {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }

    pub fn get_tmp_parse(&self) -> &String {
        match &self.tmp_parse {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }

    pub fn get_tmp_parse_mut(&mut self) -> &mut String {
        match &mut self.tmp_parse {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }

    pub fn get_wifi_in(&self) -> &String {
        match &self.wifi_in {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }

    pub fn get_wifi_in_mut(&mut self) -> &mut String {
        match &mut self.wifi_in {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }
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
        let _a = unsafe { &AT_HANDLER };
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

    fn get_id(&self) -> usize {
        self.id
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
        AT_HANDLER.connections = Some(Vec::with_capacity(0));
        AT_HANDLER.data_in = Some(['\0'; ESP_BUFFER_SIZE]);
        AT_HANDLER.wifi_in = Some(String::new());
        AT_HANDLER.tmp_parse = Some(String::new());
    }
}

pub fn create(c_type: ConnectionType) -> ConnectionFd {
    let mut id: usize = 0;
    loop {
        let mut fine = true;
        for i in unsafe { &AT_HANDLER }.get_connection() {
            if i.get_id() == id {
                id += 1;
                fine = false;
            }
        }

        if fine {
            break;
        }
    }
    let c = __Connection::new(c_type, id);
    unsafe { &mut AT_HANDLER }.get_connection_mut().push(c);
    id
}

unsafe fn add_new_char(c: char) {
    hal::usart::raw::Usart::new(hal::usart::raw::USART2)
        .data()
        .write(c as u8);
    let t = AT_HANDLER.get_data_in_mut();
    t[AT_HANDLER.ptr_write] = c;
    AT_HANDLER.ptr_write = (AT_HANDLER.ptr_write + 1) % ESP_BUFFER_SIZE;
}

pub fn read_wifi() -> String {
    unsafe { dispatch() };
    while !unsafe { AT_HANDLER.wifi_end } {
        unsafe { dispatch() };
    }
    unsafe { AT_HANDLER.wifi_end = false };
    let s: &mut String = unsafe { AT_HANDLER.get_wifi_in_mut() };
    let res = s.clone();
    *s = String::new();
    res
}

unsafe fn dispatch() {
    while AT_HANDLER.ptr_write != AT_HANDLER.ptr_read {
        let c = AT_HANDLER.get_data_in_mut()[AT_HANDLER.ptr_read];
        match AT_HANDLER.state {
            -40 => {
                if c == '>' {
                    AT_HANDLER.wifi_end = true;
                    AT_HANDLER.state = -2;
                }
            }
            -31 => {
                if c == ',' {
                    AT_HANDLER.state = AT_HANDLER.curr_connection_number as i32;
                } else {
                    AT_HANDLER.size_to_read *= 10;
                    AT_HANDLER.size_to_read += (c as u32) - 48;
                }
            }
            -30 => {
                if c == ',' {
                    AT_HANDLER.state = -31;
                } else {
                    AT_HANDLER.curr_connection_number *= 10;
                    AT_HANDLER.curr_connection_number += (c as u32) - 48;
                }
            }
            -20 => {
                if c == '\n' {
                    AT_HANDLER.state = -2;
                }
            }

            -2 => {
                let s: &mut String = &mut AT_HANDLER.get_wifi_in_mut();
                s.push(c);
                if s.ends_with("OK") || s.ends_with("ERROR") {
                    AT_HANDLER.state = -1;
                    AT_HANDLER.wifi_end = true;
                }
            }

            -1 => {
                let s_tmp: &mut String = AT_HANDLER.get_tmp_parse_mut();
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
                    AT_HANDLER.state = -20;
                    AT_HANDLER.wifi_end = false;
                    AT_HANDLER.tmp_parse = Some(String::new());
                } else if s_tmp.ends_with("CLOSE") {
                } else if s_tmp.ends_with("AT+CIPSENDBUF=") {
                    AT_HANDLER.state = -20;
                    AT_HANDLER.wifi_end = false;
                    AT_HANDLER.tmp_parse = Some(String::new());
                } else if s_tmp.ends_with("+IPD,") {
                    // AT_HANDLER.curr_connection_number = 0;
                    // AT_HANDLER.size_to_read = 0;
                    // AT_HANDLER.state = -30;
                }
            }
            x => {
                let v = find_corresponding_connection(x as usize).unwrap();
                v.push_c(c);
                AT_HANDLER.size_to_read -= 1;
                if AT_HANDLER.size_to_read == 0 {
                    AT_HANDLER.state = -1;
                }
            }
        }
        AT_HANDLER.ptr_read = (AT_HANDLER.ptr_read + 1) % ESP_BUFFER_SIZE;
    }
}
