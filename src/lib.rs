#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod models;
mod schema;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

use std::{env, ffi::CString};

lazy_static! {
    static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = {
        let url = env::var("DATABASE_URL").unwrap();
        let manager = ConnectionManager::<PgConnection>::new(url);

        Pool::builder()
            .build(manager)
            .expect("Could not build connection pool")
    };
}

#[no_mangle]
extern "C" fn db_url() -> *mut i8 {
    let bytes = env::var("DATABASE_URL").unwrap_or_default().into_bytes();
    let url = unsafe { CString::from_vec_unchecked(bytes) };
    url.into_raw()
}

#[no_mangle]
extern "C" fn connection_ok() -> bool {
    let pool = DB_POOL.clone();
    let connection = pool.get();
    connection.is_ok()
}

#[no_mangle]
extern "C" fn find_post(id_val: i32) -> *mut models::RubyPost {
    use self::diesel::prelude::*;
    use self::schema::posts::dsl::*;

    let pool = DB_POOL.clone();
    let connection = pool.get().unwrap();

    let maybe_post = posts.find(id_val).first::<models::Post>(&connection);

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
