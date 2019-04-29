#![allow(dead_code)]

macro_rules! isFree {
    ($input:expr) => {
        (*$input & 0x8000_0000) == 0
    };
}

macro_rules! setFree {
    ($input:expr) => {
        *$input = *$input & 0x7FFF_FFFF
    };
}

macro_rules! setOccupied {
    ($input:expr) => {
        *$input = *$input | 0x8000_0000
    };
}

macro_rules! getSize {
    ($input:expr) => {
        *$input & 0x7FFF_FFFF
    };
}

macro_rules! setSize {
    ($input:expr, $input2:expr) => {
        *$input = *$input | (0x7FFF_FFFF & $input2)
    };
}

macro_rules! enoughSize {
    ($input:expr, $input2:expr) => {
        getSize!($input) + 4 >= $input2
    };
}

extern "C" {
    static _ssystem_ram: u32;
}

// const _ssystem_ram: u32 = 0x2001_FC00;

pub unsafe fn alloc_init() {
    let pos = _ssystem_ram as *mut u32;
    *pos = 0x7FFF_FFFF;
}

pub unsafe fn malloc<T>(size: u32) -> Result<*mut T, bool> {
    let mut ptr = _ssystem_ram as *mut u32;
    loop {
        if isFree!(ptr) && enoughSize!(ptr, size) {
            setSize!(ptr, size + 4);
            setSize!(ptr.offset(getSize!(ptr) as isize), getSize!(ptr) - size - 4);
            setOccupied!(ptr);
            break;
        }
        ptr = ptr.offset(getSize!(ptr) as isize);
    }
    Ok(ptr.offset(4) as *mut T)
}

// pub fn malloc(size: u32) -> Result<*mut _, bool> {
//     if size > 0x7FFF_FFFF {
//         return Err(false);
//     }

//     let step = size + 4;
//     let mut pos = unsafe { _ssystem_ram } as *mut u8;
//     while !is_free(read_state(pos)) || step > (read_state(pos) & 0x7FFF_FFFF) {
//         pos = ((pos as u32) - (read_state(pos) & 0x7FFF_FFFF)) as *mut u8;
//     }
//     let rem_size = read_state(pos) & 0x7FFF_FFFF;

//     write_state(pos, 0x8000_0000 | step);

//     pos = ((pos as u32) - step) as *mut u8;

//     write_state(pos, rem_size - step);

//     Ok((pos as u32 + 1) as *mut _)
// }

// pub fn free(p: *mut u8) {
//     // let pos1 = ((p as u32) - 1) as *mut u8;

//     // let pos2 = ((pos1 as u32) - (read_state(pos1) & 0x7FFF_FFFF)) as *mut u8;

//     let mut pos = unsafe { _ssystem_ram } as *mut u8;

//     let pos2 = ((p as u32) - 1) as *mut u8;

//     let pos1: *mut u8 = loop {

//         let new_pos = ((pos as u32) - (read_state(pos) & 0x7FFF_FFFF)) as *mut u8;

//         if (new_pos as u32) < (p as u32) {
//             break new_pos;
//         }
//         pos = new_pos;
//     };

//     if is_free(read_state(pos2)) {
//         write_state(
//             pos1,
//             ((pos1 as u32) & 0x7FFF_FFFF) + ((pos2 as u32) & 0x7FFF_FFFF),
//         );
//     } else {
//         write_state(pos1, (pos1 as u32) & 0x7FFF_FFFF);
//     }
// }

// pub fn realloc(p: *mut u8, size: u32) -> *mut u8 {
//     let tmp = malloc(size).unwrap();
//     mem_cpy(p, tmp, size);
//     free(p);
//     tmp
// }

// pub fn mem_cpy(src: *mut u8, dest: *mut u8, len: u32) {
//     for i in 0..len {
//         unsafe {
//             *dest.offset(i as isize) = *src.offset(i as isize);
//         }
//     }
// }

// fn is_free(v: u32) -> bool {
//     (v & 0x8000_0000) != 0
// }

// fn read_state(v: *mut u8) -> u32 {
//     unsafe {
//         (*v as u32) << 24
//             | (*(v.offset(-1)) as u32) << 16
//             | (*(v.offset(-2)) as u32) << 8
//             | (*(v.offset(-3)) as u32)
//     }
// }

// fn write_state(v: *mut u8, value: u32) {
//     unsafe {
//         *v = ((value & 0xFF00_0000) >> 24) as u8;
//         *(v.offset(-1)) = ((value & 0x00FF_0000) >> 16) as u8;
//         *(v.offset(-2)) = ((value & 0x0000_FF00) >> 8) as u8;
//         *(v.offset(-3)) = (value & 0x0000_00FF) as u8;
//     };
// }
