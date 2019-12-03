pub use std::{
    io::{Read, Write},
    time,
};
pub use winapi::um::*;
pub fn test_bt() {

	winapi::um::fileapi::CreateFileW("COM3", winapi::um::winnt::GENERIC_READ, 0, winapi::shared::ntdef::NULL, winapi::um::fileapi::OPEN_EXISTING, 0, winapi::shared::ntdef::NULL);
}
