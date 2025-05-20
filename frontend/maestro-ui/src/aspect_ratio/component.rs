use {dioxus::prelude::*, tailwind_fuse::tw_merge};

#[derive(Clone, PartialEq, Props)]
pub struct AspectRatioProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = 1.0)]
	pub ratio: f32,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
	let AspectRatioProps { ratio, class, attributes, children } = props;

	rsx! {
		maestro_headless::aspect_ratio::AspectRatioRoot {
			class: tw_merge!(
					"overflow-hidden w-full h-full rounded-md bg-muted", class.clone()
			),
			ratio,
			extra_attributes: attributes,
			{children}
		}
	}
}
