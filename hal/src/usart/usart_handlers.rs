
pub static mut UART4_HANDLER: *mut fn(char) -> () = 0 as *mut fn(char) -> ();

#[no_mangle]
pub unsafe extern "C" fn UART4_IRQHandler() {
    let c = super::raw::Usart::new(super::raw::USART4).data().read() as char;

    if UART4_HANDLER != 0 as *mut fn(char) -> () {
        (*UART4_HANDLER)(c);
    }
}

pub static mut USART2_HANDLER: *mut fn(char) -> () = 0 as *mut fn(char) -> ();

#[no_mangle]
pub unsafe extern "C" fn USART2_IRQHandler() {
    let c = super::raw::Usart::new(super::raw::USART4).data().read() as char;

    if USART2_HANDLER != 0 as *mut fn(char) -> () {
        (*USART2_HANDLER)(c);
    }
}

