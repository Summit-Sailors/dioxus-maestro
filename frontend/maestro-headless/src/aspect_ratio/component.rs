use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AspectRatioProps {
	#[props(default = 1.0)]
	pub ratio: f32,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
	rsx! {
		div {
			position: "relative",
			width: "100%",
			padding_bottom: "{100.0 / props.ratio}%",
			..props.attributes,
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
