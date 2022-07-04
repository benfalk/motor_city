use std::{ffi::CString, os::raw::c_char};

#[derive(PartialEq)]
#[repr(u32)]
pub enum Status {
    Success = 0,
    Failure = 1,
}

#[repr(C)]
pub union ResultValue<T> {
    error_msg: *mut c_char,
    result: *mut T,
}

#[repr(C)]
pub struct RubyResult<T> {
    status: Status,
    value: ResultValue<T>,
}

pub fn ok<T>(result: T) -> *mut RubyResult<T> {
    let result = Box::into_raw(Box::new(result));

    RubyResult {
        status: Status::Success,
        value: ResultValue { result },
    }.to_ptr()
}

pub fn error<T>(message: &str) -> *mut RubyResult<T> {
    let error_msg = CString::new(message).unwrap().into_raw();

    RubyResult {
        status: Status::Failure,
        value: ResultValue { error_msg },
    }.to_ptr()
}

impl <T> RubyResult<T> {
    fn to_ptr(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn free(self) {
        if self.status != Status::Success {
            unsafe { CString::from_raw(self.value.error_msg) };
        }
    }
}
