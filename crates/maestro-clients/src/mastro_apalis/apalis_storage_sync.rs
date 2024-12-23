use {
	super::apalis_storage_async::create_apalis_storage_async,
	apalis::{postgres::PostgresStorage, prelude::Job},
	serde::{de::DeserializeOwned, Serialize},
	tokio::runtime::Runtime,
};

pub fn create_apalis_storage_sync<T: Job + Serialize + DeserializeOwned>() -> PostgresStorage<T> {
	match tokio::runtime::Handle::try_current() {
		Ok(handle) => handle.block_on(create_apalis_storage_async()),
		Err(_) => Runtime::new().unwrap().block_on(create_apalis_storage_async()),
	}
}
