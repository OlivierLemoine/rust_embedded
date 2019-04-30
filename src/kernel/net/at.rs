use super::super::super::gpio;
use super::super::super::usart;

static mut AT_HANDLER: *mut ATHandler = 0 as *mut ATHandler;

pub enum ConnectionType {
    TCP,
    UDP,
}

pub struct Connection {
    c_type: ConnectionType,
    id: u32,
}

impl Connection {
    pub fn new(c_type: ConnectionType, id: u32) -> Connection {
        Connection { c_type, id }
    }

    pub fn connect_to(&self, ip: String, port: String) {
        let u = usart::Usart::__com(usart::raw::USART4);
        u.write("AT+CIPSTART=\"TCP\",\"192.168.1.21\",8000\r\n".as_bytes());
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
        let u = usart::Usart::__com(usart::raw::USART4);
        u.write("AT+CIPSENDBUF=".as_bytes());
        u.write_dec(s.len());
        u.write(s.as_bytes());
    }
}

struct ATHandler {
    pub connections: Vec<Connection>,
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
        .set_baud_rate(115200)
        .ready_usart();

    // unsafe {
    //     AT_HANDLER = alloc::malloc::<ATHandler>(core::mem::size_of::<ATHandler>() as u32).unwrap();
    //     *AT_HANDLER = ATHandler {
    //         connections: Vec::new(),
    //     }
    // };
}

pub fn create(c_type: ConnectionType) -> Connection {
    let c = Connection::new(c_type, 0);
    c
}
