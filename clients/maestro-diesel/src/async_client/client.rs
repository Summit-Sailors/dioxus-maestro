use {
	super::AsyncDieselPool,
	diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool},
};

pub fn acreate_diesel_pool(connection_url: &str) -> AsyncDieselPool {
	deadpool::Pool::builder(AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(connection_url)).build().expect("db pool creation failed")
}
