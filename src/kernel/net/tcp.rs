use super::super::string::String;
use super::at;

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
        self.connect(ip, port);
    }

    pub fn send(&self, s: String) {
        self.send(s);
    }
}
