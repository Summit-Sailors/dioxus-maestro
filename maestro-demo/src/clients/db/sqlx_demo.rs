use dioxus::prelude::*;

use crate::{clients::db::SqlxUser, components::ui::features::Features};

#[component]
pub fn SqlxDemo() -> Element {
	let mut users = use_signal(Vec::<SqlxUser>::new);
	let mut loading = use_signal(|| true);
	let mut error = use_signal(|| None::<String>);

	let users_len = users.len() as i32;

	// 1. using async pool creation
	// preferred method when in an async context
	let aresult = use_server_future(crate::clients::db::apis::sqlx_api::fetch_users_async)?;

	// users onmount - shows both sync and async methods
	use_effect(move || match aresult.state().cloned() {
		UseResourceState::Pending => {
			loading.set(true);
		},
		UseResourceState::Ready => {
			if let Some(Ok(users_result)) = &*aresult.value().read_unchecked() {
				users.set(users_result.to_vec());
			} else {
				error.set(Some("An error occurred while fetching users".to_string()));
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
					"Maestro SQLx"
				}
				p { class: "text-[color:var(--muted-text)] text-center",
					"A sqlx utility equipped with both sync and async database connection/pool creation"
				}
			}

			div { id: "maestro-sqlx-features", class: "flex space-x-2 mt-4",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Asynchronous database connection pool creation".to_string(),
							"Synchronous database connection pool creation".to_string(),
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
			} else {
				div { class: "flex flex-col items-center justify-center mt-4 w-full",
					h2 { class: "text-xl font-semibold mb-4 text-center text-[color:var(--text-color)]",
						{format!("{users_len} users fetched")}
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
				}
			}
		}
	}
}
