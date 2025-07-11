use diesel::{
	PgConnection,
	r2d2::{ConnectionManager, Pool, PooledConnection},
};

pub type DieselPool = Pool<ConnectionManager<PgConnection>>;
pub type DieselConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_db_pool_diesel(db_url: &str) -> DieselPool {
	Pool::builder().build(ConnectionManager::<PgConnection>::new(db_url)).expect("db pool creation failed")
}
