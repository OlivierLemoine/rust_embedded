use core::panic::PanicInfo;

use super::hal;

#[no_mangle]
pub unsafe extern "C" fn default_handler_print() {
    print!("Error occurd !");
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    print!("panic occurd !");
    loop {}
}
