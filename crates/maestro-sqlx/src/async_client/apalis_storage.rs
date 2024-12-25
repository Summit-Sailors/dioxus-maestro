use {
	crate::pool::create_db_pool_sqlx,
	apalis::postgres::PostgresStorage,
	serde::{de::DeserializeOwned, Serialize},
};

pub async fn create_apalis_storage_async<T>() -> PostgresStorage<T>
where
	T: apalis::prelude::Job + Serialize + DeserializeOwned,
{
	let pool = create_db_pool_sqlx().await;
	PostgresStorage::setup(&pool).await.expect("apalis migrations failed");
	PostgresStorage::new(pool)
}
