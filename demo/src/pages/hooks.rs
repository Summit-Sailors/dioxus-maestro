use {
  async_std::task::sleep, dioxus::prelude::*, maestro_hooks::{
    clipboard::use_clipboard,
    explicit_memo::use_explicit_memo,
    pagination::use_pagination,
  }, maestro_query::prelude::futures_util::FutureExt, std::time::Duration
};

#[component]
pub fn HooksDemo() -> Element {
  let mut total_items = use_signal(|| 500);
  let clipboard = use_signal(|| use_clipboard());

  let expensive_computation = use_explicit_memo(
    total_items(),
    || {
      let future = async move {
        sleep(Duration::from_millis(100)).await;
        42 // this would be the value returned by the future in a real world setting
      };

      let sum: i32 = (1..=total_items()).sum();

      let async_result = match future.now_or_never() {
        Some(result) => result, // future has completed, use the result
        None => {
          // future hasn't completed yet.  to be handled gracefully.
          // could return a placeholder value, or a message indicating
          // that the computation is in progress.
          // no blocking here!
          0
        }
    };
      format!("Sum of 1 to {}: {} and async result: {}", total_items(), sum, async_result)
    }
  );

  let page_size = 100;
  let (pagination, (mut next_idx, mut prev_idx, mut next_page, mut prev_page)) =
      use_pagination(use_memo(move || total_items()), page_size); 

  let mut clipboard_content = use_signal(String::new);
  let mut copy_status = use_signal(|| String::new());

  let items = (1..=total_items()).collect::<Vec<i32>>();

  rsx! {
    div {
      class: "maestro-hooks-demo container mx-auto px-4 py-8 space-y-12",

      // clipboard demo section
      section {
        class: "clipboard-demo bg-gray-50 p-6 rounded-lg shadow-md",

        h2 { class: "text-xl font-bold mb-4", "Clipboard Hook Demo" }
        
        input {
          class: "border rounded px-3 py-2 shadow-sm w-full focus:ring focus:ring-blue-200",
          placeholder: "Type something to copy",
          value: "{clipboard_content}",
          oninput: move |e| clipboard_content.set(e.value().clone()),
        },

        div {
          class: "flex space-x-4 mt-4",
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
            class: "rounded-md bg-blue-500 text-white py-2 px-4 hover:bg-blue-700",
            "Copy to Clipboard"
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
            class: "rounded-md bg-green-500 text-white py-2 px-4 hover:bg-green-700",
            "Paste from Clipboard"
          }
        }

        p { class: "mt-2 text-sm text-gray-500", "{copy_status}" }
      }

      // explicit memo demo section
      section {
        class: "memo-demo bg-gray-50 p-6 rounded-lg shadow-md",
        h2 { class: "text-xl font-bold mb-4", "Explicit Memo Hook Demo" }
        
        div {
          class: "flex space-x-4 mb-4",
          button {
            onclick: move |_| {
              let current = total_items();
              total_items.set(current + 10);
            },
            class: "rounded-md bg-blue-500 text-white py-2 px-4 hover:bg-blue-700",
            "Add 10 Items"
          }
          button {
            onclick: move |_| {
              let current = total_items();
              total_items.set((current - 10).max(1));
            },
            class: "rounded-md bg-red-500 text-white py-2 px-4 hover:bg-red-700",
            "Remove 10 Items"
          }
        }

        div {
          class: "bg-white p-4 rounded-md shadow-inner",
          p { class: "font-medium", "Total Items: {total_items}" }
          p { class: "font-medium mt-2", "Memoized Result: {expensive_computation}" }
        }
      }

      // pagination demo section
      section {
        class: "pagination-demo bg-gray-50 p-6 rounded-lg shadow-md",
        h2 { class: "text-xl font-bold mb-4", "Pagination Hook Demo" }

        div {
          class: "pagination-info border-b border-gray-200 pb-4 mb-6",
          p { class: "mb-2", "Current Page: {*pagination.page.read() + 1}" }
          p { class: "mb-2", "Items per page: {*pagination.page_size.read()}" }
          p {
            "Total Pages: {((*total_items.read() as f64) / (*pagination.page_size.read() as f64)).ceil() as i32}"
          }
          p { "Current Index: {*pagination.idx.read()}" }
        }

        div {
          class: "items-container grid grid-cols-4 gap-4",
          {
            let start_idx = *pagination.idx.read();
            let page_size = *pagination.page_size.read();

            items.iter()
              .skip(start_idx as usize)
              .take(page_size as usize)
              .map(|item| {
                rsx! {
                  div {
                    class: "item border rounded p-2 bg-white shadow-sm",
                    key: "{item}",
                    "Item {item}"
                  }
                }
              })
          }
        }

        div {
          class: "pagination-controls flex space-x-4 mt-6",
          button {
            disabled: "{*pagination.prev_idx_disabled.read()}",
            onclick: move |_| prev_idx(),
            class: "rounded-md bg-gray-300 text-gray-700 py-2 px-4 hover:bg-gray-400 disabled:opacity-50",
            "Previous Item"
          }
          button {
            disabled: "{*pagination.prev_page_disabled.read()}",
            onclick: move |_| prev_page(),
            class: "rounded-md bg-gray-300 text-gray-700 py-2 px-4 hover:bg-gray-400 disabled:opacity-50",
            "Previous Page"
          }
          button {
            disabled: "{*pagination.next_page_disabled.read()}",
            onclick: move |_| next_page(),
            class: "rounded-md bg-gray-300 text-gray-700 py-2 px-4 hover:bg-gray-400 disabled:opacity-50",
            "Next Page"
          }
          button {
            disabled: "{*pagination.next_idx_disabled.read()}",
            onclick: move |_| next_idx(),
            class: "rounded-md bg-gray-300 text-gray-700 py-2 px-4 hover:bg-gray-400 disabled:opacity-50",
            "Next Item"
          }
        }
      }
    }
  }
}
