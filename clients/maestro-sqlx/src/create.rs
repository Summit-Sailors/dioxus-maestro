use tokio::runtime::{Handle, Runtime};

pub use crate::SqlxPgPool;
use crate::acreate::acreate_sqlx_pool;

pub fn create_sqlx_pool(db_url: &str) -> SqlxPgPool {
	match Handle::try_current() {
		Ok(handle) => handle.block_on(acreate_sqlx_pool(db_url)),
		Err(_) => Runtime::new().unwrap().block_on(acreate_sqlx_pool(db_url)),
	}
}
