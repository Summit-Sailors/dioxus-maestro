use {crate::presence::Presence, dioxus::prelude::*, std::rc::Rc};

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
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	rsx! {
		Presence { present: visible, node_ref: current_ref,
			span {
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				..attributes,
				{text}
				{children}
			}
		}
	}
}
