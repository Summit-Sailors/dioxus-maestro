use {
  crate::router::Route, dioxus::prelude::*, 
  maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame}, 
  strum::IntoEnumIterator
};

#[component]
pub fn Layout(children: Element) -> Element {
  let toast = use_init_toast_ctx();

  rsx! {
    head {
      link { rel: "stylesheet", href: asset!("/assets/main.css") }
      link { rel: "icon", href: asset!("/assets/favicon.ico") }
    }

    ToastFrame { manager: toast }
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
              class: "py-2 px-4 rounded-md hover:bg-gray-700 transition w-full text-left text-white",
              aria_label: route_name,
              "{route_name}"
            }
          }
        })
      }
    }
  }
}
