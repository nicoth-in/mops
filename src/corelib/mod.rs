#![allow(dead_code)]
extern crate libc;

#[repr(C)]
struct CORES {

}

pub fn tar() {
    unsafe {
        let r = CORES {};
    };
}