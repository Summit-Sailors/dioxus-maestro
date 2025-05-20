use {
	crate::{presence::Presence, shared::use_ref_provider},
	dioxus::prelude::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct HelpTextProps {
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub visible: ReadOnlySignal<bool>,

	#[props(extends = span, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn HelpText(props: HelpTextProps) -> Element {
	let HelpTextProps { visible, attributes, extra_attributes, children } = props;
	let mut current_ref = use_ref_provider();

	rsx! {
		Presence { present: visible,
			span {
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				..attributes,
				..extra_attributes,
				{children}
			}
		}
	}
}
