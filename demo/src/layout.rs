use {
  crate::{
    components::editor::editor::CodeEditor, 
    router::Route
  },
  dioxus::prelude::*,
  dioxus_free_icons::{icons::fa_solid_icons::{FaBars, FaX}, Icon},
  maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame},
  strum::IntoEnumIterator,
  tailwind_fuse::tw_join,
};

#[component]
pub fn Layout(children: Element) -> Element {
  let toast = use_init_toast_ctx();
  let mut menu_open = use_signal(|| false);
  let current_route = use_route::<Route>();

  let content = rsx! {
    match current_route {
      Route::HomePage {} => rsx!{ Outlet<Route> {} },
      _ => rsx! {
          div {
            class: "mt-8 flex flex-1",
            CodeEditor {
              title: current_route.name(),
              code: get_source_code(&current_route),
              demo: rsx!{ Outlet<Route> {} },
            }
          }
      }
    }
  };

  rsx! {
    head {
      link { rel: "stylesheet", href: asset!("/assets/main.css") }
      link { rel: "icon", href: asset!("/assets/favicon.ico") }
    }
    
    ToastFrame { manager: toast }
    
    div {
      class: "min-h-screen flex flex-col relative",
      
      div {
        class: "lg:hidden top-0 left-0 w-full h-24 bg-gray-900 z-40 mb-8",
        
        button {
          class: "fixed left-0 right-0 mx-auto max-w-md flex text-white bg-gray-800 transition-colors focus:outline-none focus:ring-2 focus:ring-inset focus:ring-gray-600",
          onclick: move |_| menu_open.set(!menu_open()),
          
          div {
            class: "p-4",
            {
              if menu_open() {
                rsx! { Icon { icon: FaX, width: 20, height: 20 } }
              } else {
                rsx! { Icon { icon: FaBars, width: 20, height: 20 } }
              }
            }
          }
        }
      }
      
      div {
        class: "flex flex-1",
        
        // sidebar
        nav {
          class: tw_join!(
            "fixed inset-y-0 left-0 z-50",
            "w-64 bg-gray-800",
            "transform transition-transform duration-300 ease-in-out",
            if !menu_open() { "hidden lg:block" } else { "block" }
          ),
          NavigationMenu { close_menu: menu_open }
        }        
        
        // main Content
        main {
          class: tw_join!(
            "flex-1 flex flex-col transition-all duration-300",
            "w-full max-w-full overflow-hidden",
            "ml-0 md:ml-64"
          ),
          
          if menu_open() {
            div {
              class: "fixed inset-0 bg-black/50 z-30 md:hidden transition-opacity duration-300",
              onclick: move |_| menu_open.set(false)
            }
          }
          
          div {
            class: "container mx-auto px-4 sm:px-4 h-full flex-1",
            {content}
          }
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
      class: "min-h-screen",

      div {
        class: "flex-1 px-4 py-2 space-y-1 overflow-y-auto",
        {Route::iter()
          .map(|route| {
            let is_current = route == current_route;
            rsx! {
              Link {
                to: route.clone(),
                class: tw_join!(
                  "block px-4 py-2 rounded-md transition-colors w-full text-left",
                  "hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-600",
                  if is_current {
                    "bg-gray-700 text-white"
                  } else {
                    "text-gray-300 hover:text-white"
                  }
                ),
                onclick: move |_| close_menu.set(false),
                "{route.name()}"
              }
            }
          })
        }
      }
    }
  }
}

fn get_source_code(route: &Route) -> &'static str {
  match route {
    Route::HomePage {} => "",
    Route::FormsDemo {} => include_str!("pages/form.rs"),
    Route::HooksDemo {} => include_str!("pages/hooks.rs"),
    Route::PlottersDemo {} => include_str!("pages/plotters.rs"),
    Route::CompleteQueryDemo {} => include_str!("pages/query.rs"),
    Route::RadioDemo {} => include_str!("pages/radio.rs"),
    Route::ToastDemo {} => include_str!("pages/toast.rs"),
    Route::UIDemo {} => include_str!("pages/ui.rs"),
    Route::CalendarDemo {} => include_str!("pages/calendar.rs"),
  }
}
