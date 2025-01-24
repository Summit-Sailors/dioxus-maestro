use {
  dioxus::prelude::*,
  maestro_toast::{
    ctx::use_toast, 
    init::use_init_toast_ctx, 
    toast_code::EToastCode, 
    toast_frame_component::ToastFrame, 
    toast_info::ToastInfo, 
    toast_position::EToastPosition
  }
};

#[component]
pub fn ToastDemo() -> Element {
  let toast_manager = use_init_toast_ctx();
  let mut toast = use_toast();
  
  let show_success = move |_| {
    let info = ToastInfo {
      heading: Some("Success!".to_string()),
      context: "Operation completed successfully".to_string(),
      icon: Some(EToastCode::Success),
      position: EToastPosition::TopRight,
      allow_toast_close: true,
      hide_after: 5,
    };
    toast.write().popup(info);
  };

  let show_error = move |_| {
    let info = ToastInfo {
      heading: Some("Error".to_string()),
      context: "Something went wrong".to_string(),
      icon: Some(EToastCode::Error),
      position: EToastPosition::BottomRight,
      allow_toast_close: true,
      hide_after: 8,
    };
    toast.write().popup(info);
  };

  let show_warning = move |_| {
    let info = ToastInfo {
      heading: None,
      context: "This is a warning message".to_string(),
      icon: Some(EToastCode::Warning),
      position: EToastPosition::BottomLeft,
      allow_toast_close: false,
      hide_after: 6,
    };
    toast.write().popup(info);
  };

  let show_info = move |_| {
    let info = ToastInfo {
      heading: Some("Info".to_string()),
      context: "Here's some information".to_string(),
      icon: Some(EToastCode::Info),
      position: EToastPosition::TopLeft,
      allow_toast_close: true,
      hide_after: 7,
    };
    toast.write().popup(info);
  };

  let show_custom = move |_| {
    let info = ToastInfo {
      heading: Some("Custom Toast".to_string()),
      context: "This is a custom toast without an icon".to_string(),
      icon: None,
      position: EToastPosition::TopRight,
      allow_toast_close: true,
      hide_after: 5,
    };
    toast.write().popup(info);
  };

  let clear_all = move |_| {
    toast.write().clear();
  };

  rsx! {
    ToastFrame { manager: toast_manager }

    div {
      class: "flex flex-col items-center justify-center min-h-screen bg-gray-100 p-4",
      div {
        class: "bg-white rounded-lg shadow-md p-6 max-w-2xl w-full",
        h1 {
          class: "text-2xl font-bold mb-6 text-center",
          "Toast Demo"
        }
        div {
          class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-6",
          button {
            class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
            onclick: show_success,
            "Show Success Toast"
          }
          button {
            class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
            onclick: show_error,
            "Show Error Toast"
          }
          button {
            class: "px-4 py-2 bg-yellow-500 text-white rounded hover:bg-yellow-600",
            onclick: show_warning,
            "Show Warning Toast"
          }
          button {
            class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
            onclick: show_info,
            "Show Info Toast"
          }
          button {
            class: "px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600",
            onclick: show_custom,
            "Show Custom Toast"
          }
          button {
            class: "px-4 py-2 bg-gray-700 text-white rounded hover:bg-gray-800",
            onclick: clear_all,
            "Clear All Toasts"
          }
        }
        div {
          class: "space-y-4 text-sm text-gray-600",
          h2 {
            class: "font-semibold text-lg",
            "Features Demonstrated:"
          }
          ul {
            class: "list-disc list-inside space-y-2",
            li { "Different positions (check each toast's position)" }
            li { "Various toast types with matching icons and colors" }
            li { "Configurable auto-close timers (5-8 seconds)" }
            li { "Optional close buttons" }
            li { "Optional headers" }
            li { "Custom toast without icon" }
          }
        }
      }
    }
  }
}
