use {
  std::{cell::RefCell, rc::Rc},

  dioxus::prelude::*,
  maestro_hooks::{
    clipboard::use_clipboard,
    explicit_memo::use_explicit_memo,
    pagination::use_pagination,
  }
};

#[component]
pub fn HooksDemo() -> Element {
  let total_items = use_signal(|| 50);
  let clipboard = Rc::new(RefCell::new(use_clipboard()));

  // use explicit_memo (dependencies and initialization function)
  let expensive_computation = use_explicit_memo(
    total_items(),
    || format!("Computed value for {} items", total_items())
  );

  // destructure pagination hook return values
  let (pagination, (mut next_idx, mut prev_idx, mut next_page, mut prev_page)) = use_pagination(use_memo(move || total_items()));

  let mut clipboard_content = use_signal(String::new);
  let mut copy_status = use_signal(|| String::new());

  let items = (1..=50).collect::<Vec<i32>>();

  rsx! {
    div {
      class: "maestro-hooks-demo container mx-auto px-4 py-8",

      // clipboard hook demo
      section {
        h2 { "Clipboard Hook Demo" }
        p { "The clipboard hook provides cross-platform clipboard functionality with error handling" }

        div {
          class: "clipboard-demo flex flex-col space-y-2",

          input {
            placeholder: "Type something to copy",
            value: "{clipboard_content}",
            oninput: move |e| clipboard_content.set(e.value().clone()),
            class: "border rounded px-3 py-2"
          }

          div {
            class: "flex space-x-2",
            button {
              onclick: {
                let clipboard = Rc::clone(&clipboard);
                move |_| {
                  let clipboard = Rc::clone(&clipboard);
                  async move {
                    let mut clipboard = clipboard.borrow_mut();
                    if let Ok(()) = clipboard.set(clipboard_content().to_string()).await {
                      copy_status.set("Content copied!".to_string());
                    } else {
                      copy_status.set("Failed to copy".to_string());
                    }
                  }
                }
              },
              class: "rounded-md bg-blue-500 text-white py-2 px-4 hover:bg-blue-700 disabled:opacity-50",
              "Copy to Clipboard"
            },
            button {
              onclick: {
                let clipboard = Rc::clone(&clipboard);
                move |_| {
                  let clipboard = Rc::clone(&clipboard);
                  async move {
                    let mut clipboard = clipboard.borrow_mut();
                    if let Ok(content) = clipboard.get().await {
                      clipboard_content.set(content);
                      copy_status.set("Content pasted!".to_string());
                    } else {
                      copy_status.set("Failed to paste".to_string());
                    }
                  }
                }
              },
              class: "rounded-md bg-green-500 text-white py-2 px-4 hover:bg-green-700 disabled:opacity-50",
              "Paste from Clipboard"
            }
          }
          p { class: "mt-2 text-sm text-gray-500", "{copy_status}" }
        }
      }

      // explicit memo hook demo
      section {
        h2 { "Explicit Memo Hook Demo" }
        p { "The explicit memo hook prevents unnecessary rerenders and provides better performance" }

        div {
          class: "memo-demo bg-gray-100 p-4 rounded-md",

          p { "Memoized Value: {expensive_computation}" }
          p {
            class: "text-sm text-gray-500",
            "This value only recomputes when total_items changes, "
            "preventing unnecessary recalculations"
          }
        }
      }

      // pagination Hook Demo
      section {
        h2 { "Pagination Hook Demo" }
        p { "The pagination hook manages complex pagination state with automatic page calculations" }

        div {
          class: "pagination-demo",

          // pagination Info
          div {
            class: "pagination-info border-b border-gray-200 pb-2 mb-4",

            p { "Current Page: {pagination.page}" }
            p { "Items per page: {pagination.page_size}" }
            p { "Total Pages: {pagination.counter_label}" }
          }

          // paginated items display
          div {
            class: "items-container grid grid-cols-4 gap-2",
            {
              let page = (pagination.page)();
              let page_size = (pagination.page_size)();

              let start_index = (page - 1).saturating_mul(page_size);

              items.iter()
                .skip(start_index as usize)
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

          // controls
          div {
            class: "pagination-controls flex space-x-2 mt-4",

            button {
              disabled: "{pagination.prev_idx_disabled}",
              onclick: move |_| prev_idx(),
              class: "rounded-md bg-gray-300 text-gray-700 py-2 px-4 hover:bg-gray-400 disabled:opacity-50",
              "Previous Item"
            }
            button {
              disabled: "{pagination.prev_page_disabled}",
              onclick: move |_| prev_page(),
              class: "rounded-md bg-gray-300 text-gray-700 py-2 px-4 hover:bg-gray-400 disabled:opacity-50",
              "Previous Page"
            }
            button {
              disabled: "{pagination.next_page_disabled}",
              onclick: move |_| next_page(),
              class: "rounded-md bg-gray-300 text-gray-700 py-2 px-4 hover:bg-gray-400 disabled:opacity-50",
              "Next Page"
            }
            button {
              disabled: "{pagination.next_idx_disabled}",
              onclick: move |_| next_idx(),
              class: "rounded-md bg-gray-300 text-gray-700 py-2 px-4 hover:bg-gray-400 disabled:opacity-50",
              "Next Item"
            }
          }
        }
      }
    }
  }
}
