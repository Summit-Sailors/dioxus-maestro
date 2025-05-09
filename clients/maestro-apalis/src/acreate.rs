#[allow(unused_imports)]
use apalis_core::backend::BackendExpose;
use apalis_sql::postgres::PostgresStorage;
use maestro_sqlx::acreate::acreate_sqlx_pool;
use serde::{Serialize, de::DeserializeOwned};

#[bon::builder]
pub async fn acreate_apalis_storage<T>(db_url: &str) -> PostgresStorage<T>
where
	T: Serialize + DeserializeOwned,
{
	let pool = acreate_sqlx_pool(db_url).await;
	PostgresStorage::setup(&pool).await.expect("Apalis migrations failed");
	PostgresStorage::new(pool)
}
