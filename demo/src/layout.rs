use {
  crate::{
    components::editor::editor::CodeEditor, 
    router::Route
  }, 
  dioxus::prelude::*, 
  dioxus_free_icons::{icons::{fa_solid_icons::{FaBars, FaX}, fi_icons::FiGithub}, Icon}, 
  maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame}, 
  std::collections::HashMap, 
  strum::IntoEnumIterator, 
  tailwind_fuse::tw_join
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
              code_map: get_source_code(&current_route),
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
      class: "min-h-screen flex flex-col relative overflow-hidden",

      div {
        class: "container flex mb-4 justify-center backdrop-blur-md",  
        header {  
          class: "flex fixed p-4 mt-0 text-white shadow-md z-50",
  
          h1 {  
            class: "text-lg font-semibold",
            "Dioxus Maestro|"
          }
  
          a {
            href: "https://github.com/Summit-Sailors/dioxus-maestro/tree/maestro-demo/demo",
            target: "_blank",
            class: "flex items-center space-x-2 text-gray-300 hover:text-white transition",
  
            Icon {
              icon: FiGithub,
              width: 20,
              height: 20,
            }
            span {  "View On GitHub"}
          }
        }
      }
      
      div {
        class: "lg:hidden top-0 left-0 w-full h-24 bg-gray-900 z-40 mb-8",
        
        button {
          class: "fixed left-0 right-0 mx-auto max-w-md flex text-white bg-gray-800 transition-colors focus:outline-none focus:ring-2 focus:ring-inset focus:ring-gray-600",
          onclick: move |_| menu_open.toggle(),
          
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
        class: "flex flex-1 items-stretch",
    
        // sidebar
        nav {
          class: tw_join!(
            "w-64 mt-2 z-50 shadow-md rounded-lg bg-gray-800",
            "transform transition-transform duration-300 ease-in-out",
            if current_route.name() == "Home" { "hidden lg:hidden translate-x-0" } 
            else if !menu_open() { "hidden lg:block" } else { "block" }
          ),
          NavigationMenu { close_menu: menu_open }
        }        
    
        // main Content
        main {
          class: tw_join!(
            "flex-1 flex flex-col transition-all duration-300",
            "w-full max-w-full overflow-hidden"
          ),
  
          if menu_open() {
            div {
              class: "fixed inset-0 bg-black/50 z-30 md:hidden transition-opacity duration-300",
              onclick: move |_| menu_open.toggle()
            }
          }
  
          div {
            class: "container mx-auto px-2 sm:px-2 h-full flex-1",
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
      class: "h-full",

      div {
        class: "px-4",
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
                onclick: move |_| close_menu.toggle(),
                "{route.name()}"
              }
            }
          })
        }
      }
    }
  }
}

// include code and it's deps/utilities
fn get_source_code(route: &Route) -> HashMap<String, String> {
  let mut code_map = HashMap::new();
  
  match route {
    Route::HomePage {} => {
      code_map.insert("home".to_string(), "".to_string());
    }
    Route::FormsDemo {} => {
      code_map.insert("form demo".to_string(), String::from(include_str!("pages/form.rs")));
      code_map.insert("form component".to_string(), String::from(include_str!("components/form/form_content.rs")));
      code_map.insert("form state debugger".to_string(), String::from(include_str!("components/form/form_state_debugger.rs")));
      code_map.insert("form field wrapper".to_string(), String::from(include_str!("components/form/form_field_wrapper.rs")));
    }
    Route::HooksDemo {} => {
      code_map.insert("hooks".to_string(), String::from(include_str!("pages/hooks.rs")));
    }
    Route::PlottersDemo {} => {
      code_map.insert("plotters".to_string(), String::from(include_str!("pages/plotters.rs")));
    }
    Route::CompleteQueryDemo {} => {
      code_map.insert("query".to_string(), String::from(include_str!("pages/query.rs")));
      code_map.insert("basic query demo".to_string(), String::from(include_str!("components/query/basic_query.rs")));
      code_map.insert("batch query demo".to_string(), String::from(include_str!("components/query/batch.rs")));
      code_map.insert("cache query demo".to_string(), String::from(include_str!("components/query/cache.rs")));
      code_map.insert("mutation query demo".to_string(), String::from(include_str!("components/query/mutation.rs")));
      code_map.insert("parallel query demo".to_string(), String::from(include_str!("components/query/parallel_query.rs")));
    }
    Route::RadioDemo {} => {
      code_map.insert("radio".to_string(), String::from(include_str!("pages/radio.rs")));
    }
    Route::ToastDemo {} => {
      code_map.insert("toast".to_string(), String::from(include_str!("pages/toast.rs")));
    }
    Route::UIDemo {} => {
      code_map.insert("ui".to_string(), String::from(include_str!("pages/ui.rs")));
    }
    Route::CalendarDemo {} => {
      code_map.insert("calendar".to_string(), String::from(include_str!("pages/calendar.rs")));
    }
  }

  code_map
}
