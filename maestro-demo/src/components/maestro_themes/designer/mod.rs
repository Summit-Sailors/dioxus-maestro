// For visual theme customization
pub mod state;
pub mod theme_components;

use dioxus::prelude::*;
use maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame};

use crate::{components::ui::navbar::NavBar, router::Route};

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
			NavBar {}
			// main Content
			main {
				id: "maestro-themes-main",
				class: "flex-1 flex flex-col overflow-hidden",
				{content}
			}
		}
	}
}
