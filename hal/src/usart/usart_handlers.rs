
pub static mut UART4_HANDLER: *const fn(char) -> () = 0 as *const fn(char) -> ();

#[no_mangle]
pub unsafe extern "C" fn UART4_IRQHandler() {
    let u = super::raw::Usart::new(super::raw::USART4);
    if u.read_data_register_not_empty().get() {
        let c = u.data().read() as char;
        if UART4_HANDLER != 0 as *mut fn(char) -> () {
            (*UART4_HANDLER)(c);
        }
    }

    if u.overrun_error().get() {
        u.overrun_error().set(false);
    }

}

pub static mut USART2_HANDLER: *const fn(char) -> () = 0 as *const fn(char) -> ();

#[no_mangle]
pub unsafe extern "C" fn USART2_IRQHandler() {
    let u = super::raw::Usart::new(super::raw::USART2);
    if u.read_data_register_not_empty().get() {
        let c = u.data().read() as char;
        if USART2_HANDLER != 0 as *mut fn(char) -> () {
            (*USART2_HANDLER)(c);
        }
    }

    if u.overrun_error().get() {
        u.overrun_error().set(false);
    }
}

