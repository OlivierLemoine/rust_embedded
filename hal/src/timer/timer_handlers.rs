use super::super::register::{Bit, Register};

static mut TEST_VAR: bool = false;

#[no_mangle]
pub unsafe extern "C" fn TIM2_IRQHandler() {
    Bit::new(Register::new(0x4000_0010), 0).set(false);

    match TEST_VAR {
        true => {
            TEST_VAR = false;
            Bit::new(Register::new(0x4002_0014), 0).set(false);
        }
        false => {
            TEST_VAR = true;
            Bit::new(Register::new(0x4002_0014), 0).set(true);
        }
    }
}
