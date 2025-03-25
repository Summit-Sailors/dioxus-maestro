use log::info;
#[allow(non_snake_case)]
use {
	crate::clients::db::{ClientsRole, ClientsUser, diesel_api},
	dioxus::prelude::*,
	maestro_hooks::pagination::use_pagination,
	std::rc::Rc,
	tailwind_fuse::tw_join,
};

#[component]
pub fn DieselDemo() -> Element {
	let mut users = use_signal(|| Vec::<ClientsUser>::new());
	let mut loading = use_signal(|| true);
	let mut error = use_signal(|| None::<String>);
	let mut total_items = use_signal(|| 0);
	let mut total_pages = use_signal(|| 0);

	let _users_result = users();
	let users_result_clone = _users_result.clone();

	// desired page size
	let page_size = 10;
	let mut pagination = use_pagination(total_items, page_size);

	let server_result = use_server_future(move || diesel_api::afetch_users_paginated(*pagination.page.read() + 1, *pagination.page_size.read()))?;

	// to fetch data when page changes
	use_effect(move || match server_result.state().cloned() {
		UseResourceState::Pending => {
			loading.set(true);
		},
		UseResourceState::Ready => {
			if let Some(Ok(users_result)) = &*server_result.value().read_unchecked() {
				users.set(users_result.records.clone());
				total_items.set(users_result.total_pages as i32 * page_size)
			}
			loading.set(false);
			info!("{} users fetched so far", users_result_clone.len())
		},
		UseResourceState::Paused => {
			error.set(Some("Server function paused".to_string()));
		},
		UseResourceState::Stopped => {
			error.set(Some("Server function stopped".to_string()));
		},
	});

	// to get the currently focused user
	let current_user = use_memo(move || {
		let users_array = users();
		let current_idx = (pagination.idx)() % (pagination.page_size)();

		if users_array.len() > current_idx as usize { Some(users_array[current_idx as usize].clone()) } else { None }
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
									"opacity-50 cursor-not-allowed" } else { "hover:bg-gray-300" }
							),
							disabled: *pagination.prev_idx_disabled.read(),
							onclick: move |_| pagination.prev_idx(),
							"Prev Item"
						}

						span { class: "text-sm", "Item: {*pagination.idx.read() + 1}" }

						button {
							class: tw_join!(
									"px-3 py-1 border rounded", if * pagination.next_idx_disabled.read() {
									"opacity-50 cursor-not-allowed" } else { "hover:bg-gray-300" }
							),
							disabled: *pagination.next_idx_disabled.read(),
							onclick: move |_| pagination.next_idx(),
							"Next Item"
						}
					}

					// currently focused user (if available)
					if let Some(user) = current_user() {
						div { class: "bg-gray-900 border border-gray-700 p-4 rounded-lg shadow-md mb-4",
							h3 { class: "text-lg font-semibold text-blue-300",
								"Currently Selected: {user.username}"
							}
							p { class: "text-sm text-gray-400", "{user.email}" }
							p { class: "mt-1 text-sm text-gray-400",
								"{user.bio.clone().unwrap_or_else(|| \"No bio available\".to_string())}"
							}
							p { class: "mt-1 text-xs text-gray-400",
								"Age: {user.age.map_or(\"N/A\".to_string(), |a| a.to_string())}"
							}
							span {
								class: tw_join!(
										"px-2 py-1 text-xs font-semibold rounded-full", match user.role {
										ClientsRole::Admin => "bg-red-100 text-red-800", ClientsRole::Moderator =>
										"bg-yellow-100 text-yellow-800", ClientsRole::User =>
										"bg-green-100 text-green-800", }
								),
								"{user.role:?}"
							}
						}
					}

					// user list
					div { class: "w-3/4 bg-gray-900 p-4 border border-gray-700 rounded-lg mb-4 max-h-96 overflow-y-auto",
						{
								_users_result
										.iter()
										.enumerate()
										.skip(
												*pagination.page.read() as usize * *pagination.page_size.read() as usize,
										)
										.take(*pagination.page_size.read() as usize)
										.map(|(i, user)| {
												let absolute_idx = (*pagination.page.read()
														* *pagination.page_size.read()) + i as i32;
												let is_selected = *pagination.idx.read() == absolute_idx;
												rsx! {
													div {
														key: "{user.id}",
														class: tw_join!(
																"py-4 px-4 rounded-lg", if is_selected {
																"border-l-4 border-blue-500 shadow-md p-3" } else { "hover:bg-gray-500" }
														),
														div { class: "flex justify-between items-center",
															div {
																p { class: "font-medium text-lg", "{user.username}" }
																p { class: "text-sm text-gray-400", "{user.email}" }
															}
															div {
																span {
																	class: tw_join!(
																			"px-2 py-1 text-xs font-semibold rounded-full", match user.role {
																			ClientsRole::Admin => "bg-red-100 text-red-800", ClientsRole::Moderator =>
																			"bg-yellow-100 text-yellow-800", ClientsRole::User =>
																			"bg-green-100 text-green-800", }
																	),
																	"{user.role:?}"
																}
															}
														}
														p { class: "mt-2 text-sm text-gray-400",
															"{user.bio.clone().unwrap_or_else(|| \"No bio available\".to_string())}"
														}
														p { class: "mt-1 text-xs text-gray-400",
															"Age: {user.age.map_or(\"N/A\".to_string(), |a| a.to_string())}"
														}
														button {
															class: "mt-3 px-3 py-1 text-xs text-blue-600 hover:text-blue-800 border border-blue-400 rounded-md transition",
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
							onclick: move |_| pagination.prev_page(),
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
							onclick: move |_| pagination.next_page(),
							"Next Page"
						}
					}
				}
			}
		}
	}
}
