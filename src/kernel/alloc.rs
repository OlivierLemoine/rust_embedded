#![allow(dead_code)]

use core::ptr;

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

extern "C" {
    static _ssystem_ram: u32;
}

// const _ssystem_ram: u32 = 0x2001_FC00;

pub unsafe fn alloc_init() {
    let pos = _ssystem_ram as *mut u32;
    ptr::write(pos, 0x7FFF_FFFF);
}

pub unsafe fn malloc<T>(size: u32) -> Result<*mut T, bool> {
    let mut ptr = _ssystem_ram as *mut u32;
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
    Ok(ptr.offset(4) as *mut T)
}

pub unsafe fn free<T>(ptr: *mut T) {
    let header = ptr.offset(-4) as *mut u32;

    let next_header = next!(header);

    if isFree!(next_header) {
        let total_size = getSize!(next_header) + getSize!(header) + 4;
        setSize!(header, total_size);
    }
    setFree!(header);
}

pub unsafe fn realloc<T>(ptr: *mut T, size: u32) -> Result<*mut T, bool> {
    let header = ptr.offset(-4) as *mut u32;
    let curr_size = getSize!(header);

    let diff_size = (size as i32) - (curr_size as i32);

    let next_header = next!(header);

    if diff_size < 0 {
        setSize!(header, size);
        setSize!(next!(header), getSize!(next_header) + (-diff_size as u32));
        setFree!(next!(header));
        return Ok(ptr);
    }

    if diff_size == 0 {
        return Ok(ptr);
    }

    if isFree!(next_header) && enoughSize!(next_header, diff_size as u32) {
        setSize!(header, size);
        setSize!(next!(header), getSize!(next_header) - size);
        setFree!(next!(header));
        setOccupied!(header);
        return Ok(ptr);
    } else {
        return match malloc(size) {
            Ok(p) => {
                mem_cpy(ptr, p, curr_size);
                free(ptr);
                Ok(p)
            }
            Err(e) => Err(e),
        };
    }
}

pub unsafe fn mem_cpy<T>(src: *mut T, dest: *mut T, len: u32) {
    let p1 = src as *mut u8;
    let p2 = dest as *mut u8;

    for i in 0..len {
        *p2.offset(i as isize) = *p1.offset(i as isize);
    }
}
