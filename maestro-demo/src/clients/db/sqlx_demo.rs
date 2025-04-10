use {
	crate::{clients::db::SqlxUser, components::ui::features::Features},
	dioxus::prelude::*,
};

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
				h1 { class: "text-slate-100 text-center text-2xl sm:text-3xl lg:text-4xl 2xl:text-5xl font-semibold",
					"Maestro SQLx"
				}
				p { class: "text-slate-300 text-center text-base lg:text-xl 2xl:text-2xl",
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
				div { class: "text-blue-500 text-center animate-pulse", "Loading users..." }
			} else if let Some(err) = error() {
				div { class: "text-red-500 text-center", "Error: {err}" }
			} else {
				div { class: "flex flex-col items-center justify-center mt-4 w-full",
					span { {format!("{} users fetched", users_len)} }
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
				}
			}
		}
	}
}
