use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(Props, PartialEq, Clone)]
pub struct ProgressProps {
	#[props(default = ReadOnlySignal::new(Signal::new(0.0)))]
	value: ReadOnlySignal<f32>,
	#[props(optional, default = 100.0)]
	max: f32,
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
	let ProgressProps { value, max, attributes, class, children } = props;

	rsx! {
		maestro_headless::progress::ProgressRoot {
			class: tw_merge!(
					"bg-primary/40 relative h-2 w-full overflow-hidden rounded-full", class.clone()
			),
			max,
			value,
			extra_attributes: attributes.clone(),
			{children.clone()}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct ProgressIndicatorProps {
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = String::new())]
	pub class: String,
	#[props(optional, default = None)]
	children: Element,
}

#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
	rsx! {
		maestro_headless::progress::ProgressIndicator {
			class: tw_merge!(
					"size-full bg-primary rounded-md flex-1 transition-all", props.class.clone()
			),
			extra_attributes: props.attributes.clone(),
			{props.children.clone()}
		}
	}
}
