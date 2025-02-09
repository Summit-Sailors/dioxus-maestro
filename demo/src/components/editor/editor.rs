use {
  async_std::task::sleep,
  dioxus::prelude::*,
  dioxus_free_icons::Icon,
  dioxus_free_icons::icons::fa_solid_icons::{FaCompress, FaCopy, FaExpand},
  maestro_hooks::clipboard::use_clipboard,
};

#[derive(Props, PartialEq, Clone)]
pub struct CodeEditorProps {
  #[props(into)]
  code: String,
  #[props(default = "rust".to_string())]
  language: String,
  #[props(into, default = String::from("Example Code"))]
  title: String,
  #[props(default = false)]
  expanded: bool,
  #[props(default = false)]
  copy_status: bool,
  #[props(into)]
  demo: Element,
}

#[component]
pub fn CodeEditor(props: CodeEditorProps) -> Element {
  let code = use_signal(|| props.code.clone());
  let mut is_expanded = use_signal(|| props.expanded);
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

  let toggle_expanded = move |_| {
    is_expanded.set(!is_expanded());
  };

  rsx! {
    div {
      class: "demo-container w-full max-w-7xl mx-auto p-4 space-y-6 bg-gray-50 rounded-lg shadow-lg",
      
      // header section with title and controls
      div {
        class: "flex items-center justify-between bg-gray-800 p-4 rounded-t-lg text-white",
        h2 { class: "text-xl font-semibold", "{props.title}" }
        div {
          class: "flex space-x-4",
          button {
            class: "p-2 rounded-full hover:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 relative",
            disabled: "{is_copying()}",
            onclick: handle_copy,
            title: "Copy Code",
            Icon {
              icon: FaCopy,
              width: 20,
              height: 20,
            }
            // tooltip for copy status
            div {
              class: format_args!("absolute -top-8 left-1/2 transform -translate-x-1/2 bg-gray-900 text-white text-xs py-1 px-2 rounded transition-opacity duration-300 {}", 
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
                rsx! {
                  Icon {
                    icon: FaCompress,
                    width: 20,
                    height: 20,
                  }
                }
              } else {
                rsx! {
                  Icon {
                    icon: FaExpand,
                    width: 20,
                    height: 20,
                  }
                }
              }
            }
          }
        }
      }

      // demo component section
      div {
        class: "bg-white p-6 rounded-lg shadow-md border border-gray-200",
        {props.demo}
      }

      // code section (expandable)
      div {
        class: format_args!("code-section overflow-hidden transition-all duration-500 ease-in-out {}", 
          if is_expanded() { "max-h-[1000px]" } else { "max-h-0" }
        ),
        div {
          class: "bg-gray-900 p-6 rounded-lg shadow-md",
          pre {
            class: "text-gray-100 overflow-x-auto",
            code {
              class: "language-{props.language} font-mono text-sm",
              "{code}"
            }
          }
        }
      }
    }
  }
}
