use {
	crate::{presence::Presence, shared::use_ref_provider},
	dioxus::prelude::*,
};

#[derive(Props, Clone, PartialEq)]
pub struct HelpTextProps {
	#[props(default = String::new())]
	pub text: String,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub visible: ReadOnlySignal<bool>,

	#[props(extends = span, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn HelpText(props: HelpTextProps) -> Element {
	let HelpTextProps { text, visible, attributes, children } = props;
	let mut current_ref = use_ref_provider();

	rsx! {
		Presence { present: visible,
			span {
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				..attributes,
				{text}
				{children}
			}
		}
	}
}
