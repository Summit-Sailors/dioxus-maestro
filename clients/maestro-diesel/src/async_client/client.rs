use {
	super::types::DbPoolAsync,
	diesel_async::pooled_connection::{deadpool, AsyncDieselConnectionManager},
};

pub fn create_db_pool_diesel_async(connection_url: &str) -> DbPoolAsync {
	deadpool::Pool::builder(AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(connection_url)).build().expect("db pool creation failed")
}
