#[allow(non_snake_case)]
use {
	dioxus::prelude::*,
	maestro_hooks::pagination::use_pagination,
	serde::{Deserialize, Serialize},
	tailwind_fuse::tw_join,
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
fn DieselDemo() -> Element {
	let users = use_signal(|| Vec::<User>::new());
	let loading = use_signal(|| true);
	let error = use_signal(|| None::<String>);
	let total_items = use_signal(|| 0);

	// desired page size
	let page_size = 10;
	let (pagination, (mut next_idx, mut prev_idx, mut next_page, mut prev_page, mut set_page_size)) = use_pagination(total_items, page_size);

	// to fetch data when page changes
	use_effect(move || {
		spawn(async move {
			loading.set(true);

			match afetch_users_paginated(*pagination.page.read() + 1, *pagination.page_size.read()).await {
				Ok(paginated_result) => {
					users.set(paginated_result.records);
					// updating total items count - this could come from total_records in the response
					total_items.set(paginated_result.records.iter().len() as i32);
					loading.set(false);
				},
				Err(err) => {
					error.set(Some(format!("Error: {}", err)));
					loading.set(false);
				},
			}
		});
		()
	});

	// to get the currently focused user
	let current_user = use_memo(move || {
		let users_array = users();
		let current_idx = (pagination.idx)() % (pagination.page_size)();

		if users_array.len() > current_idx as usize {
			Some(users_array[current_idx as usize].clone())
		} else {
			None
		}
	});

	rsx! {
		div { class: "w-4/5 mx-auto p-4",
			h1 { class: "text-2xl font-bold mb-4", "Users with Pagination" }

			if loading() {
				div { class: "text-blue-500", "Loading users..." }
			} else if let Some(err) = error() {
				div { class: "text-red-500", "Error: {err}" }
			} else {
				div {
					// pagination info
					h2 { class: "text-xl font-semibold mb-2",
						{format!("Users ({})", pagination.counter_label)}
					}

					// item navigation controls
					div { class: "flex items-center space-x-2 mb-4",
						button {
							class: tw_join!(
									"px-3 py-1 border rounded", if * pagination.prev_idx_disabled.read() {
									"opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
							),
							disabled: *pagination.prev_idx_disabled.read(),
							onclick: move |_| prev_idx(),
							"Prev Item"
						}

						span { class: "text-sm", "Item: {*pagination.idx.read() + 1}" }

						button {
							class: tw_join!(
									"px-3 py-1 border rounded", if * pagination.next_idx_disabled.read() {
									"opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
							),
							disabled: *pagination.next_idx_disabled.read(),
							onclick: move |_| next_idx(),
							"Next Item"
						}
					}

					// currently focused user (if available)
					if let Some(user) = current_user() {
						div { class: "bg-yellow-50 p-4 border border-gray-700 rounded-lg mb-4",
							h3 { class: "text-lg font-medium", "Currently Selected: {user.username}" }
							p { class: "text-sm text-gray-600", "{user.email}" }
							p { class: "mt-1 text-sm", "{user.bio}" }
							p { class: "mt-1 text-xs text-gray-500", "Age: {user.age}" }
							span {
								class: tw_join!(
										"px-2 py-1 text-xs font-semibold rounded-full", match user.role { Role::Admin =>
										"bg-red-100 text-red-800", Role::Moderator => "bg-yellow-100 text-yellow-800",
										Role::User => "bg-green-100 text-green-800", }
								),
								"{user.role:?}"
							}
						}
					}

					// user list
					ul { class: "list-none divide-y divide-gray-200",
						{
								users()
										.iter()
										.enumerate()
										.map(|(i, user)| {
												let absolute_idx = *pagination.page.read() * *pagination.page_size.read()
														+ i as i32;
												let is_selected = absolute_idx == *pagination.idx.read();
												rsx! {
													li {
														key: "{user.username}",
														class: tw_join!(
																"py-4", if is_selected { "bg-blue-50 border-l-4 border-blue-500 pl-2" } else { ""
																}
														),
														div { class: "flex justify-between",
															div {
																p { class: "font-medium", "{user.username}" }
																p { class: "text-sm text-gray-600", "{user.email}" }
															}
															div {
																span {
																	class: tw_join!(
																			"px-2 py-1 text-xs font-semibold rounded-full", match user.role { Role::Admin =>
																			"bg-red-100 text-red-800", Role::Moderator => "bg-yellow-100 text-yellow-800",
																			Role::User => "bg-green-100 text-green-800", }
																	),
																	"{user.role:?}"
																}
															}
														}
														p { class: "mt-1 text-sm", "{user.bio}" }
														p { class: "mt-1 text-xs text-gray-500", "Age: {user.age}" }
														// button to select this specific item
														button {
															class: "mt-2 px-2 py-1 text-xs text-blue-600 hover:text-blue-800",
															onclick: move |_| {
																	pagination.idx.set(absolute_idx);
															},
															"Select"
														}
													}
												}
										})
						}
					}

					// page navigation controls
					div { class: "flex justify-between items-center mt-4",

						// prev
						button {
							class: tw_join!(
									"px-4 py-2 border rounded", if * pagination.prev_page_disabled.read() {
									"opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
							),
							disabled: *pagination.prev_page_disabled.read(),
							onclick: move |_| prev_page(),
							"Prev Page"
						}

						// page indicator
						span { class: "text-sm font-medium",
							{format!("Page {}", *pagination.page.read() + 1)}
						}

						// next
						button {
							class: tw_join!(
									"px-4 py-2 border rounded", if * pagination.next_page_disabled.read() {
									"opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
							),
							disabled: *pagination.next_page_disabled.read(),
							onclick: move |_| next_page(),
							"Next Page"
						}
					}
				}
			}
		}
	}
}

// server function to handle async pagination
#[server]
async fn afetch_users_paginated(page: i32, page_size: i32) -> Result<maestro_diesel::extensions::pagination::dtos::PaginatedResultDTO<User>, ServerFnError> {
	use maestro_diesel::{
		async_client::from_server::extract_diesel_pool,
		extensions::pagination::dtos::{PaginatedResultDTO, PaginationRequestDTO},
	};
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
		.aload_paginated::<User>(pool)
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
async fn fetch_users_paginated(page: i32, page_size: i32) -> Result<maestro_diesel::extensions::pagination::dtos::PaginatedResultDTO<User>, ServerFnError> {
	use maestro_diesel::{
		async_client::from_server::extract_diesel_pool,
		extensions::pagination::dtos::{PaginatedResultDTO, PaginationRequestDTO},
	};
	// connection from a synchronous pool
	let pool = extract_diesel_pool().await?;
	let mut conn = pool.get()?;

	// the sync pagination extension
	use maestro_diesel::extensions::pagination::paginate_sync::Paginate;

	let paginated_results = crate::clients::db::diesel_schema::users::table.paginate::<User>(page, page_size, &mut conn)?;

	// UserRecord to User model
	let users = paginated_results
		.records
		.into_iter()
		.map(|record| &User { username: record.username, email: record.email, bio: record.bio, age: record.age, role: record.role })
		.collect::<Vec<User>>();

	Ok(PaginatedResultDTO {
		records: users,
		total_pages: paginated_results.total_pages,
		has_more: paginated_results.has_more,
		current_page: paginated_results.current_page,
	})
}

#[server]
async fn acreate_user_in_transaction(new_user: User) -> Result<(), ServerFnError> {
	use {
		diesel_async::{AsyncConnection, RunQueryDsl},
		maestro_diesel::{
			async_client::client::acreate_diesel_pool,
			extensions::pagination::dtos::{PaginatedResultDTO, PaginationRequestDTO},
		},
	};

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
