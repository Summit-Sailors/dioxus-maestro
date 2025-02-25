use {
	apalis::postgres::PostgresStorage,
	maestro_sqlx::acreate::acreate_sqlx_pool,
	serde::{de::DeserializeOwned, Serialize},
};

#[bon::builder]
pub async fn acreate_apalis_storage<T>(db_url: Option<&str>) -> PostgresStorage<T>
where
	T: apalis::prelude::Job + Serialize + DeserializeOwned,
{
	let pool = acreate_sqlx_pool(db_url.unwrap_or(std::env::var("APALIS_DATABASE_URL").unwrap().as_str())).await;
	PostgresStorage::setup(&pool).await.expect("apalis migrations failed");
	PostgresStorage::new(pool)
}
