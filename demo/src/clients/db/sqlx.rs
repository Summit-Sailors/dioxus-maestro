use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct User {
	username: String,
	email: String,
	bio: String,
	age: i32,
	role: Role,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
enum Role {
	Admin,
	User,
	Moderator,
}

#[component]
fn SqlxDemo() -> Element {
	let mut users = use_signal(|| Vec::<User>::new());
	let mut loading = use_signal(|| true);
	let mut error = use_signal(|| "".to_string());

	// users onmount - shows both sync and async methods
	use_effect(move || {
		spawn(async move {
			// 1. using async pool creation
			// preferred method when in an async context
			let aresult = fetch_users_async().await;

			match aresult {
				Ok(fetched_users) => {
					users.set(fetched_users);
					loading.set(false);
				},
				Err(err) => {
					error.set(format!("Async fetch error: {}", err));
					loading.set(false);
				},
			}

			let result = fetch_users_sync().await;

			// the sync approach
			match result {
				Ok(sync_users) => {
					println!("Sync fetch returned {} users", sync_users.len());
				},
				Err(err) => {
					println!("Sync fetch resulted in error: {}", err);
				},
			}
		});
		()
	});

	rsx! {
		div { class: "container mx-auto p-4",
			h1 { class: "text-2xl font-bold mb-4", "Maestro-SQLx Demo" }

			if loading() {
				div { class: "text-blue-500", "Loading users..." }
			} else if !error().is_empty() {
				div { class: "text-red-500", "Error: {error()}" }
			} else {
				div {
					h2 { class: "text-xl font-semibold mb-2", "Users" }
					ul { class: "list-disc pl-6",
						{
								users
										.read()
										.iter()
										.map(|user| {
												rsx! {
													li { key: "{user.email}",
														p { "Username: {user.username}" }
														p { "Email: {user.email}" }
														p { "Role: {user.role}" }
													}
												}
										})
						}
					}
				}
			}
		}
	}
}

#[server]
// synchronous user fetching with sync pool creation
async fn fetch_users_sync() -> Result<Vec<User>, ServerFnError> {
	use {
		maestro_sqlx::create::create_sqlx_pool,
		sqlx::Row,
		tokio::runtime::{Handle, Runtime},
	};

	let database_url = std::env::var("DATABASE_URL").map_err(ServerFnError::from)?;
	let pool = create_sqlx_pool(database_url.as_str());

	let execute_query = async {
		let users: Vec<User> = sqlx::query_as(
			r#"
          SELECT id, username, email, bio, age, role
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
		Err(e) => {
			let rt = Runtime::new()?;
			rt.block_on(execute_query).map_err(ServerFnError::from(e))
		},
	}
}

// demonstrates real-time user monitoring with SQL LISTEN/NOTIFY
#[server]
async fn listen_for_user_changes() -> Result<Vec<User>, ServerFnError> {
	use {maestro_sqlx::acreate::acreate_sqlx_pool, tokio_stream::StreamExt};

	let pool = acreate_sqlx_pool(std::env::var("DATABASE_URL")?.as_str()).await;

	let mut listener = sqlx::postgres::PgListener::connect_with(&pool).await?;

	// listen for the 'user_changed' notification
	listener.listen("user_changed").await?;

	// process notifications - in a real app you'd handle this as a stream
	let mut users = Vec::new();

	// get first 5 notifications (or timeout after 10 seconds)
	let mut stream = listener.into_stream();
	let timeout = std::time::Duration::from_secs(10);

	for _ in 0..5 {
		if let Some(notification) = tokio::time::timeout(timeout, stream.next()).await.ok().flatten() {
			// parse user ID from notification payload
			let user_id: i64 = notification.unwrap().payload().parse()?;

			// fetch updated user data
			let user = fetch_user_by_id(pool, user_id).await?;
			users.push(user);
		} else {
			break;
		}
	}

	Ok(users)
}

#[server]
async fn fetch_user_by_id(pool: sqlx::PgPool, id: i64) -> Result<User, ServerFnError> {
	let row: User = sqlx::query_as(
		r#"
        SELECT id, username, email, bio, age, role
        FROM users
        WHERE id = $1
        "#,
	)
	.fetch_one(&pool)
	.await?;

	Ok(row)
}

#[server]
pub async fn create_user(user: User, pool: sqlx::PgPool) -> Result<(), ServerFnError> {
	sqlx::query::<Postgres>(
		"INSERT INTO users (username, email, age, bio, role) VALUES ($1, $2, $3, $4, $5)
    ",
	)
	.bind(user.username)
	.bind(user.email)
	.bind(user.age)
	.bind(user.bio)
	.bind(user.role)
	.execute(&pool)
	.await?;
	Ok(())
}

#[server]
async fn fetch_users_async() -> Result<Vec<User>, ServerFnError> {
	use maestro_sqlx::acreate::acreate_sqlx_pool;

	// creating a pool asynchronously
	let pool = acreate_sqlx_pool(std::env::var("DATABASE_URL")?.as_str()).await;
	let users: Vec<sqlx::Users> = sqlx::query_as(
		r#"
      SELECT username, email, bio, age, role
      FROM users
      ORDER by age
      LIMIT 10 
    "#,
	)
	.fetch_all(&pool)
	.await?;

	Ok(users)
}
