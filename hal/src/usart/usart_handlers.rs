
pub static mut USART4_HANDLER: *mut fn(char) -> () = 0 as *mut fn(char) -> ();

#[no_mangle]
pub unsafe extern "C" fn USART4_IRQHandler() {
    let c = super::raw::Usart::new(super::raw::USART4).data().read() as char;

    if USART4_HANDLER != 0 as *mut fn(char) -> () {
        (*USART4_HANDLER)(c);
    }
}