use {super::SqlxUser, dioxus::prelude::*};

#[server]
// synchronous user fetching with sync pool creation
pub async fn fetch_users_sync() -> Result<Vec<SqlxUser>, ServerFnError> {
	use {
		maestro_sqlx::create::create_sqlx_pool,
		sqlx::prelude::*,
		tokio::runtime::{Handle, Runtime},
	};

	let database_url = std::env::var("DATABASE_URL")?;
	let pool = create_sqlx_pool(database_url.as_str());

	let execute_query = async {
		let users = sqlx::query_as::<_, SqlxUser>(
			r#"
      SELECT id, username, email, bio, age, role, created_at, updated_at
      FROM users
      ORDER BY id
      LIMIT 10
      "#,
		)
		.fetch_all(&pool)
		.await?;

		Ok(users)
	};

	match Handle::try_current() {
		Ok(rt_handle) => rt_handle.block_on(execute_query),
		Err(_) => {
			let rt = Runtime::new()?;
			Ok(rt.block_on(execute_query)?)
		},
	}
}

#[server]
pub async fn fetch_users_async() -> Result<Vec<SqlxUser>, ServerFnError> {
	use {maestro_sqlx::acreate::acreate_sqlx_pool, sqlx::prelude::*};

	// creating a pool asynchronously
	let pool = acreate_sqlx_pool(std::env::var("DATABASE_URL")?.as_str()).await;
	let users = sqlx::query_as::<_, SqlxUser>(
		r#"
    SELECT id, username, email, bio, age, role, created_at, updated_at
    FROM users
    ORDER BY age
    LIMIT 10
    "#,
	)
	.fetch_all(&pool)
	.await?;

	Ok(users)
}
