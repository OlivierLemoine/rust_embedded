use alloc::string::String;
use hal::usart::{raw::USART4, Usart};

pub fn connect(ssid: String, pwd: String) {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CWJAP=\"".as_bytes());
    u.write(ssid.as_bytes());
    u.write("\",\"".as_bytes());
    u.write(pwd.as_bytes());
    u.write("\"\r\n".as_bytes());
}