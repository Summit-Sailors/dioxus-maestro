use {
  crate::router::Route, dioxus::prelude::*, 
  maestro_toast::{init::use_init_toast_ctx, 
  toast_frame_component::ToastFrame}, 
  maestro_ui::button::{Button, ButtonType, ButtonVariant}, strum::IntoEnumIterator
};

#[component]
pub fn Layout(children: Element) -> Element {
  let toast = use_init_toast_ctx();

  rsx! {
    // ToastFrame { manager: toast }
    div {
      class: "grid grid-cols-7 h-screen bg-gray-50 text-gray-800",
      NavigationMenu {}
      div {
        class: "col-span-6 p-6 bg-white shadow-lg rounded-lg overflow-y-auto",
        Outlet::<Route> {}
      }
    }
  }
}

#[component]
fn NavigationMenu() -> Element {
  let navigator = use_navigator();
  let current_route = use_route::<Route>();

  rsx! {
    div {
      class: "p-6 bg-gray-800 text-white h-screen flex flex-col space-y-4",
      {Route::iter()
        .filter(|route| *route != current_route)
        .map(|route| {
          let route_name = route.name();
          rsx! {
            Link {
              to: route.clone(),
              class: "py-2 px-4 rounded-md hover:bg-gray-700 transition",
              aria_label: route_name,
              Button {
                class: "w-full text-left text-white",
                variant: ButtonVariant::Default,
                button_type: ButtonType::Button,
                on_click: move |_| {
                    navigator.push(route.clone());
                },
                "{route_name}"
              }
            }
          }
        })
      }
    }
  }
}
