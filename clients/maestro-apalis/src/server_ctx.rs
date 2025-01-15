pub use apalis::prelude::Storage;
use {apalis::postgres::PostgresStorage, dioxus::prelude::*};

pub async fn apalis_storage_from_ctx<T>() -> Result<apalis::postgres::PostgresStorage<T>, ServerFnError>
where
	T: Sync + Send + 'static,
{
	let FromContext(storage): FromContext<PostgresStorage<T>> = extract().await?;
	Ok(storage)
}
