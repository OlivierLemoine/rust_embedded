use alloc::vec::Vec;

mod task;

static mut SCHEDULER: Option<Scheduler> = None;
#[no_mangle]
static mut ctx: [u32; 16] = [0; 16];

struct Scheduler {
    next_id: u32,
    running_tasks: Vec<task::Task>,
    waiting_tasks: Vec<task::Task>,
}

pub fn init() {
    unsafe {
        SCHEDULER = Some(Scheduler {
            next_id: 0,
            running_tasks: Vec::new(),
            waiting_tasks: Vec::new(),
        });
    }
}

pub fn new_thread(f: *const fn() -> ()) {
    match unsafe { &mut SCHEDULER } {
        Some(s) => {
            s.running_tasks.push(task::Task::new(s.next_id, f));
        }
        None => {
            panic!();
        }
    }
}
