use dioxus::prelude::*;
use maestro_diesel::extensions::pagination::dtos::PaginatedResultDTO;

use crate::clients::db::DieselUser;

#[server(AFetchUsers)]
pub async fn afetch_users_paginated(page: i32, page_size: i32) -> Result<PaginatedResultDTO<DieselUser>, ServerFnError> {
	use diesel::{QueryDsl, SelectableHelper};
	use maestro_diesel::{async_client::from_server::extract_diesel_pool, extensions::pagination::paginate_async::PaginateAsync};

	use crate::clients::db::diesel_schema::users::dsl::*;

	let pool = extract_diesel_pool().await?;

	if pool.get().await.is_err() {
		return Err(ServerFnError::ServerError("An error occurred while getting connection from pool".to_string()));
	}

	let query = users.select(DieselUser::as_select());

	let paginated_results = query.paginate(page, page_size).aload_paginated::<DieselUser>(pool).await?;

	Ok(paginated_results)
}

#[server(FetchUsers)]
// sync version for comparison - should use sync client
pub async fn fetch_users_paginated(page: i32, page_size: i32) -> Result<PaginatedResultDTO<DieselUser>, ServerFnError> {
	use diesel::{QueryDsl, SelectableHelper};
	use maestro_diesel::{extensions::pagination::paginate_sync::Paginate, sync_client::create_db_pool_diesel};

	use crate::clients::db::diesel_schema::users::dsl::*;

	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

	// a synchronous pool
	let pool = create_db_pool_diesel(&db_url);
	if pool.get().is_err() {
		return Err(ServerFnError::ServerError("An error occurred while creating database connection".to_string()));
	}
	let mut conn = pool.get()?;

	let paginated_results = users.select(DieselUser::as_select()).paginate::<DieselUser>(page, page_size, &mut conn)?;

	Ok(paginated_results)
}
