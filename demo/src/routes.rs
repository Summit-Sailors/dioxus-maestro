use dioxus::prelude::*;
use crate::pages::{forms::FormsDemo, home::HomePage};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
  #[route("/")]
  Home {},

  #[route("/forms")]
  Forms {},

  #[route("/hooks")]
  Hooks {},

  #[route("/plotters")]
  Plotters {},

  #[route("/query")]
  Query {},

  #[route("/radio")]
  Radio {},

  #[route("/toast")]
  Toast {},

  #[route("/ui")]
  UI {},

  #[route("/:..route")]
  NotFound { route: Vec<String> },
}

#[component]
fn Home() -> Element {
  rsx!(div { HomePage {} })
}

#[component]
fn Forms() -> Element {
  rsx!(div { FormsDemo {} })
}

#[component]
fn Hooks() -> Element {
  rsx!(div { "Hooks Demo Coming Soon" })
}

#[component]
fn Plotters() -> Element {
  rsx!(div { "Charts Demo Coming Soon" })
}

#[component]
fn Query() -> Element {
  rsx!(div { "Query Demo Coming Soon" })
}

#[component]
fn Radio() -> Element {
  rsx!(div { "Radio Demo Coming Soon" })
}

#[component]
fn Toast() -> Element {
  rsx!(div { "Toast Demo Coming Soon" })
}

#[component]
fn UI() -> Element {
  rsx!(div { "UI Demo Coming Soon" })
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
  let path = route.join("/");
  rsx!(
    div { class: "text-center py-16",
      h1 { class: "text-4xl font-bold text-gray-900", "Page Not Found" }
      p { class: "mt-2 text-gray-600", "The requested path /{path} could not be found." }
      Link {
        class: "mt-4 inline-block px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
        to: Route::Home {},
        "Return Home"
      }
    }
  )
}

#[component]
pub fn Router() -> Element {
  let route = use_route();

  rsx! {
    div { class: "min-h-screen bg-gray-50",
      nav { class: "bg-white shadow-sm",
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
          div { class: "flex justify-between h-16",
            div { class: "flex",
              Link { class: "flex-shrink-0 flex items-center", to: Route::Home {},
                img { class: "h-8 w-auto", src: "/logo.svg", alt: "Maestro Demo" }
              }
              div { class: "hidden sm:ml-6 sm:flex sm:space-x-8",
                NavigationLink { to: Route::Forms {}, "Forms" }
                NavigationLink { to: Route::Hooks {}, "Hooks" }
                NavigationLink { to: Route::Plotters {}, "Charts" }
                NavigationLink { to: Route::Query {}, "Query" }
                NavigationLink { to: Route::Radio {}, "Radio" }
                NavigationLink { to: Route::Toast {}, "Toast" }
                NavigationLink { to: Route::UI {}, "UI" }
              }
            }
          }
        }
      }
      main { class: "py-10",
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
          match route {
            Route::Home {} => rsx! { Home {} },
            Route::Forms {} => rsx! { Forms {} },
            Route::Hooks {} => rsx! { Hooks {} },
            Route::Plotters {} => rsx! { Plotters {} },
            Route::Query {} => rsx! { Query {} },
            Route::Radio {} => rsx! { Radio {} },
            Route::Toast {} => rsx! { Toast {} },
            Route::UI {} => rsx! { UI {} },
            Route::NotFound { route } => rsx! { NotFound { route: route } },
          }
        }
      }
    }
  }
}

#[component]
fn NavigationLink(to: Route, children: Element) -> Element {
  let route: Route = use_route();
  let is_active = route == to;

  rsx! {
    Link {
      class: format!(
        "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium {}",
        if is_active {
          "border-blue-500 text-gray-900"
        } else {
          "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700"
        }
      ),
      to: to,
      children
    }
  }
}
