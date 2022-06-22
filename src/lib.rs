#[macro_use]
extern crate diesel;

mod schema;
mod models;

use std::env;
use std::ffi::CString;

#[no_mangle]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[no_mangle]
pub fn db_url() -> CString {
    let bytes = env::var("DB_URL").unwrap_or_default().into_bytes();
    unsafe { CString::from_vec_unchecked(bytes) }
}
