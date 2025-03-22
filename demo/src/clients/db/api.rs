use {crate::clients::db::ClientsUser, dioxus::prelude::*};
#[cfg(feature = "server")]
use {
	diesel_async::{AsyncConnection, RunQueryDsl},
	maestro_diesel::{
		async_client::{client::acreate_diesel_pool, from_server::extract_diesel_pool},
		extensions::pagination::dtos::{PaginatedResultDTO, PaginationRequestDTO},
	},
};

// server function to handle async pagination
#[server]
pub async fn afetch_users_paginated(page: i32, page_size: i32) -> Result<PaginatedResultDTO<ClientsUser>, ServerFnError> {
	// the pool from the Dioxus context
	let pool = extract_diesel_pool().await?;

	// pagination request with validation
	let pagination_request = PaginationRequestDTO {
		page,
		page_size,
		query: (), // no additional query
	};

	// the async pagination extension
	use maestro_diesel::extensions::pagination::paginate_async::PaginateAsync;

	// the diesel query builder with pagination
	let query = diesel::QueryDsl::into_boxed(crate::clients::db::diesel_schema::users::table);

	// applying pagination using the PaginateAsync trait
	let paginated_results = query
		.paginate(pagination_request.page, pagination_request.page_size)
		.aload_paginated::<ClientsUser>(pool)
		.await
		.map_err(|e| ServerFnError::ServerError(format!("Database error: {}", e)))?;

	Ok(PaginatedResultDTO {
		records: paginated_results.records,
		total_pages: paginated_results.total_pages,
		has_more: paginated_results.has_more,
		current_page: paginated_results.current_page,
	})
}

#[server]
// sync version for comparison
async fn fetch_users_paginated(page: i32, page_size: i32) -> Result<PaginatedResultDTO<ClientsUser>, ServerFnError> {
	// connection from a synchronous pool
	let pool = extract_diesel_pool().await?;
	let mut conn = pool.get()?;

	// the sync pagination extension
	use maestro_diesel::extensions::pagination::paginate_sync::Paginate;

	let paginated_results = crate::clients::db::diesel_schema::users::table.paginate::<ClientsUser>(page, page_size, &mut conn)?;

	// UserRecord to ClientsUser model
	let users = paginated_results
		.records
		.into_iter()
		.map(|record| ClientsUser { username: record.username, email: record.email, bio: record.bio, age: record.age, role: record.role })
		.collect::<Vec<ClientsUser>>();

	Ok(PaginatedResultDTO {
		records: users,
		total_pages: paginated_results.total_pages,
		has_more: paginated_results.has_more,
		current_page: paginated_results.current_page,
	})
}

#[server]
async fn acreate_user_in_transaction(new_user: ClientsUser) -> Result<(), ServerFnError> {
	let pool = acreate_diesel_pool(std::env::var("DATABASE_URL")?.as_str());
	let mut conn = pool.get().await.map_err(|e| ServerFnError::ServerError(e.to_string()))?;

	let transaction_result = conn
		.transaction(|tx| {
			let fut = async move {
				diesel::insert_into(crate::clients::db::diesel_schema::users::dsl::users).values(&new_user).execute(tx).await?;

				// additional operations within the same transaction

				Ok::<_, diesel::result::Error>(())
			};
			Box::pin(fut)
		})
		.await;

	transaction_result.map_err(|e| ServerFnError::ServerError(format!("Transaction failed: {}", e)))
}
