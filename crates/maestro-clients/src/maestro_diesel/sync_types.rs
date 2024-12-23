use diesel::{
	r2d2::{self, ConnectionManager, PooledConnection},
	PgConnection,
};

pub type DbPoolSync = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;
