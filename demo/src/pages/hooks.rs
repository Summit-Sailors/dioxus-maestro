use {
  async_std::task::sleep,
  dioxus::prelude::*,
  maestro_hooks::{
    clipboard::use_clipboard,
    explicit_memo::use_explicit_memo,
    pagination::use_pagination,
  },
  std::time::Duration,
};

#[component]
pub fn HooksDemo() -> Element {
  let mut total_items = use_signal(|| 100);
  let clipboard = use_signal(|| use_clipboard());

  let async_result = use_resource(move || async move {
    sleep(Duration::from_millis(2000)).await;
    Some(total_items())
  });

  let expensive_computation = use_explicit_memo(
    total_items(), // only depend on total_items (updates only to this signal will trigger a re-render where expensive_computation is being read)
    || {
      let sum: i32 = (1..=total_items()).sum();
      format!(
        "Sum of 1 to {}: {} (Async result: {:?})", 
        total_items(), 
        sum,
        async_result.value()
      )
    }
  );

  let page_size = 10;
  let (pagination, (mut next_idx, mut prev_idx, mut next_page, mut prev_page)) =
      use_pagination(use_memo(move || total_items()), page_size);

  let mut clipboard_content = use_signal(String::new);
  let mut copy_status = use_signal(|| String::new());

  let items = (1..=total_items()).collect::<Vec<i32>>();

  rsx! {
    div {
      class: "hooks-demo bg-gray-200 p-6 rounded-lg shadow-lg space-y-6",

      // clipboard section
      section {
        class: "clipboard-demo bg-white p-6 rounded-lg shadow border border-gray-300",

        h2 { class: "text-lg text-gray-800 text-center font-bold mb-4", "Clipboard Hook Demo" }
        
        input {
          class: "border border-gray-400 text-gray-800 w-full rounded px-3 py-2 shadow-sm focus:ring focus:ring-blue-500",
          placeholder: "Type something to copy",
          value: "{clipboard_content}",
          oninput: move |e| clipboard_content.set(e.value().clone()),
        },

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
          },
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

      // memo section with loading state
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
                span { 
                  class: "text-yellow-500 font-bold", 
                  "{expensive_computation}" 
                }
              },
              None => rsx!{
                span { 
                  class: "text-blue-400 animate-pulse", 
                  "Computing..." 
                }
              }
            }
          }
        }
      }

      // pagination section
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
          class: "grid flex grid-cols-3",
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
          class: "flex space-x-1 mt-4 justify-center",
          button {
            disabled: "{*pagination.prev_idx_disabled.read()}",
            onclick: move |_| prev_idx(),
            class: "rounded bg-gray-500 text-white py-2 px-4 hover:bg-gray-800 disabled:opacity-50",
            "<"
          }
          button {
            disabled: "{*pagination.prev_page_disabled.read()}",
            onclick: move |_| prev_page(),
            class: "rounded bg-gray-500 text-white py-2 px-4 hover:bg-gray-800 disabled:opacity-50",
            "<<"
          }
          button {
            disabled: "{*pagination.next_page_disabled.read()}",
            onclick: move |_| next_page(),
            class: "rounded bg-gray-500 text-white py-2 px-4 hover:bg-gray-800 disabled:opacity-50",
            ">>"
          }
          button {
            disabled: "{*pagination.next_idx_disabled.read()}",
            onclick: move |_| next_idx(),
            class: "rounded bg-gray-500 text-white py-2 px-4 hover:bg-gray-800 disabled:opacity-50",
            ">"
          }
        }
      }
    }
  }
}
