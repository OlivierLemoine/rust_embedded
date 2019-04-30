use core::panic::PanicInfo;

use super::hal;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    print!("panic occurd !");
    loop {}
}
