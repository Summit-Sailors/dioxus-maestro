#[allow(non_snake_case)]
use {
	crate::clients::db::{DieselUser, diesel_api},
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
      h1 { class: "text-2xl font-bold mb-4 text-center", "Users with Pagination" }

      if loading() {
        div { class: "text-blue-500 text-center", "Loading users..." }
      } else if let Some(err) = error() {
        div { class: "text-red-500", "Error: {err}" }
      } else {
        div { class: "flex flex-col items-center justify-center mt-4 w-full",
          // pagination info
          h2 { class: "text-xl font-semibold mb-4 text-center",
            {format!("Page ({})", current_page_idx() + 1)}
          }
          // iser list
          div { class: "w-3/4 max-w-4xl bg-gray-900 p-4 border border-gray-700 rounded-lg mb-4 max-h-96 overflow-y-auto mx-auto",
            {
                users
                    .iter()
                    .map(|item| {
                        rsx! {
                          div {
                            class: "border border-slate-700 rounded-md p-3 text-slate-50 bg-gray-800 shadow-md text-center space-y-1",
                            key: "{item.id}",
                            p { class: "text-lg font-semibold", "{item.username}" }
                            p { class: "text-sm text-gray-300", "{item.email:?}" }
                            p { class: "text-sm text-gray-300", "{item.age.unwrap_or(0)}" }
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
              disabled: current_page_idx() + 1 == total_pages() as i32,
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
