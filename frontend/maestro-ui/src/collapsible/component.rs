use {
	crate::shared::{EClass, ERound, ESize, EVariant},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
	let CollapsibleProps { open, default_open, on_open_change, disabled, children, attributes, class } = props;

	rsx! {
		maestro_headless::collapsible::CollapsibleRoot {
			open,
			default_open,
			on_open_change,
			disabled,
			extra_attributes: attributes,
			class: tw_merge!("flex flex-col space-y-3 w-full", class),
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleTriggerProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
	let class = EClass { variant: EVariant::Ghost, size: ESize::Md, round: ERound::Md }.with_class(props.class.clone());
	rsx! {
		maestro_headless::collapsible::CollapsibleTrigger { class: class.clone(), extra_attributes: props.attributes.clone(), {props.children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleContentProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
	rsx! {
		maestro_headless::collapsible::CollapsibleContent {
			extra_attributes: props.attributes,
			class: tw_merge!(
					"data-[state=closed]:animate-slide-out data-[state=open]:animate-slide-in", props
					.class.clone()
			),
			{props.children}
		}
	}
}
