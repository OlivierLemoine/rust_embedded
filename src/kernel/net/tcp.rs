use alloc::string::String;

use super::at;
use at::Connection;


pub struct Tcp {
    connection: at::ConnectionFd,
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
