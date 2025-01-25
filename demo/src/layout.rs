use {
  crate::router::Route, dioxus::prelude::*, maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame}, maestro_ui::button::{Button, ButtonType, ButtonVariant}, strum::IntoEnumIterator
};

#[component]
pub fn Layout(children: Element) -> Element {
  let toast = use_init_toast_ctx();

  rsx! {
    ToastFrame { manager: toast }
    div {
      class: "grid grid-cols-7 gap-4 max-h-screen h-full",
      NavigationMenu {}
      div {
        class: "p-4 rounded col-span-6 h-full min-h-screen overflow-y-auto", // Added overflow for long content
        Outlet::<Route> {}
      }
    }
  }
}

#[component]
fn NavigationMenu() -> Element {
  rsx! {
    div {
      class: "flex flex-col space-y-2 p-4 bg-gray-100",
      {
        Route::iter().map(|route| {
          let route_name = match route {
            Route::HomePage {} => "Home",
            Route::FormsDemo {} => "Forms",
            Route::HooksDemo {} => "Hooks",
            Route::PlottersDemo {} => "Plotters",
            Route::QueryDemo {} => "Query",
            Route::RadioDemo {} => "Radio",
            Route::ToastDemo {} => "Toast",
            Route::UIDemo {} => "UI",
          };

          rsx! {
            Link {
              to: route.clone(),
              class: "w-full",
              Button {
                class: "w-full",
                variant: ButtonVariant::Default,
                button_type: ButtonType::Button,
                "{route_name}"
              }
            }
          }
        })
      }
    }
  }
}
