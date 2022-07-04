use std::{ffi::CString, os::raw::c_char};

#[derive(PartialEq)]
#[repr(u32)]
pub enum Status {
    Success = 0,
    Failure = 1,
}

#[repr(C)]
pub struct RubyResult {
    status: Status,
    value: *mut u8,
}

pub fn ok<T>(result: T) -> *mut RubyResult {
    let result = Box::into_raw(Box::new(result));

    RubyResult {
        status: Status::Success,
        value: result as *mut u8,
    }.to_ptr()
}

pub fn error<T>(message: &str) -> *mut RubyResult {
    let error_msg = CString::new(message).unwrap().into_raw();

    RubyResult {
        status: Status::Failure,
        value: error_msg as *mut u8,
    }.to_ptr()
}

impl RubyResult {
    fn to_ptr(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn free(self) {
        if self.status != Status::Success {
            unsafe { CString::from_raw(self.value as *mut c_char) };
        }
    }
}
