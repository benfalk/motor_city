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

use std::{
    env,
    ffi::CString
};

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
pub fn db_url() -> CString {
    let bytes = env::var("DATABASE_URL").unwrap_or_default().into_bytes();
    unsafe { CString::from_vec_unchecked(bytes) }
}

#[no_mangle]
pub fn connection_ok() -> bool {
    // use self::models::*;
    // use self::schema::posts::dsl::*;
    // use self::diesel::prelude::*;
    let pool = DB_POOL.clone();
    let connection = pool.get();
    connection.is_ok()
}
