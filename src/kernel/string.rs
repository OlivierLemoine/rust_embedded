#![allow(dead_code)]

use super::alloc;

pub struct String {
    ptr: *mut u8,
    size: u32,
}

impl String {
    pub fn new(size: u32) -> String {
        String {
            ptr: if size == 0 {
                0 as *mut u8
            } else {
                unsafe { alloc::malloc(size) }.unwrap()
            },
            size,
        }
    }

    pub fn from_str(s: &str) -> String {
        let str_len = s.len() as u32;
        let ptr = unsafe { alloc::malloc::<u8>(str_len) }.unwrap();
        for i in 0..str_len {
            unsafe { *ptr.offset(i as isize) = s.as_bytes()[i as usize] };
        }
        String { ptr, size: str_len }
    }

    pub fn len(&self) -> u32 {
        self.size
    }

    pub fn eq(&self, other: &String) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if self.char_at(i) != other.char_at(i) {
                return false;
            }
        }

        true
    }

    pub fn char_at(&self, index: u32) -> Option<char> {
        if index >= self.size {
            return None;
        };
        Some(unsafe { *self.ptr.offset(index as isize) } as char)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe { alloc::free(self.ptr) };
    }
}
