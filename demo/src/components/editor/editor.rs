use {
  async_std::task::sleep, 
  dioxus::prelude::*, 
  dioxus_free_icons::{icons::fa_solid_icons::{FaCopy, FaMaximize, FaMinimize}, Icon}, 
  maestro_hooks::clipboard::use_clipboard, 
  tailwind_fuse::tw_join
};

#[derive(Props, PartialEq, Clone)]
pub struct CodeEditorProps {
  #[props(into)]
  code: String,
  #[props(default = "rust".to_string())]
  language: String,
  #[props(into, default = String::from("Example Code"))]
  title: String,
  #[props(into)]
  demo: Element,
}

#[component]
pub fn CodeEditor(props: CodeEditorProps) -> Element {
  let mut code = use_signal(|| props.code.clone());
  let mut is_expanded = use_signal(|| false);
  let clipboard = use_clipboard();
  let mut copy_status = use_signal(|| String::new());
  let mut is_copying = use_signal(|| false);

  let handle_copy = move |_| {
    let content = code();
    let mut clipboard = clipboard.clone();
    is_copying.set(true);
    spawn(async move {
      match clipboard.set(content).await {
        Ok(_) => copy_status.set("Copied!".to_string()),
        Err(_) => copy_status.set("Failed to copy".to_string()),
      }
      is_copying.set(false);
      spawn(async move {
        sleep(std::time::Duration::from_secs(2)).await;
        copy_status.set(String::new());
      });
    });
  };

  let handle_change = move |new_code: String| {
    code.set(new_code); // code in editor replaced with the new code
    // in case of any edits, replace the old code with the new code sample, run the new code and render results as the new component for the demo
  };

  let toggle_expanded = move |_| {
    is_expanded.set(!is_expanded());
  };

  rsx! {
    div {
      class: "h-screen p-2 bg-gray-800 rounded-lg mt-4 sm:mt-8 mb-8 w-full flex flex-col",

      // header section
      div {
        class: "flex items-center justify-between text-white fixe top-0 z-10 bg-gray-800 p-2",
        h2 { class: "text-xl font-semibold", "{props.title}" }
        div {
          class: "flex space-x-2",
          button {
            class: "p-2 rounded-full hover:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 relative",
            disabled: "{is_copying()}",
            onclick: handle_copy,
            title: "Copy Code",
            Icon { icon: FaCopy, width: 20, height: 20 }
            div {
              class: tw_join!("absolute -top-8 left-1/2 transform -translate-x-1/2 bg-gray-900 text-white text-xs py-1 px-2 rounded transition-opacity duration-300 {}", 
                if copy_status().is_empty() { "opacity-0" } else { "opacity-100" }
              ),
              "{copy_status}"
            }
          }
          button {
            class: "p-2 rounded-full hover:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500",
            onclick: toggle_expanded,
            title: if is_expanded() { "Collapse Code" } else { "Expand Code" },
            {
              if is_expanded() {
                rsx! { Icon { icon: FaMaximize, width: 20, height: 20 } }
              } else {
                rsx! { Icon { icon: FaMinimize, width: 20, height: 20 } }
              }
            }
          }
        }
      }    

      // scrollable container for demo and code
      div {
        class: tw_join!(
          "grid grid-cols-1 overflow-auto transition-all duration-500 ease-in-out",
          if is_expanded() { "md:grid-cols-2" } else { "grid-cols-1" }
        ),
        // demo component section
        div {
          class: "bg-gray-300 overflow-auto p-6 rounded-lg shadow-md border border-gray-200 mt-4",
          {props.demo}
        }
        // code section
        if is_expanded() {
          div {
            class: "overflow-hidden",
            h2 { class: "text-xl font-semibold text-center mt-4", "Source Code" }
            div {
              class: "h-screen overflow-y-auto bg-gray-900 rounded-lg shadow-md border border-gray-700 overflow-hidden p-4",
              div {
                class: "text-white font-mono text-sm overflow-auto",
                pre { "{props.code}" }
              }
            }
          }
        }
      }
    }
  }
}
