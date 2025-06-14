use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AspectRatioRootProps {
	#[props(default = 1.0)]
	pub ratio: f32,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AspectRatioRoot(props: AspectRatioRootProps) -> Element {
	rsx! {
		div {
			position: "relative",
			width: "100%",
			padding_bottom: "{100.0 / props.ratio}%",
			..props.attributes,
			..props.extra_attributes,
			div {
				position: "absolute",
				top: 0,
				left: 0,
				bottom: 0,
				right: 0,
				{props.children}
			}
		}
	}
}
