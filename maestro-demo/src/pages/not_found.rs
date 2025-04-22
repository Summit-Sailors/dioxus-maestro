use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
	rsx! {
		div {
			p { class: "text-center font-bold text-[color:var(--text-color)] mt-4",
				"Oops!, Page not found"
			}
		}
	}
}
