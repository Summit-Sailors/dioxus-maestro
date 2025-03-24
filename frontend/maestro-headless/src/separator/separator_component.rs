use {crate::utils::EOrientation, dioxus::prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct SeparatorProps {
	#[props(default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	orientation: ReadOnlySignal<EOrientation>,
	#[props(default = false)]
	decorative: bool,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
	rsx! {
		div {
			aria_orientation: props.orientation.read().to_string(),
			"data-orientation": props.orientation.read().to_string(),
			role: "separator",
			..props.attributes,
		}
	}
}
