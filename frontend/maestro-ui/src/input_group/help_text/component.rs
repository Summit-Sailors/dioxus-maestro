use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(Clone, PartialEq, Props)]
pub struct HelpTextProps {
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub visible: ReadOnlySignal<bool>,
	#[props(default = String::new())]
	pub class: String,

	#[props(extends = span, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn HelpText(props: HelpTextProps) -> Element {
	let HelpTextProps { class, visible, attributes, children } = props;

	rsx! {
		maestro_headless::help_text::HelpText {
			class: "text-xs text-muted",
			visible,
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}
