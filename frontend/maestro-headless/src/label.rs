use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct LabelProps {
	pub text: String,
	pub children: Element,
	#[props(extends = label, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
	rsx! {
		label {..props.attributes,
			span { "{props.text}" }
			{props.children}
		}
	}
}
