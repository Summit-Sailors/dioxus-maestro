use dioxus::prelude::*;

use super::{AsyncDieselConn, AsyncDieselPool};

pub async fn extract_diesel_pool() -> Result<AsyncDieselPool, ServerFnError> {
	let FromContext(pool): FromContext<AsyncDieselPool> = extract().await.expect("There was a problem extracting connection pool from server");
	Ok(pool)
}

pub async fn extract_diesel_conn() -> Result<AsyncDieselConn, ServerFnError> {
	let FromContext(pool): FromContext<AsyncDieselPool> = extract().await?;
	Ok(pool.get().await.expect("cant get conn from pool"))
}
