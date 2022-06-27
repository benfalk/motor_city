#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod models;
mod schema;
mod conn_manager;

use conn_manager::*;
use std::{env, ffi::CString};

#[no_mangle]
extern "C" fn db_url() -> *mut i8 {
    let bytes = env::var("DATABASE_URL").unwrap_or_default().into_bytes();
    let url = unsafe { CString::from_vec_unchecked(bytes) };
    url.into_raw()
}

#[no_mangle]
extern "C" fn connection_ok() -> bool {
    with_connection_result(|result| result.is_ok())
}

#[no_mangle]
extern "C" fn find_post(id_val: i32) -> *mut models::RubyPost {
    use self::diesel::prelude::*;
    use self::schema::posts::dsl::*;

    let maybe_post = with_connection(|conn| {
        posts.find(id_val).first::<models::Post>(conn)
    });

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
