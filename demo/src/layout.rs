use {
	crate::{
		components::{backdrop::Backdrop, editor::editor::CodeEditor, logo_light::LogoLight},
		router::Route,
	},
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::{
			bs_icons::{BsLayoutSidebar, BsLayoutSidebarReverse},
			io_icons::IoLogoGithub,
		},
	},
	maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame},
	maestro_ui::button::{Button, ButtonSize, ButtonVariant},
	std::collections::HashMap,
	strum::IntoEnumIterator,
	tailwind_fuse::{tw_join, tw_merge},
};

#[component]
pub fn Layout(children: Element) -> Element {
	let toast = use_init_toast_ctx();
	let mut menu_open = use_signal(|| false);
	let current_route = use_route::<Route>();

	let navigation_menu = rsx! {
		// sidebar
		nav {
			class: tw_join!(
					"relative py-6 sm:px-5 px-0 z-50 bg-slate-900 border-l border-l-slate-700",
					"transform transition-transform duration-300 ease-in-out", (current_route.name()
					== "Home").then_some("hidden lg:hidden translate-x-0")
			),
			Button {
				variant: ButtonVariant::Icon,
				size: ButtonSize::IconMd,
				r#type: "button",
				class: "text-slate-300 hover:text-slate-100 xl:hidden transition-colors mx-auto",
				onclick: move |_| menu_open.set(true),
				Icon { icon: BsLayoutSidebar, class: "w-5 h-5" }
			}
			NavigationMenu { close_menu: menu_open }
		}
	};

	let content = rsx! {
		match current_route {
				Route::HomePage {} => rsx! {
					Outlet::<Route> {}
				},
				_ => rsx! {
					div { class: "flex-1 grid xl:grid-cols-[1fr_358px] sm:grid-cols-[1fr_80px] grid-cols-[1fr_42px] overflow-y-auto relative overflow-x-hidden",
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
			document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
			document::Link {
				rel: "stylesheet",
				href: "https://fonts.googleapis.com/css2?family=Poppins:ital,wght@0,400;0,500;0,600;0,700;1,400;1,500;1,600;1,700&display=swap",
			}
		}

		ToastFrame { manager: toast }

		div { id: "maestro-demo", class: "flex flex-col h-screen",
			Backdrop { show: menu_open }
			header {
				id: "maestro-demo-header",
				class: "py-4 sticky top-0 left-0 w-full bg-slate-900 z-30 shadow-[0_0_30px_4px] shadow-slate-500/20 border-b border-b-slate-700",
				div { class: "container flex justify-between items-center w-full text-slate-100 gap-4",
					LogoLight { class: "w-32 h-auto" }

					h1 { class: "lg:text-xl text-lg font-semibold hidden sm:block",
						"Dioxus Maestro"
					}
					a {
						href: "https://github.com/Summit-Sailors/dioxus-maestro/tree/maestro-demo/demo",
						target: "_blank",
						class: "flex items-center space-x-2 text-xl text-slate-300 hover:text-slate-100 transition ring-0 ring-offset-0 focus-visible:outline-none",
						Icon {
							icon: IoLogoGithub,
							width: 16,
							height: 16,
							class: "w-8 h-8 text-slate-100",
						}
						span { class: "hidden lg:block", "View On GitHub" }
					}
				}
			}

			// main Content
			main {
				id: "maestro-demo-main",
				class: "flex-1 flex flex-col overflow-hidden",
				{content}
			}
		}
	}
}

#[component]
fn NavigationMenu(close_menu: Signal<bool>) -> Element {
	let current_route = use_route::<Route>();

	rsx! {
		div {
			id: "maestro-demo-nav",
			class: tw_merge!(
					"h-full xl:w-full w-80 bg-slate-900 xl:sticky overflow-y-auto absolute transition-all ease-linear duration-300 top-0 -right-80 z-50 px-5 xl:px-0 py-6 xl:py-0 flex gap-4 xl:border-l-0 border-l border-l-slate-600 xl:border-t-0 border-t border-t-slate-600",
					(close_menu()).then_some("right-0")
			),
			Button {
				variant: ButtonVariant::Icon,
				size: ButtonSize::IconMd,
				r#type: "button",
				class: "text-slate-300 hover:text-slate-100 xl:hidden transition-colors",
				onclick: move |_| close_menu.set(false),
				Icon { icon: BsLayoutSidebarReverse, class: "w-5 h-5" }
			}

			div { class: "w-full flex-1",
				{
						Route::iter()
								.map(|route| {
										let is_current = route == current_route;
										rsx! {
											Link {
												to: route.clone(),
												class: tw_join!(
														"block px-4 py-5 transition-colors w-full text-center text-slate-200 font-medium text-xl border-b border-b-slate-600",
														"hover:bg-slate-800/20 hover:text-slate-100 focus:outline-none focus:ring-2 focus:ring-slate-600",
														is_current.then_some("bg-slate-800 text-slate-100")
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
		Route::DieselDemo {} => {
			code_map.insert("diesel".to_string(), String::from(include_str!("clients/db/diesel_demo.rs")));
		},
	}

	code_map
}
