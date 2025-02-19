use {
	dioxus::prelude::*,
	dioxus_free_icons::{icons::fa_solid_icons::FaSpinner, Icon},
};

#[component]
pub fn FreeIconSpinner(size: u32) -> Element {
	rsx! {
		Icon { icon: FaSpinner, width: size, height: size, class: "animate-spin" }
	}
}
