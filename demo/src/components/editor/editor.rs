use {
  dioxus::prelude::*,
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
        Ok(_) => copy_status.set("Content copied!".to_string()),
        Err(_) => copy_status.set("Failed to copy".to_string()),
      }
      is_copying.set(false);
    });
  };

  let toggle_expanded = move |_| {
    is_expanded.set(!is_expanded());
  };

  rsx! {
    div {
      class: "demo-container w-full max-w-7xl mx-auto p-4 space-y-8",
      
      // header section with title and controls
      div {
        class: "flex items-center justify-between bg-gray-100 p-4 rounded-t-lg",
        h2 { class: "text-xl font-semibold text-gray-800", "{props.title}" }
        div {
            class: "flex space-x-2",
            button {
              class: "px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors flex items-center space-x-1",
              disabled: "{is_copying()}",
              onclick: handle_copy,
              span { "Copy Code" }
            }
            {
            if copy_status().len() > 0 {
              rsx! {
                span { 
                  class: "ml-2 text-sm",
                  "{copy_status}"
                }
              }
            } else {
              rsx! { }
            }
          }
          button {
            class: "px-3 py-1 bg-gray-500 text-white rounded hover:bg-gray-600 transition-colors",
            onclick: toggle_expanded,
            {
              if is_expanded() {
                "Hide Code"
              } else {
                "Show Code"
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
        class: "code-section transition-all duration-300",
        style: {
          if !is_expanded() {
            "max-height: 0; overflow: hidden;"
          } else {
            ""
          }
        },
        div {
          class: "bg-gray-900 p-6 rounded-lg shadow-md",
          pre {
            class: "text-gray-100 overflow-x-auto",
            code {
              class: "language-{props.language}",
              "{code}"
            }
          }
        }
      }
    }
  }
}
