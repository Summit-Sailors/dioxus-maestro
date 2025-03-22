#[allow(non_snake_case)]
use {
	crate::clients::db::{ClientsRole, ClientsUser},
	dioxus::prelude::*,
	maestro_hooks::pagination::use_pagination,
	serde::{Deserialize, Serialize},
	tailwind_fuse::tw_join,
};

#[cfg(feature = "server")]
use crate::clients::db::api;

#[component]
fn DieselDemo() -> Element {
	let users = use_signal(|| Vec::<ClientsUser>::new());
	let loading = use_signal(|| true);
	let error = use_signal(|| None::<String>);
	let total_items = use_signal(|| 0);

	// desired page size
	let page_size = 10;
	let pagination = use_pagination(total_items, page_size);

	// to fetch data when page changes
	use_effect(move || {
		spawn(async move {
			loading.set(true);

			#[cfg(feature = "server")]
			let server_result = use_server_future(move || api::afetch_users_paginated(*pagination.page.read() + 1, *pagination.page_size.read()))?;
			if let Some(result) = &*server_result.read_unchecked() {
				match result {
					Ok(paginated_result) => {
						users.set(paginated_result.records.clone());
						// assuming total_records exists in your DTO
						total_items.set(paginated_result.total_pages as i32);
						loading.set(false);
					},
					Err(err) => {
						error.set(Some(format!("Error fetching users: {}", err)));
						loading.set(false);
					},
				}
			} else {
				error.set(Some(format!("No response from server")));
				loading.set(false);
			}
		});
		()
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
                  "opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
              ),
              disabled: *pagination.prev_idx_disabled.read(),
              onclick: move |_| pagination.prev_idx(),
              "Prev Item"
            }

            span { class: "text-sm", "Item: {*pagination.idx.read() + 1}" }

            button {
              class: tw_join!(
                  "px-3 py-1 border rounded", if * pagination.next_idx_disabled.read() {
                  "opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
              ),
              disabled: *pagination.next_idx_disabled.read(),
              onclick: move |_| pagination.next_idx(),
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
          ul { class: "list-none divide-y divide-gray-200",
            {
                let _ = users()
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
                                      "px-2 py-1 text-xs font-semibold rounded-full", match user.role {
                                      ClientsRole::Admin => "bg-red-100 text-red-800", ClientsRole::Moderator =>
                                      "bg-yellow-100 text-yellow-800", ClientsRole::User =>
                                      "bg-green-100 text-green-800", }
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
                    });
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
