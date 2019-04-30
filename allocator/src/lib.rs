#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

extern "C" {
    static _ssystem_ram: usize;
}

macro_rules! isFree {
    ($ptr:expr) => {
        (ptr::read($ptr) & 0x8000_0000) == 0
    };
}

macro_rules! setFree {
    ($ptr:expr) => {
        ptr::write($ptr, ptr::read($ptr) & 0x7FFF_FFFF)
    };
}

macro_rules! next {
    ($ptr:expr) => {
        $ptr.offset((getSize!($ptr) + 4) as isize)
    };
}

macro_rules! setOccupied {
    ($ptr:expr) => {
        ptr::write($ptr, ptr::read($ptr) | 0x8000_0000)
    };
}

macro_rules! getSize {
    ($ptr:expr) => {
        ptr::read($ptr) & 0x7FFF_FFFF
    };
}

macro_rules! setSize {
    ($ptr:expr, $value:expr) => {
        ptr::write($ptr, ptr::read($ptr) | (0x7FFF_FFFF & $value))
    };
}

macro_rules! enoughSize {
    ($ptr:expr, $value:expr) => {
        getSize!($ptr) >= $value
    };
}

struct Alloc {}

unsafe impl Sync for Alloc {}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ptr = _ssystem_ram as *mut u32;
        let size = layout.size() as u32;
        loop {
            if isFree!(ptr) && enoughSize!(ptr, size) {
                setSize!(ptr, size);
                setSize!(next!(ptr), getSize!(ptr) - size - 4);
                setFree!(next!(ptr));
                setOccupied!(ptr);
                break;
            }
            ptr = next!(ptr);
        }
        ptr.offset(4) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _l: Layout) {
        let header = ptr.offset(-4) as *mut u32;

        let next_header = next!(header);

        if isFree!(next_header) {
            let total_size = getSize!(next_header) + getSize!(header) + 4;
            setSize!(header, total_size);
        }
        setFree!(header);
    }
}

#[global_allocator]
static HEAP: Alloc = Alloc {};

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}