use {
	super::sync_types::DbPoolSync,
	diesel::{r2d2, PgConnection},
};

pub fn create_db_pool_diesel() -> DbPoolSync {
	r2d2::Pool::builder()
		.build(r2d2::ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL").expect("DATABASE_URL ENV VAR").as_str()))
		.expect("db pool creation failed")
}
