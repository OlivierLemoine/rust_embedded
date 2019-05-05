use alloc::string::String;

use super::at;
use super::at::Connection;


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
        print!(self.connection.connect_to(ip, port));
    }

    pub fn send(&self, s: String) {
        print!(self.connection.send(s));
    }

    pub fn read(&self) -> String {
        self.connection.read()
    }
}
