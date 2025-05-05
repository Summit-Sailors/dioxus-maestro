use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_free_icons::{
	Icon,
	icons::bs_icons::{BsLayoutSidebar, BsLayoutSidebarReverse},
};
use maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame};
use maestro_ui::button::{Button, ButtonSize, ButtonVariant};
use tailwind_fuse::{tw_join, tw_merge};

use crate::{
	components::{backdrop::Backdrop, editor::code_viewer::CodeEditor, ui::navbar::NavBar},
	router::Route,
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
					"relative py-6 sm:px-5 px-0 z-50 bg-[var(--bg-color)] border-l border-l-[var(--border-color)]",
					"transform transition-transform duration-300 ease-in-out", (current_route.name()
					== "Home").then_some("hidden lg:hidden translate-x-0")
			),
			Button {
				variant: ButtonVariant::Icon,
				size: ButtonSize::IconMd,
				r#type: "button",
				class: "text-[var(--muted-text)] hover:text-[var(--text-color)] xl:hidden transition-colors mx-auto",
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
					div { class: "flex-1 grid xl:grid-cols-[1fr_358px] sm:grid-cols-[1fr_80px] grid-cols-[1fr_42px] overflow-y-auto relative overflow-x-hidden bg-[var(--bg-color)] text-[var(--text-color)]",
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
		div {
			id: "maestro-demo",
			class: "flex flex-col h-screen bg-[var(--bg-color)] text-[var(--text-color)]",
			Backdrop { show: menu_open }

			NavBar {}
			// main Content
			main {
				id: "maestro-demo-main",
				class: "flex-1 flex flex-col overflow-hidden bg-[var(--card-bg)] text-[var(--card-text)]",
				{content}
			}
		}
	}
}

#[component]
fn NavigationMenu(close_menu: Signal<bool>) -> Element {
	rsx! {
		div {
			id: "maestro-demo-nav",
			class: tw_merge!(
					"h-full xl:w-full w-80 bg-[var(--bg-color)] xl:sticky overflow-y-auto absolute transition-all ease-linear duration-300 top-0 -right-80 z-50 px-5 xl:px-0 py-6 xl:py-0 flex gap-4",
					"xl:border-l-0 border-l border-l-[var(--border-color)] xl:border-t-0 border-t border-t-[var(--border-color)]",
					(close_menu()).then_some("right-0")
			),
			Button {
				variant: ButtonVariant::Icon,
				size: ButtonSize::IconMd,
				r#type: "button",
				class: "text-[var(--muted-text)] hover:text-[var(--text-color)] xl:hidden transition-colors",
				onclick: move |_| close_menu.set(false),
				Icon { icon: BsLayoutSidebarReverse, class: "w-5 h-5" }
			}

			div { class: "w-full flex-1 space-y-2 overflow-auto max-h-screen",
				{
						render_section(
										"General",
										&[
												(Route::HomePage {}, "Home"),
												(Route::UIDemo {}, "UI"),
												(Route::FormsDemo {}, "Form"),
												(Route::HooksDemo {}, "Hooks"),
										],
										close_menu,
								)
								.unwrap()
				}

				{
						render_section(
										"Components & Features",
										&[
												(Route::CalendarDemo {}, "Calendar"),
												(Route::PlottersDemo {}, "Plotters"),
												(Route::CompleteQueryDemo {}, "Query"),
												(Route::RadioDemo {}, "Radio"),
												(Route::ToastDemo {}, "Toast"),
										],
										close_menu,
								)
								.unwrap()
				}

				{
						render_section(
										"Database",
										&[(Route::DieselDemo {}, "Diesel"), (Route::SqlxDemo {}, "Sqlx")],
										close_menu,
								)
								.unwrap()
				}
				{
						render_section(
										"Others",
										&[
												(Route::ApalisDemo {}, "Apalis"),
												(Route::SerpApiDemo {}, "SerpaAPI"),
												(Route::AlpacaDemo {}, "Alpaca"),
												(Route::AnthropicDemo {}, "Anthropic"),
										],
										close_menu,
								)
								.unwrap()
				}
			}

		}
	}
}

fn render_section(title: &str, routes: &[(Route, &str)], mut close_menu: Signal<bool>) -> Element {
	rsx! {
		div { class: "border-b border-slate-700 pb-2 mb-2",
			h3 { class: "text-slate-400 text-sm font-bold uppercase tracking-wide",
				"{title}"
			}
		}
		div {
			for (route , name) in routes.iter() {
				Link {
					to: route.clone(),
					class: tw_join!(
							"block px-4 py-2 transition-colors w-full text-left font-small text-xl text-[var(--muted-text)]",
							"hover:bg-[color-mix(in_oklch,var(--bg-color)_80%,black)] hover:text-[var(--text-color)]",
							"focus:outline-none focus:ring-2 focus:ring-[var(--border-color)]", (use_route::<
							Route > () == * route)
							.then_some("bg-[color-mix(in_oklch,var(--bg-color)_60%,black)] text-[var(--text-color)]")
					),
					onclick: move |_| close_menu.set(false),
					"{name}"
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
			code_map.insert("diesel api".to_string(), String::from(include_str!("clients/db/apis/diesel_api.rs")));
			code_map.insert("schema".to_string(), String::from(include_str!("clients/db/diesel_schema.rs")));
		},
		Route::SqlxDemo {} => {
			code_map.insert("sqlx".to_string(), String::from(include_str!("clients/db/sqlx_demo.rs")));
			code_map.insert("sqlx api".to_string(), String::from(include_str!("clients/db/apis/sqlx_api.rs")));
			code_map.insert("mod".to_string(), String::from(include_str!("clients/db/mod.rs")));
		},
		Route::SerpApiDemo {} => {
			code_map.insert("serpapi".to_string(), String::from(include_str!("clients/utilities/serpapi_demo.rs")));
			code_map.insert("serpapi api".to_string(), String::from(include_str!("clients/utilities/apis/serpapi_api.rs")));
		},
		Route::ApalisDemo {} => {
			code_map.insert("apalis".to_string(), String::from(include_str!("clients/utilities/apalis_demo.rs")));
			code_map.insert("apalis api".to_string(), String::from(include_str!("clients/utilities/apis/apalis_api.rs")));
			code_map.insert("mod".to_string(), String::from(include_str!("clients/utilities/mod.rs")));
		},
		Route::AlpacaDemo {} => {
			code_map.insert("alpaca".to_string(), String::from(include_str!("clients/utilities/alpaca_demo.rs")));
		},
		Route::AnthropicDemo {} => {
			code_map.insert("anthropic".to_string(), String::from(include_str!("clients/utilities/anthropic_demo.rs")));
		},
		Route::ThemeEditor { components_id: _ } => {
			code_map.insert("anthropic".to_string(), String::from(""));
		},
		Route::NotFound { route: _ } => {
			code_map.insert("not found".to_string(), String::from(""));
		},
	}

	code_map
}
