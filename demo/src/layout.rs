use {
  crate::{
    components::layout::demo_wrapper::DemoWrapper, 
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
  
  let content = match current_route {
    Route::HomePage {} => rsx! { {children} },
    _ => rsx! {
      DemoWrapper {
        title: current_route.name(),
        source_code: get_source_code(&current_route),
        {children}
      }
    },
  };

  rsx! {
    head {
      link { rel: "stylesheet", href: asset!("/assets/main.css") }
      link { rel: "icon", href: asset!("/assets/favicon.ico") }
    }
    
    ToastFrame { manager: toast }
    
    div {
      class: "min-h-screen bg-gray-50 flex flex-col relative",
      
      div {
        class: "lg:hidden fixed top-0 left-0 w-full h-16 bg-gray-900 flex items-center justify-between px-4 z-50",
        
        button {
          class: "p-2 rounded-md bg-gray-800 text-white hover:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-600",
          onclick: move |_| menu_open.set(!menu_open()),
          
          {
            if menu_open() {
              rsx! { Icon { icon: FaX, width: 20, height: 20 } }
            } else {
              rsx! { Icon { icon: FaBars, width: 20, height: 20 } }
            }
          }
        }
      }
      
      div {
        class: "flex min-h-screen pt-16 lg:pt-0",
        
        nav {
          class: tw_join!(
            "fixed inset-y-0 left-0 z-40",
            "w-64 bg-gray-800",
            "transform transition-transform duration-300 ease-in-out",
            "lg:static",
            "lg:block",
            if !menu_open() { 
              "hidden lg:block"
            } else {
              "block"
            }
          ),
          NavigationMenu {
            close_menu: menu_open.clone()
          }
        }
        
        main {
          class: tw_join!(
            "flex-1 transition-all duration-300",
            "px-4 py-4 sm:px-6 lg:px-8",
            "lg:ml-64",
            "w-full max-w-full overflow-x-hidden"
          ),
          
          if menu_open() {
            div {
              class: "fixed inset-0 bg-black/50 z-30 lg:hidden transition-opacity duration-300",
              onclick: move |_| menu_open.set(false)
            }
          }
          
          div {
            class: "container mx-auto max-w-7xl px-0 sm:px-4",
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
      class: "min-h-screen flex flex-col pt-16 lg:pt-4",
      
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
    Route::FormsDemo {} => include_str!("pages/forms.rs"),
    Route::HooksDemo {} => include_str!("pages/hooks.rs"),
    Route::PlottersDemo {} => include_str!("pages/plotters.rs"),
    Route::CompleteQueryDemo {} => include_str!("pages/query.rs"),
    Route::RadioDemo {} => include_str!("pages/radio.rs"),
    Route::ToastDemo {} => include_str!("pages/toast.rs"),
    Route::UIDemo {} => include_str!("pages/ui.rs"),
    Route::CalendarDemo {} => include_str!("pages/calendar.rs"),
  }
}
