pub use crate::SqlxPgPool;

pub async fn acreate_sqlx_pool(db_url: &str) -> SqlxPgPool {
	SqlxPgPool::connect(db_url).await.expect("Couldn't connect to the database")
}
