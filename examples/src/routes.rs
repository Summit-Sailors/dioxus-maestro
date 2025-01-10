
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::forms::FormsDemo;

#[derive(Routable, Clone)]
pub enum Route {
  #[route("/")]
  Home {},
  
  #[route("/forms")]
  Forms {},
  
  // TODO: routes for future demos
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

/// main router component that handles navigation and layout
#[component]
pub fn Router(cx: Scope) -> Element {
  let route: Route = use_route(&cx);
  
  cx.render(rsx! {
    div { class: "min-h-screen bg-gray-50",
      nav { class: "bg-white shadow-sm",
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
          div { class: "flex justify-between h-16",
            div { class: "flex",
              // home link
              Link { class: "flex-shrink-0 flex items-center", to: Route::Home {},
                img {
                  class: "h-8 w-auto",
                  src: "/logo.svg",
                  alt: "Maestro Demo"
                }
              }
              
              // main navigation
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
      
      // main content area
      main { class: "py-10",
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
          // route content
          match route {
            Route::Home {} => rsx! { HomePage {} },
            Route::Forms {} => rsx! { FormsDemo {} },
            Route::Hooks {} => rsx! { "Hooks Demo Coming Soon" },
            Route::Plotters {} => rsx! { "Charts Demo Coming Soon" },
            Route::Query {} => rsx! { "Query Demo Coming Soon" },
            Route::Radio {} => rsx! { "Radio Demo Coming Soon" },
            Route::Toast {} => rsx! { "Toast Demo Coming Soon" },
            Route::UI {} => rsx! { "UI Demo Coming Soon" },
            Route::NotFound { route } => rsx! {
              div { class: "text-center py-16",
                h1 { class: "text-4xl font-bold text-gray-900",
                  "Page Not Found"
                }
                p { class: "mt-2 text-gray-600",
                  "The requested path /{route.join("/")} could not be found."
                }
                Link {
                  class: "mt-4 inline-block px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                  to: Route::Home {},
                  "Return Home"
                }
              }
            }
          }
        }
      }
    }
  })
}

/// Navigation link component with active state handling
#[component]
fn NavigationLink(cx: Scope, to: Route, children: Element) -> Element {
  let route: Route = use_route(&cx);
  let is_active = matches!((route.clone(), to.clone()),
    (Route::Home {}, Route::Home {}) |
    (Route::Forms {}, Route::Forms {}) |
    (Route::Hooks {}, Route::Hooks {}) |
    (Route::Plotters {}, Route::Plotters {}) |
    (Route::Query {}, Route::Query {}) |
    (Route::Radio {}, Route::Radio {}) |
    (Route::Toast {}, Route::Toast {}) |
    (Route::UI {}, Route::UI {})
  );
  
  cx.render(rsx! {
    Link {
      class: "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium {if is_active { 
        "border-blue-500 text-gray-900" 
      } else { 
        "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700" 
      }}",
      to: to,
      children
    }
  })
}
