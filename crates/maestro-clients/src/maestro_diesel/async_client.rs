use {
	super::async_types::DbPoolAsync,
	diesel_async::pooled_connection::{deadpool, AsyncDieselConnectionManager},
};

pub fn create_db_pool_diesel_async() -> DbPoolAsync {
	deadpool::Pool::builder(AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
		std::env::var("DATABASE_URL").expect("DATABASE_URL ENV VAR").as_str(),
	))
	.build()
	.expect("db pool creation failed")
}
