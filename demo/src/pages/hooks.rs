use dioxus::prelude::*;
use maestro_hooks::{
    clipboard::use_clipboard,
    explicit_memo::use_explicit_memo,
    pagination::use_pagination,
};

#[component]
pub fn Hooks(cx: Scope) -> Element {
    let total_items = use_memo(cx);
    let clipboard = use_clipboard();
    let expensive_computation = use_explicit_memo(
      total_items(),  // dependency
      || {
        // expensive computation sim
        format!("Computed value for {} items", total_items())
      }
    );
    
    let pagination = use_pagination(total_items);
    
    // state for clipboard demo
    let clipboard_content = use_state(cx, String::new);
    let copy_status = use_state(cx, || "");
    
    // items for pagination demo
    let items = (1..=50).collect::<Vec<i32>>();
    
    cx.render(rsx! {
      div {
        class: "maestro-hooks-demo",
        
        // 1. Clipboard Hook Demo
        section {
          h2 { "Clipboard Hook Demo" }
          p { "The clipboard hook provides cross-platform clipboard functionality with error handling" }
          
          div {
              class: "clipboard-demo",
              input {
                placeholder: "Type something to copy",
                value: "{clipboard_content}",
                oninput: move |e| clipboard_content.set(e.value.clone())
              }
              
              button {
                onclick: move |_| async move {
                  if let Ok(()) = clipboard.set(clipboard_content.get().clone()).await {
                    copy_status.set("Content copied!");
                  } else {
                    copy_status.set("Failed to copy");
                  }
                },
                "Copy to Clipboard"
              }
              
              button {
                onclick: move |_| async move {
                  let mut clip = clipboard.clone();
                  if let Ok(content) = clip.get().await {
                    clipboard_content.set(content);
                    copy_status.set("Content pasted!");
                  } else {
                    copy_status.set("Failed to paste");
                  }
                },
                "Paste from Clipboard"
              }
              
              p { "{copy_status}" }
          }
        }
        
        // 2. Explicit Memo Hook Demo
        section {
          h2 { "Explicit Memo Hook Demo" }
          p { 
            "The explicit memo hook prevents unnecessary rerenders and provides better performance"
          }
          
          div {
            class: "memo-demo",
            p { "Memoized Value: {expensive_computation}" }
            p { 
              "This value only recomputes when total_items changes, "
              "preventing unnecessary recalculations"
            }
          }
        }
        
        // 3. Pagination Hook Demo
        section {
          h2 { "Pagination Hook Demo" }
          p { 
            "The pagination hook manages complex pagination state with automatic page calculations"
          }
          
          div {
            class: "pagination-demo",
            // pagination Info
            div {
              class: "pagination-info",
              p { "Current Page: {pagination.page()}" }
              p { "Items per page: {pagination.page_size()}" }
              p { "Total Pages: {pagination.counter_label()}" }
            }
            
            // paginated items display
            div {
              class: "items-container",
              {
                items.iter()
                .skip((pagination.page() - 1) * pagination.page_size() as usize)
                .take(pagination.page_size() as usize)
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
                disabled: "!pagination.can_prev_idx()",
                onclick: move |_| pagination.idx -= 1,
                "Previous Item"
              }
              button {
                disabled: "!pagination.can_prev_page()",
                onclick: move |_| pagination.idx -= pagination.page_size(),
                "Previous Page"
              }
              button {
                disabled: "!pagination.can_next_page()",
                onclick: move |_| pagination.idx += pagination.page_size(),
                "Next Page"
              }
              button {
                disabled: "!pagination.can_next_idx()",
                onclick: move |_| pagination.idx += 1,
                "Next Item"
              }
            }
          }
        }
      }
    })
}
