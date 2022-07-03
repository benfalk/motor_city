#[macro_use]
extern crate diesel;

mod conn_manager;
mod models;
mod schema;

use conn_manager::*;
use std::{env, ffi::CString, os::raw::c_char};

#[no_mangle]
extern "C" fn db_url() -> *mut i8 {
    let bytes = env::var("DATABASE_URL").unwrap_or_default().into_bytes();
    let url = unsafe { CString::from_vec_unchecked(bytes) };
    url.into_raw()
}

#[no_mangle]
extern "C" fn connection_ok(pool: *mut DbPool) -> bool {
    if pool.is_null() {
        return false;
    }

    let result = connection_from_pool(pool);

    result.is_ok()
}

#[no_mangle]
extern "C" fn establish_connection(db_url: *mut c_char) -> *mut DbPool {
    let db_url = unsafe { CString::from_raw(db_url) };

    let db_url = match db_url.clone().into_string() {
        Err(_) => return std::ptr::null_mut(),
        Ok(string) => {
            let _ = db_url.into_raw();
            string
        }
    };

    match get_pool(db_url) {
        Err(_) => std::ptr::null_mut(),
        Ok(pool) => Box::into_raw(Box::new(pool)),
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
