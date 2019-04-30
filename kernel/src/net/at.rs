use hal::gpio;
use hal::usart;


use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
static mut AT_HANDLER: ATHandler = ATHandler {
    connections: None,
    data_in: None,
};

pub enum ConnectionType {
    TCP,
    UDP,
}

struct __Connection {
    c_type: ConnectionType,
    id: usize,
}

struct ATHandler {
    connections: Option<Vec<__Connection>>,
    data_in: Option<String>,
}

pub type ConnectionFd = usize;

pub trait Connection {
    fn connect_to(&self, ip: String, port: String);
    fn send(&self, msg: String);
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

    pub fn get_data_in(&self) -> &String {
        match &self.data_in {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }

    pub fn get_data_in_mut(&mut self) -> &mut String {
        match &mut self.data_in {
            Some(v) => v,
            None => panic!("Initialize AT before use"),
        }
    }
}

impl __Connection {
    pub fn new(c_type: ConnectionType, id: usize) -> __Connection {
        __Connection { c_type, id }
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

    usart::Usart::new(usart::raw::USART4)
        .set_active()
        .into_rxtx()
        .into_8_bit_message()
        .into_no_parity()
        .into_1_stop_bit()
        .enable_receive_interrupt()
        .set_on_received_callback(add_new_char as *mut fn(char))
        .set_baud_rate(115200)
        .ready_usart();

    unsafe {
        AT_HANDLER.connections = Some(Vec::with_capacity(0));
        AT_HANDLER.data_in = Some(String::from(""));
    }
}

pub fn create(c_type: ConnectionType) -> ConnectionFd {
    let id = unsafe { &AT_HANDLER }.get_connection().len();
    let c = __Connection::new(c_type, id);
    unsafe { &mut AT_HANDLER }.get_connection_mut().push(c);
    id
}

fn add_new_char(c: char) {
    let s: &mut String = unsafe { &mut AT_HANDLER }.get_data_in_mut();
    s.push(c);
    let u = hal::usart::Usart::reopen_com(hal::usart::raw::USART2);
    u.write(s.as_bytes());
    u.put_char(b'\n');
}