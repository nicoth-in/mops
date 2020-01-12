#![allow(dead_code)]
extern crate libc;

extern {
    fn find_path() -> libc::c_int;
}

pub fn tar() {
    unsafe {
        find_path()
    };
}