use {
  crate::router::Route,
  dioxus::prelude::*,
  dioxus_free_icons::{icons::fa_solid_icons::{FaBars, FaX}, Icon},
  maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame},
  strum::IntoEnumIterator,
  tailwind_fuse::tw_join
};

#[component]
pub fn Layout(children: Element) -> Element {
  let toast = use_init_toast_ctx();
  let mut menu_open = use_signal(|| false);
  let current_route = use_route::<Route>();

  let content = match current_route {
    Route::HomePage {} => children,
    Route::FormsDemo {} => create_demo_wrapper(
      children,
      include_str!("pages/forms.rs"),
      "Forms Demo"
    ),
    Route::HooksDemo {} => create_demo_wrapper(
      children,
      include_str!("pages/hooks.rs"),
      "Hooks Demo"
    ),
    Route::PlottersDemo {} => create_demo_wrapper(
      children,
      include_str!("pages/plotters.rs"),
      "Plotters Demo"
    ),
    Route::CompleteQueryDemo {} => create_demo_wrapper(
      children,
      include_str!("pages/query.rs"),
      "Query Demo"
    ),
    Route::RadioDemo {} => create_demo_wrapper(
      children,
      include_str!("pages/radio.rs"),
      "Radio Demo"
    ),
    Route::ToastDemo {} => create_demo_wrapper(
      children,
      include_str!("pages/toast.rs"),
      "Toast Demo"
    ),
    Route::UIDemo {} => create_demo_wrapper(
      children,
      include_str!("pages/ui.rs"),
      "UI Demo"
    ),
    Route::CalendarDemo {} => create_demo_wrapper(
      children,
      include_str!("pages/calendar.rs"),
      "Calendar Demo"
    ),
  };

  rsx! {
    head {
      link { rel: "stylesheet", href: asset!("/assets/main.css") }
      link { rel: "icon", href: asset!("/assets/favicon.ico") }
    }

    ToastFrame { manager: toast }

    div {
      class: "lg:hidden fixed top-4 left-4 z-50",
      button {
        class: tw_join!(
          "bg-gray-800 text-white p-2 rounded-md transition-transform duration-300 ease-in-out",
          if menu_open() { "rotate-90" } else { "" }
        ),
        onclick: move |_| menu_open.set(!menu_open()),
        Icon {
          width: 30,
          height: 30,
          fill: "white",
          icon: FaBars
        }
      }
    }

    div {
      class: "flex min-h-screen bg-gray-50 text-gray-800",

      // nav menu
      div {
        class: tw_join!(
          "fixed inset-y-0 left-0 z-40 w-full sm:w-64 bg-gray-800 text-white overflow-y-auto transition-transform duration-300 ease-in-out lg:translate-x-0",
          if menu_open() { "translate-x-0" } else { "-translate-x-full lg:translate-x-0" }
        ),
        NavigationMenu { close_menu: menu_open.clone() }
      }

      // outlet content
      div {
        class: tw_join!(
          "flex-grow p-4 sm:p-6 overflow-y-auto transition-all duration-300 ease-in-out",
          "lg:ml-64" 
        ),
        div {
          class: "max-w-7xl mx-auto",
          {content}
        }
      }      
    }
  }
}

#[component]
fn NavigationMenu(close_menu: Signal<bool>) -> Element {
  let current_route = use_route::<Route>();

  rsx! {
    div {
      class: "w-full h-full p-6 flex flex-col space-y-4",

      div {
        class: "lg:hidden mb-4 flex justify-between items-center",
        h2 { class: "text-xl font-bold", "Menu" }
        button {
          class: "text-white p-2 rounded-md transition-colors duration-300 hover:bg-gray-700",
          onclick: move |_| close_menu.set(false),
          Icon {
            width: 24,
            height: 24,
            fill: "white",
            icon: FaX
          }
        }
      }

      {Route::iter()
        .filter(|route| *route != current_route)
        .map(|route| {
          let route_name = route.name();
          rsx! {
            Link {
              to: route.clone(),
              class: tw_join!(
                "py-2 px-4 rounded-md hover:bg-gray-700 transition-colors duration-300 w-full text-left text-white"
              ),
              aria_label: route_name,
              onclick: move |_| close_menu.set(false),
              "{route_name}"
            }
          }
        })
      }
    }
  }
}

fn create_demo_wrapper(children: Element, source_code: &str, title: &str) -> Element {
  rsx! {
    div {
      class: "space-y-8",
      h1 { class: "text-3xl font-bold mb-4", "{title}" }
      div { class: "bg-white rounded-lg shadow-md p-6", {children} }
      div {
        class: "bg-gray-100 rounded-lg p-6 mt-8",
        h2 { class: "text-xl font-semibold mb-4", "Source Code" }
        pre {
          class: "bg-gray-800 text-white p-4 rounded overflow-x-auto",
          code { "{source_code}" }
        }
      }
    }
  }
}

