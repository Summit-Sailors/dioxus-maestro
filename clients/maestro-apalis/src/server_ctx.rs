pub use apalis::prelude::Storage;
use {apalis_sql::postgres::PostgresStorage, dioxus::prelude::*};

pub async fn apalis_storage_from_ctx<T>() -> Result<PostgresStorage<T>, ServerFnError>
where
	T: Sync + Send + 'static,
{
	let FromContext(storage): FromContext<PostgresStorage<T>> = extract().await?;
	Ok(storage)
}
