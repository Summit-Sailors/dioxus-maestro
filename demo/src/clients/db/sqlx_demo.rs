use {crate::clients::db::SqlxUser, dioxus::prelude::*};

#[component]
pub fn SqlxDemo() -> Element {
	let mut users = use_signal(|| Vec::<SqlxUser>::new());
	let mut loading = use_signal(|| true);
	let mut error = use_signal(|| None::<String>);

	// 1. using async pool creation
	// preferred method when in an async context
	let aresult = use_server_future(move || crate::clients::db::sqlx_api::fetch_users_async())?;

	// users onmount - shows both sync and async methods
	use_effect(move || match aresult.state().cloned() {
		UseResourceState::Pending => {
			loading.set(true);
		},
		UseResourceState::Ready => {
			if let Some(Ok(users_result)) = &*aresult.value().read_unchecked() {
				users.set(users_result.to_vec());
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
    div { class: "container mx-auto p-6 flex flex-col items-center justify-center min-h-screen bg-gray-950 text-white",

      h1 { class: "text-3xl font-extrabold mb-6 text-center text-gray-200", "Maestro-SQLx Demo" }

      if loading() {
        div { class: "text-blue-400 text-center text-lg font-medium animate-pulse",
          "Loading users..."
        }
      } else if error().is_some() {
        div { class: "text-red-500 text-center text-lg font-semibold bg-gray-800 p-3 rounded-md shadow-md",
          "Error: {error():?}"
        }
      } else {
        div { class: "w-3/4 max-w-4xl bg-gray-900 p-6 border border-gray-700 rounded-lg mb-6 max-h-96 overflow-y-auto mx-auto shadow-lg",
          {
              let _ = users()
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
                  });
          }
        }
      }
    }
  }
}
