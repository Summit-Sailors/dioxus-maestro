use {
	crate::async_client::apalis_storage::create_apalis_storage_async,
	apalis::{postgres::PostgresStorage, prelude::Job},
	serde::{de::DeserializeOwned, Serialize},
	tokio::runtime::Runtime,
};

pub fn create_apalis_storage_sync<T>() -> PostgresStorage<T>
where
	T: Job + Serialize + DeserializeOwned,
{
	match tokio::runtime::Handle::try_current() {
		Ok(handle) => handle.block_on(create_apalis_storage_async()),
		Err(_) => Runtime::new().unwrap().block_on(create_apalis_storage_async()),
	}
}
