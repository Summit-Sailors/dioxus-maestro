use {crate::shared::EOrientation, dioxus::prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct SeparatorProps {
	#[props(default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	orientation: ReadOnlySignal<EOrientation>,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
	let SeparatorProps { orientation, attributes, extra_attributes } = props;

	rsx! {
		div {
			aria_orientation: orientation().to_string(),
			"data-orientation": orientation().to_string(),
			role: "separator",
			..attributes,
			..extra_attributes,
		}
	}
}
