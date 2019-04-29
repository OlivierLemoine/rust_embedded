#![allow(dead_code)]

use super::alloc;

pub struct Vec<T> {
    ptr: *mut T,
    capacity: u32,
    current_size: u32,
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T> {
        Vec {
            ptr: unsafe { alloc::malloc(1) }.unwrap(),
            capacity: 1,
            current_size: 0,
        }
    }

    pub fn at(&mut self, index: u32) -> Option<*mut T> {
        if index >= self.current_size {
            None
        } else {
            Some(unsafe { self.ptr.offset(index as isize) })
        }
    }

    pub fn push(&mut self, v: T) {
        self.current_size += 1;
        if self.current_size >= self.capacity {
            unsafe { self.ptr = alloc::realloc(self.ptr, self.capacity * 2).unwrap() };
        }
        unsafe { *self.ptr.offset(self.current_size as isize) = v };
    }

    pub fn pop(&mut self) -> Option<*mut T> {
        if self.current_size == 0 {
            None
        } else {
            self.current_size -= 1;
            Some(unsafe { self.ptr.offset(self.current_size as isize) })
        }
    }
}
