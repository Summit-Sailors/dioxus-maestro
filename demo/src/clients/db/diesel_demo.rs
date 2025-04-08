#[allow(non_snake_case)]
use {
	crate::{
		clients::db::{DieselUser, apis::diesel_api},
		components::ui::features::Features,
	},
	dioxus::prelude::*,
	tailwind_fuse::tw_join,
};

#[component]
pub fn DieselDemo() -> Element {
	let mut users = use_signal(|| Vec::<DieselUser>::new());
	let mut loading = use_signal(|| true);
	let mut error = use_signal(|| None::<String>);
	let mut total_pages = use_signal(|| 0);

	let mut current_page_idx = use_signal(|| 0);

	// desired page size
	let page_size = 10;

	let server_result = use_server_future(move || diesel_api::afetch_users_paginated(current_page_idx() + 1, page_size))?;

	// to fetch data when page changes
	use_effect(move || match server_result.state().cloned() {
		UseResourceState::Pending => {
			loading.set(true);
		},
		UseResourceState::Ready => {
			if let Some(Ok(users_result)) = &*server_result.value().read_unchecked() {
				users.set(users_result.records.clone());
				total_pages.set(users_result.total_pages);
			} else {
				error.set(Some("An error occurred when fetching users".to_string()))
			}
			loading.set(false);
		},
		UseResourceState::Paused => {
			error.set(Some("Server function paused".to_string()));
		},
		UseResourceState::Stopped => {
			error.set(Some("Server function stopped".to_string()));
		},
	});

	rsx! {
		div { class: "w-full mx-auto p-4",
			div { class: "flex flex-col gap-3",
				h1 { class: "text-slate-100 text-center text-2xl sm:text-3xl lg:text-4xl 2xl:text-5xl font-semibold",
					"Maestro Diesel"
				}
				p { class: "text-slate-300 text-center text-base lg:text-xl 2xl:text-2xl",
					"A diesel utility equipped with both sync and async database connection/pool creation and an extension for paginated queries"
				}
			}

			div { id: "maestro-diesel-features", class: "flex space-x-2 mt-4",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Pagination support. It is possible to fetch paginated resultsfrom the database"
									.to_string(),
							"Asynchronous database connection pool creation".to_string(),
							"Synchronous database connection pool creation".to_string(),
							"Database pool retrieval from server context".to_string(),
							"Database connection retrieval from server context".to_string(),
							"Simple integration with Dioxus".to_string(),
					],
				}
			}

			if loading() {
				div { class: "text-blue-500 text-center", "Loading users..." }
			} else if let Some(err) = error() {
				div { class: "text-red-500 text-center", "Error: {err}" }
			} else if users.is_empty() {
				div { class: "text-center text-gray-50", "No Users found" }
			} else {
				div { class: "flex flex-col items-center justify-center mt-4 w-full",
					// pagination info
					h2 { class: "text-xl font-semibold mb-4 text-center",
						{format!("Page ({})", current_page_idx() + 1)}
					}
					// iser list
					div { class: "w-full max-w-5xl mx-auto bg-gray-900 p-6 border border-gray-700 rounded-2xl shadow-lg mb-6 max-h-[80vh] overflow-y-auto space-y-4",
						{
								users
										.iter()
										.map(|item| {
												rsx! {
													div {
														class: "border border-slate-700 rounded-xl p-4 bg-gray-800 text-slate-100 shadow transition hover:shadow-lg hover:border-slate-500",
														key: "{item.id}",
														p { class: "text-xl font-bold text-white", "{item.username}" }
														p { class: "text-sm text-gray-300", "{item.email:?}" }
														p { class: "text-sm text-gray-300", "{item.age.unwrap_or(0)} years old" }
														p { class: "text-sm text-gray-400 italic", "Role: {item.role:?}" }
													}
												}
										})
						}
					}
					// page navigation controls
					div { class: "flex items-center justify-center space-x-4 mt-4",
						// prev Button
						button {
							class: tw_join!(
									"px-4 py-2 border rounded", if current_page_idx() == 0 {
									"opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
							),
							disabled: current_page_idx() == 0,
							onclick: move |_| {
									current_page_idx -= 1;
							},
							"Prev Page"
						}
						// page Indicator
						span { class: "text-sm font-medium px-4",
							{format!("Page {}", current_page_idx() + 1)}
						}
						// next Button
						button {
							class: tw_join!(
									"px-4 py-2 border rounded", if current_page_idx() + 1 == total_pages() as i32 {
									"opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
							),
							disabled: current_page_idx() + 1 == total_pages() as i32 || users.len() == 0,
							onclick: move |_| {
									current_page_idx += 1;
							},
							"Next Page"
						}
					}
				}
			}
		}
	}
}
