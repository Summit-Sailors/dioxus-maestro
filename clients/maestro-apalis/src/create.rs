use {
	crate::acreate::acreate_apalis_storage,
	apalis::{postgres::PostgresStorage, prelude::Job},
	serde::{de::DeserializeOwned, Serialize},
	tokio::runtime::Runtime,
};

#[bon::builder]
pub fn create_apalis_storage_sync<T>(db_url: Option<&str>) -> PostgresStorage<T>
where
	T: Job + Serialize + DeserializeOwned,
{
	match tokio::runtime::Handle::try_current() {
		Ok(handle) => handle.block_on(acreate_apalis_storage().maybe_db_url(db_url).call()),
		Err(_) => Runtime::new().unwrap().block_on(acreate_apalis_storage().maybe_db_url(db_url).call()),
	}
}
