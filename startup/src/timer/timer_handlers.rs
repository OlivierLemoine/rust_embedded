use super::super::register::{Bit, Register};

#[no_mangle]
pub unsafe extern "C" fn TIM2_IRQHandler() {
    Bit::new(Register::new(0x4000_0010), 6).set(false);
}
