use {
	super::{AsyncDieselConn, AsyncDieselPool},
	dioxus::prelude::*,
};

pub async fn extract_diesel_pool() -> Result<AsyncDieselPool, ServerFnError> {
	let FromContext(pool): FromContext<AsyncDieselPool> = extract().await?;
	Ok(pool)
}

pub async fn extract_diesel_conn() -> Result<AsyncDieselConn, ServerFnError> {
	let FromContext(pool): FromContext<AsyncDieselPool> = extract().await?;
	Ok(pool.get().await.expect("cant get conn from pool"))
}
