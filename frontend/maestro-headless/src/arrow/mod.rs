use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ArrowProps {
	#[props(default = 10.0)]
	pub width: f32,
	#[props(default = 5.0)]
	pub height: f32,

	#[props(extends = svg, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn Arrow(props: ArrowProps) -> Element {
	rsx! {
		svg {
			view_box: "0 0 30 10",
			width: props.width,
			height: props.height,
			xmlns: "http://www.w3.org/2000/svg",
			..props.attributes.clone(),
			polygon { points: "0,0 30,0 15,10" }
		}
	}
}
