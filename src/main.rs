#![no_main]
#![no_std]

extern crate alloc;
extern crate allocator;
#[macro_use]
extern crate hal;
extern crate hal_alloc;
extern crate kernel;

mod panic_handler;


use alloc::boxed::Box;
use alloc::string::String;

#[no_mangle]
pub unsafe extern "C" fn main() {
    // hal::rcc::Rcc::new().enable_hsi().sysclock_into_hsi();
    let rcc = hal::rcc::Rcc::new()
        .enable_hsi()
        .main_pll_src_into_hsi()
        .set_pll_m(8)
        .unwrap()
        .set_pll_n(168)
        .unwrap()
        .set_pll_p(2)
        .set_pll_q(7)
        .set_ahb_prescaler(1)
        .set_apb1_prescaler(4)
        .set_apb2_prescaler(2)
        .enable_pll()
        .unwrap();
    rcc.sysclock_into_pll()
        .unwrap();
    allocator::init();
    kernel::init();

    hal_alloc::setup_usart2(Box::new(|_c| {
        print_char!(_c);
    }));

    let _list = kernel::net::wifi::list_available_ap();

    // kernel::net::wifi::connect(String::from("Livebox-092d"), String::from("wifieasy"));

    // let socket = kernel::net::tcp::Tcp::new();
    // socket.connect(String::from("192.168.1.21"), String::from("8000"));

    loop {}
}
