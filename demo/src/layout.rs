use {
	crate::{
		components::{editor::editor::CodeEditor, logo_light::LogoLight},
		router::Route,
	},
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::{
			fa_solid_icons::{FaBars, FaX},
			fi_icons::FiGithub,
		},
		Icon,
	},
	dioxus_logger::tracing::info,
	maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame},
	std::collections::HashMap,
	strum::IntoEnumIterator,
	tailwind_fuse::tw_join,
};

#[component]
pub fn Layout(children: Element) -> Element {
	let toast = use_init_toast_ctx();
	let mut menu_open = use_signal(|| false);
	let current_route = use_route::<Route>();

	let navigation_menu = rsx! {
    // sidebar
    nav {
      // class: "min-w-40",
      class: tw_join!(
          "py-2 z-50 shadow-lg rounded-lg bg-gray-800 min-w-40",
          "transform transition-transform duration-300 ease-in-out", if current_route
          .name() == "Home" { "hidden lg:hidden translate-x-0" } else if ! menu_open() {
          "hidden xl:block" } else { "absolute top-0 right-0 left-0 block" }
      ),
      NavigationMenu { close_menu: menu_open }
    }
  };

	let content = rsx! {
    match current_route {
        Route::HomePage {} => rsx! {
          Outlet::<Route> {}
        },
        _ => rsx! {
          div {
            class: tw_join!(
                "my-8 mr-4 flex gap-[1px] flex-1 h-full max-h-[inherit] relative lg:overflow-hidden",
            ),
            CodeEditor {
              title: current_route.name(),
              code_map: get_source_code(&current_route),
              demo: rsx! {
                Outlet::<Route> {}
              },
            }
          
            {navigation_menu}
          }
        },
    }
  };

	rsx! {
    head {
      document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
    }

    ToastFrame { manager: toast }

    div { class: "flex flex-col relative h-full max-h-screen",
      header {
        class: tw_join!(
            "flex justify-between items-center w-full text-white gap-4",
            "px-8 py-4 sticky top-0 left-0 w-full bg-gray-900 z-50 shadow-md hover:shadow-lg",
            "sm:text-lg text-base"
        ),

        LogoLight { class: "w-24 sm:w-28 h-auto" }

        h1 { class: "sm:text-lg text-base font-semibold", "Dioxus Maestro" }

        a {
          href: "https://github.com/Summit-Sailors/dioxus-maestro/tree/maestro-demo/demo",
          target: "_blank",
          class: "flex items-center space-x-2 text-gray-300 hover:text-white transition",

          Icon { icon: FiGithub, width: 16, height: 16 }
          span { "View On GitHub" }
        }
      }

      div { class: "xl:hidden flex justify-center w-full mt-8",

        button {
          class: tw_join!(
              "left-0 right-0 max-w-md flex text-white bg-gray-800 transition-colors focus:outline-none focus:ring-2 focus:ring-inset focus:ring-gray-600",
              if current_route.name() == "Home" { "hidden" } else { "block" }
          ),
          onclick: move |_| menu_open.toggle(),

          div { class: "p-4",
            {
                if menu_open() {
                    rsx! {
                      Icon { icon: FaX, width: 20, height: 20 }
                    }
                } else {
                    rsx! {
                      Icon { icon: FaBars, width: 20, height: 20 }
                    }
                }
            }
          }
        }
      }

      // main Content
      main {
        class: tw_join!(
            "flex-1 flex flex-col transition-all duration-300 max-h-[inherit]",
            "w-full max-w-full md:container md:mx-auto flex-1 max-h-[inherit] px-4 lg:overflow-hidden"
        ),
        {content}
      }
    }
  }
}

#[component]
fn NavigationMenu(close_menu: Signal<bool>) -> Element {
	let current_route = use_route::<Route>();

	rsx! {
    div { class: "h-full w-full",

      div { class: "px-4 w-full",
        {
            Route::iter()
                .map(|route| {
                    let is_current = route == current_route;
                    rsx! {
                      Link {
                        to: route.clone(),
                        class: tw_join!(
                            "block px-4 py-2 rounded-md transition-colors w-full text-center",
                            "hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-600", if
                            is_current { "bg-gray-700 text-white" } else { "text-gray-300 hover:text-white" }
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

// include code and it's deps/utilities
fn get_source_code(route: &Route) -> HashMap<String, String> {
	let mut code_map = HashMap::new();

	match route {
		Route::HomePage {} => {
			code_map.insert("home".to_string(), "".to_string());
		},
		Route::FormsDemo {} => {
			code_map.insert("form demo".to_string(), String::from(include_str!("pages/form.rs")));
			code_map.insert("form component".to_string(), String::from(include_str!("components/form/form_content.rs")));
			code_map.insert("form state debugger".to_string(), String::from(include_str!("components/form/form_state_debugger.rs")));
			code_map.insert("form field wrapper".to_string(), String::from(include_str!("components/form/form_field_wrapper.rs")));
		},
		Route::HooksDemo {} => {
			code_map.insert("hooks".to_string(), String::from(include_str!("pages/hooks.rs")));
		},
		Route::PlottersDemo {} => {
			code_map.insert("plotters".to_string(), String::from(include_str!("pages/plotters.rs")));
		},
		Route::CompleteQueryDemo {} => {
			code_map.insert("query".to_string(), String::from(include_str!("pages/query.rs")));
			code_map.insert("basic query demo".to_string(), String::from(include_str!("components/query/basic_query.rs")));
			code_map.insert("batch query demo".to_string(), String::from(include_str!("components/query/batch.rs")));
			code_map.insert("cache query demo".to_string(), String::from(include_str!("components/query/cache.rs")));
			code_map.insert("mutation query demo".to_string(), String::from(include_str!("components/query/mutation.rs")));
			code_map.insert("parallel query demo".to_string(), String::from(include_str!("components/query/parallel_query.rs")));
		},
		Route::RadioDemo {} => {
			code_map.insert("radio".to_string(), String::from(include_str!("pages/radio.rs")));
		},
		Route::ToastDemo {} => {
			code_map.insert("toast".to_string(), String::from(include_str!("pages/toast.rs")));
		},
		Route::UIDemo {} => {
			code_map.insert("ui".to_string(), String::from(include_str!("pages/ui.rs")));
		},
		Route::CalendarDemo {} => {
			code_map.insert("calendar".to_string(), String::from(include_str!("pages/calendar.rs")));
		},
	}

	code_map
}
