use {
	crate::acreate::acreate_apalis_storage,
	apalis_sql::postgres::PostgresStorage,
	serde::{Serialize, de::DeserializeOwned},
	tokio::runtime::Runtime,
};

#[bon::builder(derive(Clone))]
pub fn create_apalis_storage_sync<T>(db_url: &str) -> PostgresStorage<T>
where
	T: Serialize + DeserializeOwned,
{
	match tokio::runtime::Handle::try_current() {
		Ok(handle) => handle.block_on(acreate_apalis_storage().db_url(db_url).call()),
		Err(_) => Runtime::new().unwrap().block_on(acreate_apalis_storage().db_url(db_url).call()),
	}
}
