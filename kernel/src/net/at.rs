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
    state: -1,
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
    state: i32,
    size_to_read: i32,
}

pub type ConnectionFd = usize;

pub trait Connection {
    fn connect_to(&self, ip: String, port: String);
    fn send(&self, msg: String);
    fn read(&self) -> String;
}

impl Connection for ConnectionFd {
    fn connect_to(&self, ip: String, port: String) {
        let tmp = &unsafe { &AT_HANDLER }.get_connection()[*self];
        tmp.connect_to(ip, port);
    }

    fn send(&self, msg: String) {
        let tmp = &unsafe { &AT_HANDLER }.get_connection()[*self];
        tmp.send(msg);
    }

    fn read(&self) -> String {
        String::new()
    }
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
        // u.write("AT+CIPSTART=\"TCP\",\"192.168.1.21\",8000\r\n".as_bytes());
        u.write(b"AT+CIPSTART=");
        u.write(match self.c_type {
            ConnectionType::TCP => b"TCP,",
            ConnectionType::UDP => b"UDP,",
        });
        u.write(ip.as_bytes());
        u.put_char(b',');
        u.write(port.as_bytes());
        u.write(b"\r\n");
    }

    pub fn send(&self, s: String) {
        let u = usart::Usart::reopen_com(usart::raw::USART4);
        u.write("AT+CIPSENDBUF=".as_bytes());
        u.write_dec(s.len() as u32);
        u.write(s.as_bytes());
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
        .ready_usart();

    unsafe {
        AT_HANDLER.connections = Some(Vec::with_capacity(0));
        AT_HANDLER.data_in = Some(['\0'; ESP_BUFFER_SIZE]);
        AT_HANDLER.wifi_in = Some(String::from(""));
        AT_HANDLER.tmp_parse = Some(String::from(""));
    }
}

pub fn create(c_type: ConnectionType) -> ConnectionFd {
    let id = unsafe { &AT_HANDLER }.get_connection().len();
    let c = __Connection::new(c_type, id);
    unsafe { &mut AT_HANDLER }.get_connection_mut().push(c);
    id
}

unsafe fn add_new_char(c: char) {
    print_char!(c);
    AT_HANDLER.ptr_write += 1 % ESP_BUFFER_SIZE;
    AT_HANDLER.get_data_in_mut()[AT_HANDLER.ptr_write] = c;
}

unsafe fn dispatch() {
    while AT_HANDLER.ptr_write != AT_HANDLER.ptr_read {
        let c = AT_HANDLER.get_data_in_mut()[AT_HANDLER.ptr_read];
        match AT_HANDLER.state {
            -2 => {
                let s: &mut String = &mut AT_HANDLER.get_wifi_in_mut();
                s.push(c);
                if s.ends_with("OK") || s.ends_with("ERROR") {
                    AT_HANDLER.state = -1;
                }
            }

            -1 => {
                if c == '\n' {
                    match AT_HANDLER.get_tmp_parse().as_str() {
                        "AT+CWLAP" => {
                            AT_HANDLER.state = -2;
                            AT_HANDLER.tmp_parse = Some(String::new());
                        }
                        _ => {}

                    }
                } else if c == ':' {
                } else {
                    AT_HANDLER.get_tmp_parse_mut().push(c);
                }
            }
            x => {
                let v: &mut Vec<__Connection> = &mut AT_HANDLER.get_connection_mut();
                v[x as usize].push_c(c);
                AT_HANDLER.size_to_read -= 1;
                if AT_HANDLER.size_to_read == 0 {
                    AT_HANDLER.state = -1;
                }
            }
        }
        AT_HANDLER.ptr_read += 1 % ESP_BUFFER_SIZE;
    }
}
