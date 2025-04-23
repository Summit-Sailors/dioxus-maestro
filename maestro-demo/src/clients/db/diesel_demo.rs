#![allow(non_snake_case)]
use dioxus::prelude::*;
use tailwind_fuse::tw_join;

use crate::{
	clients::db::{DieselUser, apis::diesel_api},
	components::ui::features::Features,
};

#[component]
pub fn DieselDemo() -> Element {
	let mut users = use_signal(Vec::<DieselUser>::new);
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
				h1 { class: "text-[color:var(--text-color)] text-center text-3xl font-bold mb-2",
					"Maestro Diesel"
				}
				p { class: "text-[color:var(--muted-text)] text-center",
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
				div { class: "text-[color:var(--primary)] text-center animate-pulse",
					"Loading users..."
				}
			} else if let Some(err) = error() {
				div { class: "text-[color:var(--destructive)] text-center", "Error: {err}" }
			} else if users.is_empty() {
				div { class: "text-center text-[color:var(--muted-text)]", "No Users found" }
			} else {
				div { class: "flex flex-col items-center justify-center mt-4 w-full",
					h2 { class: "text-xl font-semibold mb-4 text-center text-[color:var(--text-color)]",
						{format!("Page ({})", current_page_idx() + 1)}
					}
					div { class: "w-full max-w-5xl mx-auto bg-[color:var(--card-bg)] p-6 border border-[color:var(--border-color)] rounded-2xl shadow-lg mb-6 max-h-[80vh] overflow-y-auto space-y-4",
						{
								users
										.iter()
										.map(|item| {
												rsx! {
													div {
														class: "border border-[color:var(--border-color)] rounded-xl p-4 bg-[color:var(--card-bg)] text-[color:var(--card-text)] shadow transition hover:shadow-lg hover:border-[color:var(--highlight-color)]",
														key: "{item.id}",
														p { class: "text-xl font-bold text-[color:var(--text-color)]", "{item.username}" }
														p { class: "text-sm text-[color:var(--muted-text)]", "{item.email:?}" }
														p { class: "text-sm text-[color:var(--muted-text)]", "{item.age.unwrap_or(0)} years old" }
														p { class: "text-sm text-[color:var(--text-color)] italic", "Role: {item.role:?}" }
													}
												}
										})
						}
					}
					// page navigation controls
					div { class: "flex items-center justify-center space-x-4 mt-4",
						button {
							class: tw_join!(
									"px-4 py-2 border rounded text-[color:var(--text-color)] border-[color:var(--border-color)]",
									if current_page_idx() == 0 { "opacity-50 cursor-not-allowed" } else {
									"hover:bg-[color:var(--hover-bg)]" }
							),
							disabled: current_page_idx() == 0,
							onclick: move |_| {
									current_page_idx -= 1;
							},
							"Prev Page"
						}
						span { class: "text-sm font-medium px-4 text-[color:var(--text-color)]",
							{format!("Page {}", current_page_idx() + 1)}
						}
						button {
							class: tw_join!(
									"px-4 py-2 border rounded text-[color:var(--text-color)] border-[color:var(--border-color)]",
									if current_page_idx() + 1 == total_pages() as i32 {
									"opacity-50 cursor-not-allowed" } else { "hover:bg-[color:var(--hover-bg)]" }
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
