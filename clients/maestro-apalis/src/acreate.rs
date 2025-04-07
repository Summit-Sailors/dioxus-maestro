use {
	apalis_sql::postgres::PostgresStorage,
	maestro_sqlx::acreate::acreate_sqlx_pool,
	serde::{Serialize, de::DeserializeOwned},
};

#[bon::builder]
pub async fn acreate_apalis_storage<T>(db_url: &str) -> PostgresStorage<T>
where
	T: Serialize + DeserializeOwned,
{
	let pool = acreate_sqlx_pool(db_url).await;
	PostgresStorage::setup(&pool).await.expect("apalis migrations failed");
	PostgresStorage::new(pool)
}
