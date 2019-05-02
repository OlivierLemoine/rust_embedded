use alloc::string::String;
use hal::usart::{raw::USART4, Usart};

pub fn connect(ssid: String, pwd: String) {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CWMODE".as_bytes());
    hal::delay(1000);
    u.write("AT+CWJAP=\"".as_bytes());
    u.write(ssid.as_bytes());
    u.write("\",\"".as_bytes());
    u.write(pwd.as_bytes());
    u.write("\"\r\n".as_bytes());
}

pub fn list_available_ap() {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CWLAP\r\n".as_bytes());
}