use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

pub(crate) type Connection = PooledConnection<ConnectionManager<PgConnection>>;
pub(crate) type DbPool = Pool<ConnectionManager<PgConnection>>;

pub(crate) fn get_pool<S: Into<String>>(db_url: S) -> Result<DbPool, r2d2::Error> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager)
}

pub(crate) fn connection_from_pool(pool: *mut DbPool) -> Result<Connection, r2d2::Error> {
    unsafe { pool.as_ref() }.unwrap().get()
}
