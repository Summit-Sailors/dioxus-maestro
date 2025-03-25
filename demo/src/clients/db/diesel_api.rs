use {
	crate::clients::db::ClientsUser,
	dioxus::prelude::*,
	maestro_diesel::extensions::pagination::dtos::{PaginatedResultDTO, PaginationRequestDTO},
};

#[server(AFetchUsers)]
pub async fn afetch_users_paginated(page: i32, page_size: i32) -> Result<PaginatedResultDTO<ClientsUser>, ServerFnError> {
	use {
		crate::clients::db::{ClientsUser, diesel_schema::users::dsl::*},
		diesel::{QueryDsl, SelectableHelper, dsl::count_star},
		maestro_diesel::{async_client::from_server::extract_diesel_pool, extensions::pagination::paginate_async::PaginateAsync},
	};

	let pool = extract_diesel_pool().await?;
	let pagination_request = PaginationRequestDTO { page, page_size, query: () };

	let query = users.select((ClientsUser::as_select()));

	let paginated_results = query.paginate(page, page_size).aload_paginated::<ClientsUser>(pool).await?;

	Ok(paginated_results)
}

// #[server(FetchUsers)]
// // sync version for comparison - should use sync client
// async fn fetch_users_paginated(page: i32, page_size: i32) -> Result<PaginatedResultDTO<ClientsUser>, ServerFnError> {
// 	use maestro_diesel::{extensions::pagination::paginate_sync::Paginate, sync_client::create_db_pool_diesel};

// 	let db_url = std::env::var("DATABASE_URL").map_err(|e| ServerFnError::ServerError(e.to_string()))?;

// 	// a synchronous pool
// 	let pool = create_db_pool_diesel(&db_url);
// 	let mut conn = pool.get().map_err(|e| ServerFnError::ServerError(e.to_string()))?;

// 	let paginated_results = crate::clients::db::diesel_schema::users::table
// 		.paginate::<ClientsUser>(page, page_size, &mut conn)
// 		.map_err(|e| ServerFnError::ServerError(format!("Database error: {}", e)))?;

// 	Ok(paginated_results)
// }

// #[server(InsertUser)]
// async fn acreate_user_in_transaction(new_user: ClientsUser) -> Result<(), ServerFnError> {
// 	use {
// 		diesel_async::{AsyncConnection, RunQueryDsl},
// 		maestro_diesel::async_client::from_server::extract_diesel_pool,
// 	};
// 	let pool = extract_diesel_pool().await?;
// 	let mut conn = pool.get().await.map_err(|e| ServerFnError::ServerError(e.to_string()))?;

// 	let transaction_result = conn
// 		.transaction(|tx| {
// 			let fut = async move {
// 				diesel::insert_into(crate::clients::db::diesel_schema::users::table).values(&new_user).execute(tx).await?;
// 				// additional operations within the same transaction
// 				Ok::<_, diesel::result::Error>(())
// 			};
// 			Box::pin(fut)
// 		})
// 		.await;

// 	transaction_result.map_err(|e| ServerFnError::ServerError(format!("Transaction failed: {}", e)))
// }
