use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

use std::env;

pub(crate) type Connection = PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = {
        let url = env::var("DATABASE_URL").unwrap();
        let manager = ConnectionManager::<PgConnection>::new(url);

        Pool::builder()
            .build(manager)
            .expect("Could not build connection pool")
    };
}

pub(crate) fn with_connection<F, R>(fun: F) -> R
where
    F: Fn(&Connection) -> R,
{
    let connection = DB_POOL.get().unwrap();
    fun(&connection)
}

pub(crate) fn with_connection_result<F, R>(fun: F) -> R
where
    F: Fn(Result<Connection, r2d2::Error>) -> R,
{
    fun(DB_POOL.get())
}
