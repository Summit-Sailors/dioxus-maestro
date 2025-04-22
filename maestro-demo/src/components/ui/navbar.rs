use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::io_icons::IoLogoGithub};

use crate::components::logo_light::LogoLight;

#[component]
pub fn NavBar() -> Element {
	let mut is_appropriate_env = use_signal(|| false);

	#[cfg(target_arch = "wasm32")]
	const IS_WASM: bool = true;
	#[cfg(not(target_arch = "wasm32"))]
	const IS_WASM: bool = false;

	use_effect(move || {
		is_appropriate_env.set(IS_WASM);
	});

	rsx! {
		header {
			id: "maestro-demo-header",
			class: "navbar sticky top-0 left-0 w-full z-30 shadow-md border-b",
			style: "--tw-shadow-color: var(--highlight-color); background-color: var(--bg-color); border-color: var(--border-color);",

			div { class: "flex justify-between items-center w-full gap-4 text-base text-[color:var(--text-color)] px-4 py-4",

				LogoLight { class: "w-32 h-auto" }

				h1 { class: "lg:text-xl text-lg font-semibold hidden sm:block", "Dioxus Maestro" }

				if is_appropriate_env() {
					crate::components::maestro_themes::theme::prelude::ThemeSelect {}
				}

				a {
					href: "https://github.com/Summit-Sailors/dioxus-maestro/tree/maestro-demo/demo",
					target: "_blank",
					class: "flex items-center space-x-2 text-xl transition ring-0 ring-offset-0 focus-visible:outline-none hover:opacity-80",
					style: "color: var(--text-color);",

					Icon {
						icon: IoLogoGithub,
						width: 16,
						height: 16,
						class: "w-8 h-8",
						style: "color: var(--text-color);",
					}

					span { class: "hidden lg:block", "View On GitHub" }
				}
			}
		}
	}
}
