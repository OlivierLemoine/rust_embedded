#![no_std]
#![allow(dead_code)]

#![feature(alloc)]
extern crate alloc;
extern crate allocator;
#[macro_use]
extern crate hal;

pub mod task;
pub mod net;

pub fn init(){
    task::init();
    net::init();
}