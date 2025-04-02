use {crate::shared::EOrientation, dioxus::prelude::*};

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
	#[props(default = ReadOnlySignal::new(Signal::new(EOrientation::Vertical)))]
	pub orientation: ReadOnlySignal<EOrientation>,
	#[props(default = String::new())]
	pub label: String,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = label, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
	let LabelProps { label, orientation, disabled, attributes, children } = props;

	rsx! {
		label {
			aria_disabled: disabled(),
			aria_orientation: orientation.read().to_string(),
			"data-disabled": disabled(),
			"data-orientation": orientation.read().to_string(),
			..attributes,
			if !label.is_empty() {
				span { {label} }
			}
			{children}
		}
	}
}
