use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use maestro_hooks::{
    clipboard::use_clipboard,
    explicit_memo::use_explicit_memo,
    pagination::use_pagination,
};

#[component]
pub fn Hooks() -> Element {
  let total_items = use_signal(|| 50);
  let clipboard = Rc::new(RefCell::new(use_clipboard()));
  
  // use explicit_memo  (dependencies and initialization function)
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
      class: "maestro-hooks-demo",
      
      // cllipboard hook demo
      section {
        h2 { "Clipboard Hook Demo" }
        p { "The clipboard hook provides cross-platform clipboard functionality with error handling" }
        
        div {
          class: "clipboard-demo",
          input {
            placeholder: "Type something to copy",
            value: "{clipboard_content}",
            oninput: move |e| clipboard_content.set(e.value().clone())
          }
          
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
          "Paste from Clipboard"
        }
          p { "{copy_status}" }
        }
      }
      
      // explicit memo hook demo
      section {
          h2 { "Explicit Memo Hook Demo" }
          p { "The explicit memo hook prevents unnecessary rerenders and provides better performance" }
          
          div {
            class: "memo-demo",
            p { "Memoized Value: {expensive_computation}" }
            p { 
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
                class: "pagination-info",
                p { "Current Page: {pagination.page}" }
                p { "Items per page: {pagination.page_size}" }
                p { "Total Pages: {pagination.counter_label}" }
              }
              
              // paginated items display
              div {
                class: "items-container",
                {
                  items.iter()
                    .skip((((pagination.page)() - 1) * (pagination.page_size)()).try_into().unwrap())
                    .take((pagination.page_size)() as usize)
                    .map(|item| {
                      rsx! {
                        div {
                          class: "item",
                          key: "{item}",
                          "Item {item}"
                        }
                      }
                    })
                }
              }
              
              // controls
              div {
                class: "pagination-controls",
                button {
                  disabled: "{pagination.prev_idx_disabled}",
                  onclick: move |_| prev_idx(),
                  "Previous Item"
                }
                button {
                  disabled: "{pagination.prev_page_disabled}",
                  onclick: move |_| prev_page(),
                  "Previous Page"
                }
                button {
                  disabled: "{pagination.next_page_disabled}",
                  onclick: move |_| next_page(),
                  "Next Page"
                }
                button {
                  disabled: "{pagination.next_idx_disabled}",
                  onclick: move |_| next_idx(),
                  "Next Item"
                }
              }
          }
      }
    }
  }
}
