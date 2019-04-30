use alloc::{vec::Vec, boxed::Box};

mod task;

static mut SCHEDULER: Scheduler = Scheduler { tasks: None };
#[no_mangle]
static mut ctx: [u32; 16] = [0; 16];

struct Scheduler {
    tasks: Option<Vec<task::Task>>,
}

pub fn init() {
    unsafe { SCHEDULER.tasks = Some(Vec::new()) };
}

pub fn new_thread(f: impl Fn() -> ()){
    
}