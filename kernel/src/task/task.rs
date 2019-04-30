use alloc::boxed::Box;
use core::ptr;

pub struct Task {
    id: u32,
    ctx: [u32; 16],
}

impl Task {
    pub fn new(id: u32, f: *const ()) -> Task {
        let mut t = Task { id, ctx: [0; 16] };
        t.ctx[15] = f as u32;
        t
    }
}

fn test() {}
