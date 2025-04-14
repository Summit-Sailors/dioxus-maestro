use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool};

use super::AsyncDieselPool;

pub fn acreate_diesel_pool(connection_url: &str) -> AsyncDieselPool {
	deadpool::Pool::builder(AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(connection_url)).build().expect("db pool creation failed")
}
