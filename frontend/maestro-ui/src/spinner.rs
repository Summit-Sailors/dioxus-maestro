use {
	dioxus::prelude::*,
	dioxus_free_icons::{icons::fa_solid_icons::FaSpinner, Icon},
	tailwind_fuse::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct FreeIconSpinnerProps {
	#[props(default = 32)]
	pub size: u32,
	#[props(default = String::new())]
	pub class: String,
}

#[component]
pub fn FreeIconSpinner(props: FreeIconSpinnerProps) -> Element {
	rsx! {
		Icon {
			icon: FaSpinner,
			width: props.size,
			height: props.size,
			class: tw_merge!("animate-spin", & props.class),
		}
	}
}
