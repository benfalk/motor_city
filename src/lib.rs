use std::ffi::CString;

#[no_mangle]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[no_mangle]
pub fn hello() -> CString {
    CString::new("hello world").unwrap()
}
