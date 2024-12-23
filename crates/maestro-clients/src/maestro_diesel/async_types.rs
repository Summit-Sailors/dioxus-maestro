use diesel_async::{
	pooled_connection::{deadpool, AsyncDieselConnectionManager},
	AsyncPgConnection,
};

pub type DbConnAsync = AsyncDieselConnectionManager<AsyncPgConnection>;
pub type DbPoolAsync = deadpool::Pool<AsyncPgConnection>;
