use {crate::clients::db::SqlxUser, dioxus::prelude::*};

#[component]
pub fn SqlxDemo() -> Element {
	let mut users = use_signal(|| Vec::<SqlxUser>::new());
	let mut loading = use_signal(|| true);
	let mut error = use_signal(|| None::<String>);

	let users_len = users.len() as i32;

	// 1. using async pool creation
	// preferred method when in an async context
	let aresult = use_server_future(move || crate::clients::db::apis::sqlx_api::fetch_users_async())?;

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
		div { class: "w-4/5 mx-auto p-4",

			h1 { class: "text-2xl font-bold mb-4 text-center", "Maestro-SQLx Demo" }

			if loading() {
				div { class: "text-blue-500 text-center animate-pulse", "Loading users..." }
			} else if let Some(err) = error() {
				div { class: "text-red-500 text-center", "Error: {err}" }
			} else {
				div { class: "flex flex-col items-center justify-center mt-4 w-full",
					span { {format!("{} users fetched", users_len)} }
					div { class: "w-3/4 max-w-4xl bg-gray-900 p-4 border border-gray-700 rounded-lg mb-4 max-h-96 overflow-y-auto mx-autog",
						{
								users
										.iter()
										.map(|item| {
												rsx! {
													div {
														class: "border border-slate-700 rounded-md p-4 text-slate-50 bg-gray-800 shadow-md text-center space-y-2 mb-3",
														key: "{item.id}",
														p { class: "text-xl font-bold text-gray-100", "Name: {item.username}" }
														p { class: "text-sm text-gray-300", "Email: {item.email:?}" }
														p { class: "text-sm text-gray-300", "Age: {item.age.unwrap_or(0)}" }
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
