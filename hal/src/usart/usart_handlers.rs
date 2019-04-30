use alloc::boxed::Box;

pub static mut USART4_HANDLER: Option<Box<Fn(char) -> ()>> = None;

#[no_mangle]
pub unsafe extern "C" fn USART4_IRQHandler() {
    let c = super::raw::Usart::new(super::raw::USART4).data().read() as char;
    match &USART4_HANDLER {
        Some(f) => f(c),
        None => {}
    }
}