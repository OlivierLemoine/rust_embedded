use super::at;
use alloc::string::String;

pub struct Tcp {
    connection: at::Connection,
}

impl Tcp {
    pub fn new() -> Tcp {
        Tcp {
            connection: at::create(at::ConnectionType::TCP),
        }
    }

    pub fn connect(&self, ip: String, port: String) {
        self.connection.connect_to(ip, port);
    }

    pub fn send(&self, s: String) {
        self.connection.send(s);
    }
}
