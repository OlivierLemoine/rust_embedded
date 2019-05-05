use alloc::string::String;
use hal::usart::{raw::USART4, Usart};

// pub fn connect(ssid: String, pwd: String) {
//     let u = Usart::reopen_com(USART4);
//     u.write("AT+CWMODE".as_bytes());
//     hal::delay(1000);
//     u.write("AT+CWJAP=\"".as_bytes());
//     u.write(ssid.as_bytes());
//     u.write("\",\"".as_bytes());
//     u.write(pwd.as_bytes());
//     u.write("\"\r\n".as_bytes());
// }

pub fn reset_module() -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+RST\r\n".as_bytes());
    super::at::read_wifi()
}

pub fn wifi_into_client_and_ap() -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CWMODE=3\r\n".as_bytes());
    super::at::read_wifi()
}

pub fn wifi_into_ap() -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CWMODE=2\r\n".as_bytes());
    super::at::read_wifi()
}

pub fn wifi_into_client() -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CWMODE=1\r\n".as_bytes());
    super::at::read_wifi()
}

pub fn chip_version() -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+GMR\r\n".as_bytes());
    super::at::read_wifi()
}

pub fn list_available_ap() -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CWLAP\r\n".as_bytes());
    super::at::read_wifi()
}

pub fn connect_to_ap(ssid: String, pwd: String) -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CWJAP=\"".as_bytes());
    u.write(ssid.as_bytes());
    u.write("\",\"".as_bytes());
    u.write(pwd.as_bytes());
    u.write("\"\r\n".as_bytes());
    super::at::read_wifi()
}

pub fn get_self_ip() -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CIFSR\r\n".as_bytes());
    super::at::read_wifi()
}

pub fn into_multiple_connection() -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CIPMUX=1\r\n".as_bytes());
    super::at::read_wifi()
}

pub fn into_single_connection() -> String {
    let u = Usart::reopen_com(USART4);
    u.write("AT+CIPMUX=0\r\n".as_bytes());
    super::at::read_wifi()
}