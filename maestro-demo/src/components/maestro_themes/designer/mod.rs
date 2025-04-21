// For visual theme customization
pub mod state;
pub mod theme_components;

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::io_icons::IoLogoGithub};
use maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame};

use crate::{components::logo_light::LogoLight, router::Route};

#[component]
pub fn ThemeDesignerLayout(children: Element) -> Element {
	let toast = use_init_toast_ctx();

	let content = rsx! {
		Outlet::<Route> {}
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
		div { id: "maestro-theming", class: "flex flex-col h-screen",
			header {
				id: "maestro-demo-header",
				class: "py-4 sticky top-0 left-0 w-full bg-slate-900 z-30 shadow-[0_0_30px_4px] shadow-slate-500/20 border-b border-b-slate-700",
				div { class: "container flex justify-between items-center w-full text-slate-100 gap-4",
					LogoLight { class: "w-32 h-auto" }
					h1 { class: "lg:text-xl text-lg font-semibold hidden sm:block",
						"Dioxus Maestro Pre-theming"
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
				id: "maestro-themes-main",
				class: "flex-1 flex flex-col overflow-hidden",
				{content}
			}
		}
	}
}
