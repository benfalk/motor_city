#[macro_use]
extern crate diesel;

mod conn_manager;
mod result;
mod models;
mod schema;

use conn_manager::*;
use result::RubyResult;
use std::{ffi::CString, os::raw::c_char};

#[no_mangle]
extern "C" fn connection_ok(pool: *mut DbPool) -> bool {
    if pool.is_null() {
        return false;
    }

    let result = connection_from_pool(pool);

    result.is_ok()
}

#[no_mangle]
extern "C" fn establish_connection(db_url: *mut c_char) -> *mut RubyResult {
    let db_url = unsafe { CString::from_raw(db_url) };

    let db_url = match db_url.clone().into_string() {
        Err(_) => return result::error::<DbPool>("cannot convert db url"),
        Ok(string) => {
            let _ = db_url.into_raw();
            string
        }
    };

    match get_pool(db_url) {
        Err(err) =>  result::error::<DbPool>(&format!("{err}")),
        Ok(pool) => result::ok(pool),
    }
}

#[no_mangle]
extern "C" fn find_post_with_pool(id_val: i32, pool: *mut DbPool) -> *mut models::RubyPost {
    use self::diesel::prelude::*;
    use self::schema::posts::dsl::*;

    let conn = connection_from_pool(pool).unwrap();

    let maybe_post = posts.find(id_val).first::<models::Post>(&conn);

    if maybe_post.is_ok() {
        return Box::into_raw(Box::new(maybe_post.unwrap().into()));
    }

    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn free_post(post_ptr: *mut models::RubyPost) {
    if post_ptr.is_null() {
        return;
    }

    unsafe {
        let post: Box<models::RubyPost> = Box::from_raw(post_ptr);
        let title = CString::from_raw(post.title);
        let body = CString::from_raw(post.body);
        std::mem::drop(title);
        std::mem::drop(body);
    }
}

#[no_mangle]
extern "C" fn free_result(result_ptr: *mut RubyResult) {
    let result = unsafe { Box::from_raw(result_ptr) };
    result.free();
}
