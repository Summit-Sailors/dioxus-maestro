use {
	crate::router::Route,
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::io_icons::IoLogoGithub},
	dioxus_logger::tracing::info,
	maestro_headless::button::Button,
	maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame},
	strum::IntoEnumIterator,
	tailwind_fuse::tw_merge,
};

#[component]
pub fn Layout(children: Element) -> Element {
	let toast = use_init_toast_ctx();
	let mut menu_open = use_signal(|| false);
	let current_route = use_route::<Route>();
	let is_home = Route::Home {} == current_route;
	let is_headless = current_route.to_string().starts_with("/headless");
	let is_styled = current_route.to_string().starts_with("/styled");

	let current_routes = if is_headless { Route::get_headless_routes() } else { Route::get_styled_routes() };

	rsx! {
		ToastFrame { manager: toast }
		header { class: "py-4 px-10 sticky top-0 left-0 w-full z-30 border-b  backdrop-blur-sm border-b-neutral-800 bg-neutral-900",
			div { class: "flex justify-end items-center w-full gap-4",
				a {
					href: "https://github.com/Summit-Sailors/dioxus-maestro/tree/maestro-demo/demo",
					target: "_blank",
					class: "flex items-center gap-2 font-medium transition ring-0 ring-offset-0 focus-visible:outline-none text-neutral-300 hover:text-orange-600",
					Icon { icon: IoLogoGithub, class: "w-5 h-5" }
					span { class: "hidden lg:block", "View On GitHub" }
				}
				Button {
					class: "lg:hidden w-8 h-8 p-0 rounded-sm flex flex-col items-center justify-center *:w-5 *:h-px *:transition-all *:ease-linear space-y-1 focus-visible:outline-none *:bg-neutral-300 hover:*:bg-orange-600 focus-visible:*:bg-orange-600 focus-visible:ring-2 rong-offset-2 focus-visible:ring-orange-500 focus-visible:ring-offset-neutral-900",
					onclick: move |_| menu_open.toggle(),
					span { class: if menu_open() { "rotate-45 m-0 translate-y-[2px]" } else { "rotate-0" } }
					span { class: if menu_open() { "opacity-0" } else { "opacity-100" } }
					span { class: if menu_open() { "-rotate-45 m-0 -translate-y-[3px]" } else { "rotate-0" } }
				}
			}
		}
		div { class: "grid lg:grid-cols-[260px_1fr] grid-cols-1 grow overflow-y-hidden overflow-x-hidden relative",
			aside {
				class: tw_merge!(
						"bg-neutral-900 flex-1 flex flex-col w-full h-full lg:sticky absolute overflow-y-auto transition-all ease-linear duration-300 top-0 lg:left-0 z-10 lg:border-r border-r-neutral-800",
						if menu_open() { "bottom-0 right-0" } else { "-right-[100%]" }
				),
				nav { class: "flex-1 flex flex-col h-full *:not-last:border-b *:border-b-neutral-700 overflow-y-auto *:focus-visible:bg-neutral-800 *:focus-visible:text-neutral-100 *:hover:bg-neutral-800 *:hover:text-neutral-100 *:transition-colors *:ease-linear *:px-6 *:py-3 *:ring-0 *:ring-offset-0 *:focus-visible:outline-none",
					{
							current_routes
									.iter()
									.map(|route| {
											let is_current = route == &current_route;
											rsx! {
												Link { to: route.clone(), class: if is_current { "text-orange-600" } else { "" }, "{route.name()}" }
											}
									})
					}
				}
			}
			main { class: "flex-1 flex flex-col overflow-auto", Outlet::<Route> {} }
		}
	}
}
