use {
	apalis::postgres::PostgresStorage,
	maestro_sqlx::acreate::acreate_sqlx_pool,
	serde::{de::DeserializeOwned, Serialize},
};

#[bon::builder]
pub async fn acreate_apalis_storage<T>(db_url: Option<&str>) -> PostgresStorage<T>
where
	T: apalis::prelude::Job + Serialize + DeserializeOwned,
{
	let pool = acreate_sqlx_pool(db_url.unwrap_or(std::env::var("APALIS_DATABASE_URL").unwrap().as_str())).await;
	PostgresStorage::setup(&pool).await.expect("apalis migrations failed");
	PostgresStorage::new(pool)
}



rsx! {
  div {
    class: "hooks-demo bg-gray-900 p-6 rounded-lg shadow-lg space-y-6",

    div { class: "mb-8",
      h1 { class: "text-gray-100 text-center text-3xl font-bold mb-2", "Maestro Hooks" }
      p { class: "text-gray-300 text-center",
        "Enhanced hooks collection for Dioxus applications that provides type safety, cross-platform compatibility, and optimized performance."
      }
    }

    div {
      class: "flex justify-center",
      Features {
        features: vec![
          "Type Safety: Enhanced error handling and type-safe operations compared to standard Dioxus hooks".to_string(),
          "Cross-Platform: Seamless operation across desktop and web platforms with optimized implementations".to_string(),
          "Performance: Optimized state management and preventing unnecessary rerenders".to_string(),
          "Developer Experience: Intuitive APIs with clear separation of concerns".to_string(),
        ]
      }
    }

    // Navigation Bar
    div {
      class: "flex justify-center space-x-4 border-b border-gray-700 pb-4",
      button {
        class: "px-4 py-2 rounded " + if active_section() == HookSection::Clipboard { "bg-blue-600 text-white" } else { "bg-gray-700 text-gray-300" },
        onclick: move |_| active_section.set(HookSection::Clipboard),
        "Clipboard"
      }
      button {
        class: "px-4 py-2 rounded " + if active_section() == HookSection::Memo { "bg-blue-600 text-white" } else { "bg-gray-700 text-gray-300" },
        onclick: move |_| active_section.set(HookSection::Memo),
        "Memo"
      }
      button {
        class: "px-4 py-2 rounded " + if active_section() == HookSection::Pagination { "bg-blue-600 text-white" } else { "bg-gray-700 text-gray-300" },
        onclick: move |_| active_section.set(HookSection::Pagination),
        "Pagination"
      }
    }

    // Render active section only.
    (match active_section() ({
      HookSection::Clipboard => rsx! {
        section {
          class: "clipboard-demo bg-white p-6 rounded-lg shadow border border-gray-300",
          h2 { class: "text-lg text-gray-800 text-center font-bold mb-4", "Clipboard Hook Demo" }
          input {
            class: "border border-gray-400 text-gray-800 w-full rounded px-3 py-2 shadow-sm focus:ring focus:ring-blue-500",
            placeholder: "Type something to copy",
            value: "{clipboard_content}",
            oninput: move |e| clipboard_content.set(e.value().clone()),
          }
          div {
            class: "flex space-x-4 mt-4 justify-center",
            button {
              onclick: move |_| {
                let content = clipboard_content();
                let mut clipboard_ref = clipboard();
                spawn(async move {
                  match clipboard_ref.set(content).await {
                    Ok(_) => copy_status.set("Content copied!".to_string()),
                    Err(_) => copy_status.set("Failed to copy".to_string()),
                  }
                  clipboard_content.set(String::new());
                });
              },
              class: "rounded bg-blue-600 text-white py-2 px-4 hover:bg-blue-700",
              "Copy"
            }
            button {
              onclick: move |_| {
                let mut clipboard_ref = clipboard();
                spawn(async move {
                  match clipboard_ref.get().await {
                    Ok(content) => {
                      clipboard_content.set(content);
                      copy_status.set("Content pasted!".to_string());
                    },
                    Err(_) => copy_status.set("Failed to paste".to_string()),
                  }
                });
              },
              class: "rounded bg-green-500 text-white py-2 px-4 hover:bg-green-700",
              "Paste"
            }
          }
          p { class: "mt-2 text-sm text-gray-500 text-center", "{copy_status}" }
        }
      },
      HookSection::Memo => rsx! {
        section {
          class: "memo-demo bg-white p-6 rounded-lg shadow border border-gray-300",
          h2 { class: "text-lg text-center text-gray-800 font-bold mb-4", "Explicit Memo Hook Demo" }
          div {
            class: "flex space-x-4 justify-center mb-8",
            button {
              onclick: move |_| total_items.set(total_items() + 10),
              class: "rounded bg-blue-500 text-white py-2 px-4 hover:bg-blue-700",
              "+10"
            }
            button {
              onclick: move |_| total_items.set((total_items() - 10).max(0)),
              class: "rounded bg-red-500 text-white py-2 px-4 hover:bg-red-700",
              "-10"
            }
          }
          div {
            class: "bg-gray-800 p-2 rounded-md text-center shadow-inner",
            p { 
              class: "font-medium text-gray-300", 
              "Total Items: " 
              span { class: "text-yellow-500 font-bold", "{total_items}" }
            }
            p { 
              class: "font-medium text-gray-300 mt-2", 
              match *async_result.value().read_unchecked() {
                Some(_) => rsx!{ 
                  "Memoized Result: " 
                  span { class: "text-yellow-500 font-bold", "{expensive_computation}" }
                },
                None => rsx!{
                  span { class: "text-blue-400 animate-pulse", "Computing..." }
                }
              }
            }
          }
        }
      },
      HookSection::Pagination => rsx! {
        section {
          class: "pagination-demo bg-white p-6 rounded-lg shadow border border-gray-300",
          h2 { class: "text-lg font-bold text-gray-800 text-center mb-4", "Pagination Hook Demo" }
          div {
            class: "border-b border-gray-500 text-center pb-4 mb-6",
            p { class: "mb-2 text-gray-500", "Current Page: " span { class: "text-yellow-500 font-bold", "{*pagination.page.read() + 1}" } }
            p { class: "mb-2 text-gray-500", "Items per page: " span { class: "text-yellow-500 font-bold", "{*pagination.page_size.read()}" } }
            p { class: "text-gray-500", "Total Pages: " span { class: "text-yellow-500 font-bold", "{((*total_items.read() as f64) / (*pagination.page_size.read() as f64)).ceil() as i32}" } }
            p { class: "text-gray-500", "Current Index: " span { class: "text-yellow-500 font-bold", "{*pagination.idx.read()}" } }
          }
          div {
            class: "bg-gray-100 p-2 rounded mb-4 text-center " + if *pagination.touched.read() { "text-green-600" } else { "text-gray-500" },
            "Pagination Status: " 
            span { 
              class: "font-medium",
              if *pagination.touched.read() { "Active" } else { "Untouched" }
            }
          }
          div {
            class: "text-center mb-4 text-gray-700 font-medium",
            "{*pagination.counter_label.read()}"
          }
          div {
            class: "flex justify-center text-gray-800 items-center gap-4 mb-4",
            "Items per page: "
            select {
              class: "border border-gray-800 rounded p-1",
              value: "{*pagination.page_size.read()}",
              onchange: move |e| {
                if let Ok(size) = e.value().parse::<i32>() {
                  set_page_size(size);
                }
              },
              option { value: "5", "5" }
              option { value: "10", "10" }
              option { value: "15", "15" }
              option { value: "20", "20" }
            }
          }
          div {
            class: "grid grid-cols-3 gap-4",
            {
              let start_idx = *pagination.idx.read();
              let page_size = *pagination.page_size.read();
              items.iter()
                .skip(start_idx as usize)
                .take(page_size as usize)
                .map(|item| {
                  rsx! {
                    div {
                      class: "border rounded-md p-2 bg-gray-700 shadow-sm text-center",
                      key: "{item}",
                      "Item {item}"
                    }
                  }
                })
            }
          }
          hr { class:"border border-gray-700 mt-4 w-full", }
          div {
            class: "flex space-x-4 mt-6 justify-center",
            button {
              disabled: "{*pagination.prev_idx_disabled.read()}",
              onclick: move |_| prev_idx(),
              class: "rounded bg-gray-500 text-white py-2 px-4 hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",
              "<"
            }
            button {
              disabled: "{*pagination.prev_page_disabled.read()}",
              onclick: move |_| prev_page(),
              class: "rounded bg-gray-500 text-white py-2 px-4 hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",
              "<<"
            }
            button {
              disabled: "{*pagination.next_page_disabled.read()}",
              onclick: move |_| next_page(),
              class: "rounded bg-gray-500 text-white py-2 px-4 hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",
              ">>"
            }
            button {
              disabled: "{*pagination.next_idx_disabled.read()}",
              onclick: move |_| next_idx(),
              class: "rounded bg-gray-500 text-white py-2 px-4 hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",
              ">"
            }
          }
        }
      },
    )
  }
}