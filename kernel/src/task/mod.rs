use alloc::vec::Vec;

mod task;

static mut SCHEDULER: Option<Scheduler> = None;
#[no_mangle]
static mut ctx: [u32; 16] = [0; 16];

struct WaitingTask {
    task: task::Task,
    remaining_ms: u32,
}

struct Scheduler {
    next_id: u32,
    current_task: usize,
    running_tasks: Vec<task::Task>,
    waiting_tasks: Vec<WaitingTask>,

    calling_ctx: [u32; 16],
}

pub fn init() {
    unsafe {
        SCHEDULER = Some(Scheduler {
            next_id: 0,
            current_task: 0,
            running_tasks: Vec::new(),
            waiting_tasks: Vec::new(),

            calling_ctx: [0; 16],
        });
    }
}

pub fn new_thread(f: *const fn() -> ()) {
    match unsafe { &mut SCHEDULER } {
        Some(s) => {
            s.running_tasks.push(task::Task::new(s.next_id, f));
        }
        None => panic!(),
    }
}

pub fn wait(ms: u32) {
    match unsafe { &mut SCHEDULER } {
        Some(s) => {
            s.waiting_tasks.push(WaitingTask {
                task: s.running_tasks.remove(s.current_task),
                remaining_ms: ms,
            });
            s.current_task %= s.running_tasks.len();
        }
        None => panic!(),
    }
}

pub fn tick() {
    match unsafe { &mut SCHEDULER } {
        Some(s) => {

            for i in 0..16 {
                s.running_tasks[s.current_task].set_ctx_at(i, s.calling_ctx[i]);
            }

            s.current_task = (s.current_task + 1) % s.running_tasks.len();

            for i in 0..16 {
                s.calling_ctx[i] = s.running_tasks[s.current_task].get_ctx_at(i);
            }
        }
        None => panic!(),
    }
}