use {
  crate::components::ui::features::Features, async_std::task::sleep, 
  dioxus::prelude::*, dioxus_free_icons::{ Icon, icons::fa_solid_icons::{ FaCopy, FaPaste } },
  maestro_hooks::{
    clipboard::use_clipboard,
    explicit_memo::use_explicit_memo,
    pagination::use_pagination,
  }, std::time::Duration, tailwind_fuse::tw_join
};

#[derive(PartialEq)]
enum HookSection {
  Clipboard,
  Memo,
  Pagination,
}

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
    let (pagination, (mut next_idx, mut prev_idx, mut next_page, mut prev_page, mut set_page_size)) =
        use_pagination(use_memo(move || total_items()), page_size);

  let mut clipboard_content = use_signal(String::new);
  let mut copy_status = use_signal(|| String::new());

  let items = (1..=total_items()).collect::<Vec<i32>>();

  let mut active_section = use_signal(|| HookSection::Clipboard);

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
        class: "flex",
        Features {
          title: "Hooks".to_string(),
          features: vec![
            "Type Safety: Enhanced error handling and type-safe operations compared to standard Dioxus hooks".to_string(),
            "Cross-Platform: Seamless operation across desktop and web platforms with optimized implementations".to_string(),
            "Performance: Optimized state management and preventing unnecessary rerenders".to_string(),
            "Developer Experience: Intuitive APIs with clear separation of concerns".to_string(),
          ]
        }
      }

      div {
        class: "flex flex-wrap sm:flex-nowrap space-x-2 sm:space-x-0 border-b border-gray-700 pb-4",
        span {
          class: tw_join!(
            "py-2 px-2 cursor-pointer rounded text-sm sm:text-xs",
            if *active_section.read() == HookSection::Clipboard { "text-white border-b-4 border-gray-500" } else { "text-gray-300" }
          ),
          onclick: move |_| active_section.set(HookSection::Clipboard),
          "Clipboard"
        }
        span {
          class: tw_join!(
            "py-2 px-2 cursor-pointer rounded text-sm sm:text-xs",
            if *active_section.read() == HookSection::Memo { "text-white border-b-4 border-gray-500" } else { "text-gray-300" }
          ),
          onclick: move |_| active_section.set(HookSection::Memo),
          "Memo"
        }
        span {
          class: tw_join!(
            "py-2 px-2 cursor-pointer rounded text-sm sm:text-xs",
            if *active_section.read() == HookSection::Pagination { "text-white border-b-4 border-gray-500" } else { "text-gray-300" }
          ),
          onclick: move |_| active_section.set(HookSection::Pagination),
          "Pagination"
        }
      }

      match *active_section.read() {
        HookSection::Clipboard => rsx! {
          section {
            class: "clipboard-demo bg-gray-900 p-6 rounded-lg shadow",
    
            h2 { class: "text-lg text-gray-100 text-center font-bold mb-4", "Clipboard Hook" }

            p { 
              class: "text-gray-300 text-center mb-4",
					    "A unified clipboard interface that works seamlessly across desktop and web platforms with comprehensive error handling."
				    }
            
            input {
              class: "bg-gray-900 border border-gray-500 text-gray-100 w-full rounded px-3 py-2 shadow-sm focus:ring-2 focus:ring-blue-500 focus:outline-none placeholder-gray-500",
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
                span {
                  class: "flex items-center gap-2",
                  "Copy",
                  Icon {
                    width: 16,
                    height: 16,
                    icon: FaCopy,
                    class: "text-gray-50",
                  }
                }
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
                span {
                  class: "flex items-center gap-2",
                  "Paste",
                  Icon {
                    width: 16,
                    height: 16,
                    icon: FaPaste,
                    class: "text-gray-50",
                  }
                }
              }
            }
            
    
            p { class: "mt-2 text-sm text-gray-500 text-center", "{copy_status}" }
          }

          div {
            class: "flex mt-4",
            Features {
              title: "Clipboard".to_string(),
              features: vec![
                "Feature-flagged platform-specific optimizations (desktop and web features)".to_string(),
                "Structured error handling with custom ClipboardError enum".to_string(),
                "Automatic context management and cleanup".to_string(),
                "First-class support for async operations".to_string(),
              ]
            }
          }
        },
        HookSection::Memo => rsx! {
          section {
            class: "memo-demo bg-gray-900 p-6 rounded-lg shadow",
    
            h2 { class: "text-lg text-center text-gray-100 font-bold mb-4", "Explicit Memo Hook" }
            
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

          div {
            class: "flex mt-4",
            Features {
              title: "Explicit Memo".to_string(),
              features: vec![
                "Zero unnecessary rerenders guaranteed by explicit dependency tracking".to_string(),
                "More predictable and efficient memory usage".to_string(),
                "Clear separation between dependencies and computation logic".to_string(),
                "Perfect for expensive computations where performance is critical".to_string(),
              ]
            }
          }
        },
        HookSection::Pagination => rsx! {
          section {  
            class: "pagination-demo bg-gray-900 p-6 rounded-lg shadow",

            h2 { class: "text-lg font-bold text-gray-100 text-center mb-4", "Pagination Hook" }

            table {
              class: "w-auto mx-auto text-center border-collapse rounded-md shadow border border-gray-500 rounded-lg mb-6",
              tr {
                td { class: "border px-4 py-2 text-gray-200", "Current Page:" }
                td { class: "border px-4 py-2 text-yellow-500 font-bold", "{*pagination.page.read() + 1}" }
              }
              tr {
                td { class: "border px-4 py-2 text-gray-200", "Items per page:" }
                td { class: "border px-4 py-2 text-yellow-500 font-bold", "{*pagination.page_size.read()}" }
              }
              tr {
                td { class: "border px-4 py-2 text-gray-200", "Total Pages:" }
                td { class: "border px-4 py-2 text-yellow-500 font-bold", "{((*total_items.read() as f64) / (*pagination.page_size.read() as f64)).ceil() as i32}" }
              }
              tr {
                td { class: "border px-4 py-2 text-gray-200", "Current Index:" }
                td { class: "border px-4 py-2 text-yellow-500 font-bold", "{*pagination.idx.read()}" }
              }
              tr {
                td {
                  class: "border px-4 py-2 text-gray-200",
                  "Pagination Status: "
                }
                td {
                  class: "border px-4 py-2 text-gray-200",
                  span {
                    class: "font-medium",
                    if *pagination.touched.read() { "Active" } else { "Untouched" }
                  }
                }
              }
            }

            div {
              class: "flex justify-center text-gray-200 items-center gap-4 mb-4",
              "Items per page: "
              select {
                class: "border bg-gray-900 border-gray-500 rounded p-1",
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
                        class: "border border-gray-800 rounded-md p-2 bg-blue-500 shadow-sm text-center",
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

          div {
            class: "flex mt-4",
            Features {
              title: "Pagination".to_string(),
              features: vec![
                "Automatic page size calculations".to_string(),
                "Bidirectional navigation (both by page and by individual items)".to_string(),
                "Real-time counter labels and disabled states".to_string(),
                "Optimized state updates with Signal integration".to_string(),
                "Zero-based index support with automatic boundary handling".to_string(),
              ]
            }
          }
        }
      }     
    }
  }
}
