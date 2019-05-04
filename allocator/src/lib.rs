#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

extern "C" {
    static mut _ebss: u32;
}

fn get_heap_start() -> *mut u32 {
    let a = unsafe { &mut _ebss };
    a as *mut u32
}

macro_rules! isFree {
    ($ptr:expr) => {
        (ptr::read($ptr) & 0x8000_0000) == 0
    };
}

macro_rules! next {
    ($ptr:expr) => {
        $ptr.offset((getSize!($ptr)) as isize)
    };
}

macro_rules! getSize {
    ($ptr:expr) => {
        ptr::read($ptr) & 0x7FFF_FFFF
    };
}

macro_rules! set {
    ($ptr:expr, $isOcc:expr, $size:expr) => {
        ptr::write(
            $ptr,
            if $isOcc == true { 0x8000_0000 } else { 0 } | ($size & 0x7FFF_FFFF),
        )
    };
}

macro_rules! enoughSize {
    ($ptr:expr, $value:expr) => {
        getSize!($ptr) >= $value
    };
}

struct Alloc {
    size: usize,
}

unsafe impl Sync for Alloc {}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let raw_size = layout.size() as u32;
        let size = 1 + raw_size / 4 + if raw_size % 4 < 3 { 1 } else { 0 };
        let mut p = get_heap_start();
        loop {
            if isFree!(p) && enoughSize!(p, size) {
                let prev_size = getSize!(p);
                set!(p, true, size);
                if prev_size != size {
                    let next_p = next!(p);
                    set!(next_p, false, prev_size - size);
                }
                break;
            } else if ptr::read(p) == 0 {
                on_oom(layout);
            }
            p = next!(p);
        }
        let res = p.offset(1);
        res as *mut u8
    }

    unsafe fn dealloc(&self, p: *mut u8, _l: Layout) {
        let tmp = p as *mut u32;
        let header = tmp.offset(-1);

        let next_header = next!(header);

        set!(header, false, getSize!(header));


        if isFree!(next_header) {
            let total_size = getSize!(next_header) + getSize!(header);
            set!(header, false, total_size);
        } else {
            set!(header, false, getSize!(header));
        }
    }
}

#[global_allocator]
static HEAP: Alloc = Alloc { size: 0x8000 };

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    panic!("Out of memory");
}

pub fn init() {
    let p = get_heap_start();
    unsafe { ptr::write(p, HEAP.size as u32) };
}